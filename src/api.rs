pub mod kv2;
pub mod pki;
pub mod sys;

use rustify::client::{Request, Response};
use rustify::endpoint::{Endpoint, MiddleWare};
use rustify::errors::ClientError as RestClientError;
use serde::{de::DeserializeOwned, Deserialize};

use crate::sys::wrapping;
use crate::{client::VaultClient, error::ClientError};

use self::sys::responses::WrappingLookupResponse;

/// Represents the wrapper that mosts responses from the Vault API are wrapped
/// in. It contains data about the response like wrapping info, warnings, and
/// details about any contained leases. The actual response content is contained
/// in the `data` field.
///
/// Most endpoints are configured to pass their responses through [strip] in
/// order to strip the result and return the enclosed response. Any warninings
/// are automatically logged accordingly.
#[derive(Deserialize, Debug)]
pub struct EndpointResult<T> {
    pub data: Option<T>,
    pub lease_id: String,
    pub lease_duration: u32,
    pub renewable: bool,
    pub request_id: String,
    pub warnings: Option<Vec<String>>,
    pub wrap_info: Option<WrapInfo>,
}

impl<T: DeserializeOwned> rustify::endpoint::Wrapper for EndpointResult<T> {
    type Value = T;
}

#[derive(Deserialize, Debug)]
pub struct WrapInfo {
    pub token: String,
    pub accessor: String,
    pub ttl: u64,
    pub creation_time: String,
    pub creation_path: String,
}

pub struct WrappedResponse<E: Endpoint> {
    pub info: WrapInfo,
    pub endpoint: E,
}

impl<E: Endpoint> WrappedResponse<E> {
    pub fn lookup(&self, client: &VaultClient) -> Result<WrappingLookupResponse, ClientError> {
        wrapping::lookup(client, self.info.token.as_str()).map_err(|e| match &e {
            ClientError::APIError {
                url: _,
                code: 400,
                errors: _,
            } => ClientError::WrapInvalidError,
            _ => e,
        })
    }

    pub fn unwrap(&self, client: &VaultClient) -> Result<E::Result, ClientError> {
        wrapping::unwrap(client, self.info.token.as_str())
    }
}

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
}
impl MiddleWare for EndpointMiddleware {
    fn request<E: Endpoint>(
        &self,
        _: &E,
        req: &mut Request,
    ) -> Result<(), rustify::errors::ClientError> {
        // Prepend API version to all requests
        let url_c = req.url.clone();
        let mut segs: Vec<&str> = url_c.path_segments().unwrap().collect();
        segs.insert(0, self.version.as_str());
        req.url.path_segments_mut().unwrap().clear().extend(segs);

        // Add Vault token to all requests
        req.headers
            .push(("X-Vault-Token".to_string(), self.token.clone()));

        // Optionally wrap response
        if let Some(wrap) = &self.wrap {
            req.headers
                .push(("X-Vault-Wrap-TTL".to_string(), wrap.clone()));
        }

        Ok(())
    }

    fn response<E: Endpoint>(
        &self,
        _: &E,
        _: &mut Response,
    ) -> Result<(), rustify::errors::ClientError> {
        Ok(())
    }
}

/// Executes an [Endpoint] which is expected to return an empty response.
///
/// Any errors which occur in execution are wrapped in a
/// [ClientError::RestClientError] and propogated.
pub fn exec_with_empty<E>(client: &VaultClient, endpoint: E) -> Result<(), ClientError>
where
    E: Endpoint<Result = ()>,
{
    endpoint
        .exec_mut(&client.http, &client.middle)
        .map_err(parse_err)
        .map(|_| ())
}

/// Executes an [Endpoint] which is expected to return an empty response.
///
/// Any errors which occur in execution are wrapped in a
/// [ClientError::RestClientError] and propogated.
pub fn exec_with_empty_result<E>(client: &VaultClient, endpoint: E) -> Result<(), ClientError>
where
    E: Endpoint,
{
    endpoint
        .exec_wrap_mut(&client.http, &client.middle)
        .map_err(parse_err)?
        .ok_or(ClientError::ResponseEmptyError)
        .map(strip)
        .map(|_| ())
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
///   propogated errors.
pub fn exec_with_result<E>(client: &VaultClient, endpoint: E) -> Result<E::Result, ClientError>
where
    E: Endpoint,
{
    // let r: Result<Option<EndpointResult<E::Result>>, rustify::errors::ClientError> =
    //     endpoint.exec_wrap_mut(&client.http, &client.middle);
    endpoint
        .exec_wrap_mut(&client.http, &client.middle)
        .map_err(parse_err)?
        .ok_or(ClientError::ResponseEmptyError)
        .map(strip)?
        .ok_or(ClientError::ResponseDataEmptyError)
    // endpoint
    //     .exec_mut(&client.http, &client.middle)
    //     .map_err(parse_err)?
    //     .ok_or(ClientError::ResponseEmptyError)
    //     .map(strip)?
    //     .ok_or(ClientError::ResponseDataEmptyError)
}

pub fn wrap<E>(client: &VaultClient, endpoint: E) -> Result<WrappedResponse<E>, ClientError>
where
    E: Endpoint,
{
    let mut m = client.middle.clone();
    m.wrap = Some("10m".to_string());
    let info = endpoint
        .exec_wrap_mut(&client.http, &m)
        .map_err(parse_err)?
        .ok_or(ClientError::ResponseEmptyError)
        .map(strip_wrap)??;
    Ok(WrappedResponse { info, endpoint })
}

fn strip_wrap<T>(result: EndpointResult<T>) -> Result<WrapInfo, ClientError> {
    if let Some(w) = &result.warnings {
        match w.is_empty() {
            false => log::warn!("Server returned warnings with response: {:#?}", w),
            true => {}
        }
    }
    result.wrap_info.ok_or(ClientError::ResponseWrapError {})
}

/// Strips an [EndpointResult] off a response and logs any warnings found within
fn strip<T>(result: EndpointResult<T>) -> Option<T>
where
    T: DeserializeOwned,
{
    if let Some(w) = &result.warnings {
        match w.is_empty() {
            false => log::warn!("Server returned warnings with response: {:#?}", w),
            true => {}
        }
    }
    result.data
}

/// Attempts to parse the enclosed API errors returned from a
/// [rustify::errors::ClientError::ServerResponseError]. If errors can be parsed
/// it returns the result as a [ClientError::APIError], otherwise it returns a
/// [ClientError::RestClientError].
fn parse_err(e: RestClientError) -> ClientError {
    if let RestClientError::ServerResponseError { url, code, content } = &e {
        match content {
            Some(c) => {
                let errs: Result<EndpointError, _> = serde_json::from_str(c.as_str());
                match errs {
                    Ok(err) => ClientError::APIError {
                        url: url.clone(),
                        code: *code,
                        errors: err.errors,
                    },
                    Err(_) => ClientError::from(e),
                }
            }
            None => ClientError::from(e),
        }
    } else {
        ClientError::from(e)
    }
}
