use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Response from executing
/// [GetSecretRequest][crate::api::kv1::requests::GetSecretRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct GetSecretResponse {
    pub data: Value,

    /// Auth is always null, official doc does not document this field
    pub auth: Option<String>,
    pub lease_duration: i32,
    pub lease_id: String,
    pub renewable: bool,
    pub request_id: String,
}

/// Response from executing
/// [ListSecretRequest][crate::api::kv1::requests::ListSecretRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListSecretResponse {
    pub data: ListSecretResponseKeys,

    /// Auth is always null, official doc does not document this field
    pub auth: Option<String>,
    pub lease_duration: i32,
    pub lease_id: String,
    pub renewable: bool,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ListSecretResponseKeys {
    pub keys: Vec<String>,
}
