pub mod pki;
pub mod sys;

use rustify::errors::ClientError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct EndpointResult<T: Serialize> {
    pub request_id: String,
    pub lease_id: String,
    pub renewable: bool,
    pub lease_duration: u32,
    pub data: T,
}

pub fn strip<T: DeserializeOwned + Serialize>(res: String) -> Result<String, ClientError> {
    let r: EndpointResult<T> =
        serde_json::from_str(res.as_str()).map_err(|e| ClientError::GenericError {
            source: Box::new(e),
        })?;
    serde_json::to_string(&r.data).map_err(|e| ClientError::GenericError {
        source: Box::new(e),
    })
}
