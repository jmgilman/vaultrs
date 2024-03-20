use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Response from executing
/// [CreateEntityAliasRequest](crate::api::identity::entity_alias::requests::CreateEntityAliasRequest)
#[derive(Deserialize, Debug, Serialize)]
pub struct CreateEntityAliasResponse {
    pub canonical_id: String,
    pub id: String,
}

/// Response from executing
/// [ReadEntityAliasByIdRequest](crate::api::identity::entity_alias::requests::ReadEntityAliasByIdRequest)
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadEntityAliasByIdResponse {
    pub creation_time: String,
    pub canonical_id: String,
    pub custom_metadata: Option<HashMap<String, String>>,
    pub id: String,
    pub last_update_time: String,
    pub local: bool,
    pub metadata: Option<HashMap<String, String>>,
    pub mount_accessor: String,
    pub mount_path: String,
    pub mount_type: String,
    pub name: String,
}

/// Response from executing
/// [ListEntityAliasesById](crate::api::identity::entity_alias::requests::ListEntityAliasesByIdRequest)
#[derive(Deserialize, Debug, Serialize)]
pub struct ListEntityAliasesByIdResponse {
    pub key_info: HashMap<String, KeyInfo>,
    pub keys: Vec<String>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct KeyInfo {
    pub canonical_id: String,
    pub custom_metadata: Option<HashMap<String, String>>,
    pub local: bool,
    pub mount_accessor: String,
    pub mount_path: String,
    pub mount_type: String,
    pub name: String,
}
