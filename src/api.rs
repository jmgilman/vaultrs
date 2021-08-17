pub mod pki;
pub mod sys;

use rustify::endpoint::Endpoint;
use rustify::errors::ClientError as RestClientError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{client::VaultClient, error::ClientError};

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

#[derive(Deserialize, Debug)]
pub struct EndpointError {
    pub errors: Vec<String>,
}

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

pub fn exec_with_empty<E: Endpoint>(client: &VaultClient, endpoint: E) -> Result<(), ClientError> {
    endpoint
        .execute(&client.http)
        .map_err(parse_err)
        .map(|_| ())
}

pub fn exec_with_result<E: Endpoint>(
    client: &VaultClient,
    endpoint: E,
) -> Result<E::Response, ClientError> {
    endpoint
        .execute(&client.http)
        .map_err(parse_err)?
        .ok_or(ClientError::ResponseEmptyError)
}

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
