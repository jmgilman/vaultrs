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

/// Strips the enclosed response from a [EndpointResult] and returns it as a
/// JSON string.
pub fn strip<T: DeserializeOwned + Serialize>(res: String) -> Result<String, RestClientError> {
    let r: EndpointResult<T> =
        serde_json::from_str(res.as_str()).map_err(|e| RestClientError::GenericError {
            source: Box::new(e),
        })?;

    if let Some(w) = r.warnings {
        match w.is_empty() {
            false => log::warn!("Server returned warnings with response: {:#?}", w),
            true => {}
        }
    }

    serde_json::to_string(&r.data).map_err(|e| RestClientError::GenericError {
        source: Box::new(e),
    })
}

/// Executes an [Endpoint], mapping any [rustify::errors::ClientError] returned
/// to a [ClientError] and discarding the enclosed response.
pub fn exec_with_empty<E: Endpoint>(client: &VaultClient, endpoint: E) -> Result<(), ClientError> {
    endpoint
        .execute(&client.http)
        .map_err(parse_err)
        .map(|_| ())
}

/// Executes an [Endpoint], mapping any [rustify::errors::ClientError] returned
/// to a [ClientError], erroring if an empty response is detected, and finally
/// returning the result from the execution.
pub fn exec_with_result<E: Endpoint>(
    client: &VaultClient,
    endpoint: E,
) -> Result<E::Response, ClientError> {
    endpoint
        .execute(&client.http)
        .map_err(parse_err)?
        .ok_or(ClientError::ResponseEmptyError)
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
