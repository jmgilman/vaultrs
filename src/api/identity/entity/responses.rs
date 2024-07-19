use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Response from executing
/// [CreateEntityRequest](crate::api::identity::entity::requests::CreateEntityRequest)
#[derive(Deserialize, Debug, Serialize)]
pub struct CreateEntityResponse {
    pub id: String,
    pub alias: Option<Vec<Alias>>,
}

/// Response from executing
/// [ReadEntityByIdRequest](crate::api::identity::entity::requests::ReadEntityByIdRequest)
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadEntityByIdResponse {
    pub id: String,
    pub name: String,
    pub disabled: bool,
    pub policies: Vec<String>,
    pub last_update_time: String,
    pub creation_time: String,
    pub metadata: Option<HashMap<String, String>>,
    pub aliases: Vec<Alias>,
    pub direct_group_ids: Vec<String>,
    pub group_ids: Vec<String>,
    pub inherited_group_ids: Vec<String>,
    pub merged_entity_ids: Option<Vec<String>>,
    pub namespace_id: String,
}

/// Response from executing
/// [ReadEntityByNameRequest](crate::api::identity::entity::requests::ReadEntityByNameRequest)
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadEntityByNameResponse {
    pub id: String,
    pub name: String,
    pub disabled: bool,
    pub policies: Vec<String>,
    pub last_update_time: String,
    pub creation_time: String,
    pub metadata: Option<HashMap<String, String>>,
    pub aliases: Vec<Alias>,
    pub direct_group_ids: Vec<String>,
    pub group_ids: Vec<String>,
    pub inherited_group_ids: Vec<String>,
    pub merged_entity_ids: Option<Vec<String>>,
    pub namespace_id: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Alias {
    pub id: String,
    pub canonical_id: String,
    pub mount_accessor: String,
    pub name: String,
    pub creation_time: String,
    pub last_update_time: String,
    pub local: bool,
    pub mount_type: String,
    pub mount_path: String,
    pub custom_metadata: Option<HashMap<String, String>>,
    pub metadata: Option<HashMap<String, String>>,
    pub merged_from_canonical_ids: Option<Vec<String>>,
}

/// Response from executing
/// [ListEntitiesById](crate::api::identity::entity::requests::ListEntitiesByIdRequest)
#[derive(Deserialize, Debug, Serialize)]
pub struct ListEntitiesByIdResponse {
    pub keys: Vec<String>,
}

/// Response from executing
/// [CreateEntityByNameRequest](crate::api::identity::entity::requests::CreateEntityByNameRequest)
#[derive(Deserialize, Debug, Serialize)]
pub struct CreateEntityByNameResponse {
    pub aliases: Option<Vec<Alias>>,
    pub id: String,
    pub name: String,
}

/// Response from executing
/// [ListEntitiesById](crate::api::identity::entity::requests::ListEntitiesByNameRequest)
#[derive(Deserialize, Debug, Serialize)]
pub struct ListEntitiesByNameResponse {
    pub keys: Vec<String>,
}
