use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Debug, Serialize)]
pub struct CreateEntityResponse {
    pub request_id: String,
    pub lease_id: String,
    pub renewable: bool,
    pub lease_duration: i64,
    pub data: Data,
    pub wrap_info: Value,
    pub warnings: Value,
    pub auth: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub aliases: Value,
    pub id: String,
    pub name: String,
}
