pub mod auth;
pub mod aws;
pub mod cubbyhole;
pub mod database;
pub mod identity;
pub mod kv1;
pub mod kv2;
pub mod pki;
pub mod rabbitmq;
pub mod ssh;
pub mod sys;
pub mod token;
pub mod transit;

use std::collections::HashMap;
use std::str::FromStr;

use async_trait::async_trait;
use rustify::endpoint::{Endpoint, MiddleWare};
use rustify::errors::ClientError as RestClientError;
use serde::{de::DeserializeOwned, Deserialize};

use crate::sys::wrapping;
use crate::{client::Client, error::ClientError};

use self::sys::responses::WrappingLookupResponse;

/// Represents the wrapper that mosts responses from the Vault API are wrapped
/// in. It contains data about the response like wrapping info, warnings, and
/// details about any contained leases. The actual response content is contained
/// in the `data` field.
///
/// Most endpoints are configured to pass their responses through `strip` in
/// order to strip the result and return the enclosed response. Any warnings
/// are automatically logged accordingly.
#[derive(Deserialize, Debug)]
pub struct EndpointResult<T> {
    pub data: Option<T>,
    pub auth: Option<AuthInfo>,
    pub lease_id: String,
    pub lease_duration: u32,
    pub renewable: bool,
    pub request_id: String,
    pub warnings: Option<Vec<String>>,
    pub wrap_info: Option<WrapInfo>,
}

impl<T: DeserializeOwned + Send + Sync> rustify::endpoint::Wrapper for EndpointResult<T> {
    type Value = T;
}

/// The information stored in the optional `wrap_info` field of API responses
#[derive(Deserialize, Debug)]
pub struct WrapInfo {
    pub token: String,
    pub accessor: String,
    pub ttl: u64,
    pub creation_time: String,
    pub creation_path: String,
}

/// The information stored in the optional `auth` field of API responses
#[derive(Deserialize, Debug)]
pub struct AuthInfo {
    pub client_token: String,
    pub accessor: String,
    pub policies: Vec<String>,
    pub token_policies: Vec<String>,
    pub metadata: Option<HashMap<String, String>>,
    pub lease_duration: u64,
    pub renewable: bool,
    pub entity_id: String,
    pub token_type: String,
    pub orphan: bool,
}

/// Represents an API response that has been wrapped by a unique token.
///
/// See [response wrapping][<https://developer.hashicorp.com/vault/docs/concepts/response-wrapping>] for details on how this works. This struct stores
/// the unique token returned by the server as well as the original endpoint
/// request that generated this token. The struct contains methods for
/// interacting with the wrapped response.
///
pub struct WrappedResponse<E: Endpoint> {
    pub info: WrapInfo,
    pub endpoint: rustify::endpoint::EndpointResult<E::Response>,
}

impl<E: Endpoint> WrappedResponse<E> {
    /// Retrieves information about this wrapped response
    pub async fn lookup(
        &self,
        client: &impl Client,
    ) -> Result<WrappingLookupResponse, ClientError> {
        debug!("Looking up wrapped response information");
        wrapping::lookup(client, self.info.token.as_str())
            .await
            .map_err(|e| match &e {
                ClientError::APIError {
                    code: 400,
                    errors: _,
                } => ClientError::WrapInvalidError,
                _ => e,
            })
    }

    /// Unwraps this response, returning the original response
    pub async fn unwrap(&self, client: &impl Client) -> Result<E::Response, ClientError> {
        wrapping::unwrap(client, Some(self.info.token.as_str())).await
    }
}

/// Provides a method for wrapping [Endpoint]s
#[async_trait]
pub trait ResponseWrapper: Endpoint {
    async fn wrap(self, client: &impl Client) -> Result<WrappedResponse<Self>, ClientError> {
        wrap(client, self).await
    }
}

impl<E: Endpoint> ResponseWrapper for E {}

/// Represents the format that the Vault server uses when returning errors. This
/// structure is usually accompanied with HTTP error response codes like 404
/// or 500 in the content body. It is parsed and returned as a
/// [ClientError::APIError].
#[derive(Deserialize, Debug)]
pub struct EndpointError {
    pub errors: Vec<String>,
}

/// A [MiddleWare] for adding version and token information to all requests.
///
/// Implements [MiddleWare] to provide support for prepending API version
/// information to all requests and adding a Vault token to the header of all
/// requests. This is automatically passed by the API functions when an endpoint
/// is executed.
#[derive(Debug, Clone)]
pub struct EndpointMiddleware {
    pub token: String,
    pub version: String,
    pub wrap: Option<String>,
    pub namespace: Option<String>,
}
impl MiddleWare for EndpointMiddleware {
    fn request<E: Endpoint>(
        &self,
        _: &E,
        req: &mut http::Request<Vec<u8>>,
    ) -> Result<(), rustify::errors::ClientError> {
        // Prepend API version to all requests
        trace!(
            "Middleware: prepending {} version to URL",
            self.version.as_str()
        );
        let url = url::Url::parse(req.uri().to_string().as_str()).unwrap();
        let mut url_c = url.clone();
        let mut segs: Vec<&str> = url.path_segments().unwrap().collect();
        segs.insert(0, self.version.as_str());
        url_c.set_path(format!("{}{}", self.version, url_c.path()).as_str());
        *req.uri_mut() = http::Uri::from_str(url_c.as_str()).unwrap();
        trace!("Middleware: final URL is {}", url_c.as_str());

        // Add X-Vault-Request to all requests
        req.headers_mut().append(
            "X-Vault-Request",
            http::HeaderValue::from_str("true").unwrap(),
        );

        // Add Vault token to all requests
        if !self.token.is_empty() {
            trace!("Middleware: adding token to header");
            req.headers_mut().append(
                "X-Vault-Token",
                http::HeaderValue::from_str(self.token.as_str()).unwrap(),
            );
        }

        // Optionally wrap response
        if let Some(wrap) = &self.wrap {
            trace!("Middleware: adding wrap header with {} ttl", wrap);
            req.headers_mut().append(
                "X-Vault-Wrap-TTL",
                http::HeaderValue::from_str(wrap.as_str()).unwrap(),
            );
        }

        // Optionally wrap response
        if let Some(namespace) = &self.namespace {
            trace!("Middleware: adding namespace header {}", namespace);
            req.headers_mut().append(
                "X-Vault-Namespace",
                http::HeaderValue::from_str(namespace.as_str()).unwrap(),
            );
        }

        Ok(())
    }

    fn response<E: Endpoint>(
        &self,
        _: &E,
        _: &mut http::Response<Vec<u8>>,
    ) -> Result<(), rustify::errors::ClientError> {
        Ok(())
    }
}

/// Executes an [Endpoint] which is expected to return an empty HTTP response.
///
/// Any errors which occur in execution are wrapped in a
/// [ClientError::RestClientError] and propagated.
#[instrument(name = "request", skip_all, fields(method = ?endpoint.method(), path = %endpoint.path()), err)]
pub async fn exec_with_empty<E>(client: &impl Client, endpoint: E) -> Result<(), ClientError>
where
    E: Endpoint,
{
    trace!("start request");
    endpoint
        .with_middleware(client.middle())
        .exec(client.http())
        .await
        .map_err(parse_err)
        .map(|_| ())
}

/// Executes an [Endpoint] which is expected to return an empty API result.
///
/// Any errors which occur in execution are wrapped in a
/// [ClientError::RestClientError] and propagated.
#[instrument(name = "request", skip_all, fields(method = ?endpoint.method(), path = %endpoint.path()), err)]
pub async fn exec_with_empty_result<E>(client: &impl Client, endpoint: E) -> Result<(), ClientError>
where
    E: Endpoint,
{
    trace!("start request");
    endpoint
        .with_middleware(client.middle())
        .exec(client.http())
        .await
        .map_err(ClientError::from)?
        .wrap::<EndpointResult<_>>()
        .map_err(parse_err)
        .map(strip)
        .map(|_| ())
}

/// Executes an [Endpoint] which is expected to return an unwrapped response.
///
/// Any errors which occur in execution are wrapped in a
/// [ClientError::RestClientError] and propagated.
#[instrument(name = "request", skip_all, fields(method = ?endpoint.method(), path = %endpoint.path()), err)]
pub async fn exec_with_no_result<E>(
    client: &impl Client,
    endpoint: E,
) -> Result<E::Response, ClientError>
where
    E: Endpoint,
{
    trace!("start request");
    endpoint
        .with_middleware(client.middle())
        .exec(client.http())
        .await
        .map_err(parse_err)?
        .parse()
        .map_err(ClientError::from)
}

/// Executes an [Endpoint] and returns the result.
///
/// The result from the executed endpoint has a few operations performed on it:
///
/// * Any potential API error responses from the execution are searched for and,
///   if found, converted to a [ClientError::APIError]
/// * All other errors are mapped from [rustify::errors::ClientError] to
///   [ClientError::RestClientError]
/// * An empty content body from the execution is rejected and a
///   [ClientError::ResponseEmptyError] is returned instead
/// * The enclosing [EndpointResult] is stripped off and any warnings found in
///   the result are logged
/// * An empty `data` field in the [EndpointResult] is rejected and a
///   [ClientError::ResponseDataEmptyError] is returned instead
/// * The value from the enclosed `data` field is returned along with any
///   propagated errors.
#[instrument(name = "request", skip_all, fields(method = ?endpoint.method(), path = %endpoint.path()), err)]
pub async fn exec_with_result<E>(
    client: &impl Client,
    endpoint: E,
) -> Result<E::Response, ClientError>
where
    E: Endpoint,
{
    trace!("start request");
    endpoint
        .with_middleware(client.middle())
        .exec(client.http())
        .await
        .map_err(parse_err)?
        .wrap::<EndpointResult<_>>()
        .map_err(ClientError::from)
        .map(strip)?
        .ok_or(ClientError::ResponseDataEmptyError)
}

/// Executes the given endpoint but requests that the Vault server to return a
/// token wrapped response.
///
/// The token is stored in a [WrappedResponse] and the original response can
/// be fetched using the `unwrap` method provided by the struct.
pub async fn wrap<E>(client: &impl Client, endpoint: E) -> Result<WrappedResponse<E>, ClientError>
where
    E: Endpoint,
{
    trace!(
        "Executing {} and returning a wrapped response",
        endpoint.path()
    );
    let mut m = client.middle().clone();
    m.wrap = Some("10m".to_string());
    let resp = endpoint
        .with_middleware(&m)
        .exec(client.http())
        .await
        .map_err(parse_err)?;
    let info = resp
        .wrap::<EndpointResult<_>>()
        .map_err(ClientError::from)
        .map(strip_wrap)??;
    Ok(WrappedResponse {
        info,
        endpoint: resp,
    })
}

pub async fn auth<E>(client: &impl Client, endpoint: E) -> Result<AuthInfo, ClientError>
where
    E: Endpoint<Response = ()>,
{
    trace!(
        "Executing {} and returning authentication info",
        endpoint.path()
    );
    let r: EndpointResult<()> = endpoint
        .with_middleware(client.middle())
        .exec(client.http())
        .await
        .map_err(parse_err)?
        .wrap::<EndpointResult<_>>()
        .map_err(ClientError::from)?;
    r.auth.ok_or(ClientError::ResponseEmptyError)
}

/// Strips the wrapping information out of an [EndpointResult], returning the
/// enclosing information as a [WrapInfo].
fn strip_wrap<T>(result: EndpointResult<T>) -> Result<WrapInfo, ClientError> {
    trace!("Stripping wrap info from API response");
    if let Some(w) = &result.warnings {
        if !w.is_empty() {
            warn!("Server returned warnings with response: {:#?}", w);
        }
    }
    result.wrap_info.ok_or(ClientError::ResponseWrapError {})
}

/// Strips an [EndpointResult] off a response and logs any warnings found within
fn strip<T>(result: EndpointResult<T>) -> Option<T>
where
    T: DeserializeOwned,
{
    trace!("Stripping response wrapper from API response");
    if let Some(w) = &result.warnings {
        if !w.is_empty() {
            warn!("Detected warnings in API response: {:#?}", w);
        }
    }
    result.data
}

/// Attempts to parse the enclosed API errors returned from a
/// [rustify::errors::ClientError::ServerResponseError]. If errors can be parsed
/// it returns the result as a [ClientError::APIError], otherwise it returns a
/// [ClientError::RestClientError].
fn parse_err(e: RestClientError) -> ClientError {
    if let RestClientError::ServerResponseError { code, content } = &e {
        match content {
            Some(c) => {
                let errs: Result<EndpointError, _> = serde_json::from_str(c.as_str());
                match errs {
                    Ok(err) => {
                        if !err.errors.is_empty() {
                            error!("Detected errors in API response: {:#?}", err.errors);
                        }
                        ClientError::APIError {
                            code: *code,
                            errors: err.errors,
                        }
                    }
                    Err(_) => ClientError::from(e),
                }
            }
            None => ClientError::from(e),
        }
    } else {
        ClientError::from(e)
    }
}
