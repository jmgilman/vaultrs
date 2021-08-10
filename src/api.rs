pub mod pki;
pub mod sys;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EndpointResult<T> {
    pub request_id: String,
    pub lease_id: String,
    pub renewable: bool,
    pub lease_duration: u32,
    pub data: T,
}
