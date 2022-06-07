use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Debug, Serialize)]
pub struct CreateIdentityAliasResponse {
    pub request_id: String,
    pub lease_id: String,
    pub renewable: bool,
    pub lease_duration: i64,
    pub data: Data,
    pub warnings: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub canonical_id: String,
    pub id: String,
}
