use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct CreateEntityAliasResponse {
    pub request_id: String,
    pub lease_id: String,
    pub renewable: bool,
    pub lease_duration: i64,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub canonical_id: String,
    pub id: String,
}
