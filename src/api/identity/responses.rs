use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Response from executing
/// [CreateEntityRequest](crate::api::entity::requests::CreateEntityRequest)
#[derive(Deserialize, Debug, Serialize)]
pub struct CreateEntityResponse {
    pub request_id: String,
    pub lease_id: String,
    pub renewable: bool,
    pub lease_duration: i64,
    pub data: CreateEntityResponseData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateEntityResponseData {
    pub aliases: Value,
    pub id: String,
    pub name: String,
}

/// Response from executing
/// [ReadEntityByNameRequest](crate::api::entity::requests::ReadEntityByNameRequest)
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadEntityByNameResponse {
    pub data: ReadEntityResponseData,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ReadEntityResponseData {
    pub id: String,
    pub name: String,
    // TODO other fields
}

/// Response from executing
/// [CreateEntityAliasRequest](crate::api::entity_alias::request::CreateEntityAliasRequest)
#[derive(Deserialize, Debug, Serialize)]
pub struct CreateEntityAliasResponse {
    pub request_id: String,
    pub lease_id: String,
    pub renewable: bool,
    pub lease_duration: u64,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub canonical_id: String,
    pub id: String,
}
