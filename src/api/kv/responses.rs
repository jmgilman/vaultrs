use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Response from executing
/// [GetSecretRequest][crate::api::kv::requests::GetSecretRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct GetSecretResponse {
    pub data: Value,

    /// Auth is always null, official doc does not document this field
    pub auth: Option<String>,
    pub lease_duration: i32,
    pub lease_id: String,
    pub renewable: bool,
    pub request_id: String
}