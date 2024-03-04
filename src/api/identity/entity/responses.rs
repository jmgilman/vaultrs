use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Response from executing
/// [ReadEntityByIdRequest](crate::api::identity::requests::ReadEntityByIdRequest)
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
}

/// Response from executing
/// [ReadEntityByNameRequest](crate::api::identity::requests::ReadEntityByNameRequest)
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
    // TODO other fields
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
    // TODO other fields
}

/// Response from executing
/// [ListEntitiesById](crate::api::identity::requests::ListEntitiesById)
#[derive(Deserialize, Debug, Serialize)]
pub struct ListEntitiesByIdResponse {
    pub keys: Vec<String>,
}

/// Response from executing
/// [CreateEntityByNameRequest](crate::api::identity::requests::CreateEntityByNameRequest)
#[derive(Deserialize, Debug, Serialize)]
pub struct CreateEntityByNameResponse {
    pub aliases: Option<Vec<Alias>>,
    pub id: String,
    pub name: String,
}

/// Response from executing
/// [ListEntitiesById](crate::api::identity::requests::ListEntitiesById)
#[derive(Deserialize, Debug, Serialize)]
pub struct ListEntitiesByNameResponse {
    pub keys: Vec<String>,
}
