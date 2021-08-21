pub mod kv2;
pub mod pki;
pub mod sys;

use rustify::endpoint::Endpoint;
use rustify::errors::ClientError as RestClientError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{client::VaultClient, error::ClientError};

/// Represents the wrapper that mosts responses from the Vault API are wrapped
/// in. It contains data about the response like wrapping info, warnings, and
/// details about any contained leases. The actual response content is contained
/// in the `data` field.
///
/// Most endpoints are configured to pass their responses through [strip] in
/// order to strip the result and return the enclosed response. Any warninings
/// are automatically logged accordingly.
#[derive(Deserialize, Debug)]
pub struct EndpointResult<T: Serialize> {
    pub data: Option<T>,
    pub lease_id: String,
    pub lease_duration: u32,
    pub renewable: bool,
    pub request_id: String,
    pub warnings: Option<Vec<String>>,
    pub wrap_info: Option<String>,
}

/// Represents the format that the Vault server uses when returning errors. This
/// structure is usually accompanied with HTTP error response codes like 404
/// or 500 in the content body. It is parsed and returned as a
/// [ClientError::APIError].
#[derive(Deserialize, Debug)]
pub struct EndpointError {
    pub errors: Vec<String>,
}

/// Executes an [Endpoint] which is expected to return an empty response.
///
/// Any errors which occur in execution are wrapped in a
/// [ClientError::RestClientError] and propogated.
pub fn exec_with_empty<E>(client: &VaultClient, endpoint: E) -> Result<(), ClientError>
where
    E: Endpoint<Result = ()>,
{
    endpoint.exec(&client.http).map_err(parse_err).map(|_| ())
}

/// Executes an [Endpoint] which is expected to return an empty response.
///
/// Any errors which occur in execution are wrapped in a
/// [ClientError::RestClientError] and propogated.
pub fn exec_with_empty_result<E>(client: &VaultClient, endpoint: E) -> Result<(), ClientError>
where
    E: Endpoint<Result = EndpointResult<()>>,
{
    endpoint
        .exec(&client.http)
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
pub fn exec_with_result<E, R>(client: &VaultClient, endpoint: E) -> Result<R, ClientError>
where
    E: Endpoint<Result = EndpointResult<R>>,
    R: DeserializeOwned + Serialize,
{
    endpoint
        .exec(&client.http)
        .map_err(parse_err)?
        .ok_or(ClientError::ResponseEmptyError)
        .map(strip)?
        .ok_or(ClientError::ResponseDataEmptyError)
}

/// Strips an [EndpointResult] off a response and logs any warnings found within
fn strip<T>(result: EndpointResult<T>) -> Option<T>
where
    T: DeserializeOwned + Serialize,
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
