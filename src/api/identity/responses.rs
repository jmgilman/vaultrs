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

// #[derive(Deserialize, Debug, Serialize)]
// pub struct ReadEntityResponseData {
//     pub id: String,
//     pub name: String,
//     pub disabled: bool,
//     pub policies: Vec<String>,
//     pub last_update_time: String,
//     pub creation_time: String,
//     pub metadata: Option<HashMap<String, String>>,
//     pub aliases: Option<Vec<Alias>>,
//     // TODO other fields
// }

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
/// Response from executing
/// [CreateEntityAliasRequest](crate::api::ididentity::request::CreateEntityAliasRequest)
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

/// Response from executing
/// [ReadEntityAliasByIdRequest](crate::api::identity::requests::ReadEntityAliasByIdRequest)
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
/// [ListEntityAliasesById](crate::api::identity::requests::ListEntityAliasesById)
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

/// Response from executing
/// [ReadGroupByIdRequest](crate::api::identity::requests::ReadGroupByIdRequest)
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadGroupByIdResponse {
    // TODO What's the type of Alias?
    // pub alias: Option<String>,
    pub creation_time: String,
    pub id: String,
    pub last_update_time: String,
    pub member_entity_ids: Vec<String>,
    pub member_group_ids: Option<Vec<String>>,
    pub metadata: Option<HashMap<String, String>>,
    pub modify_index: u64,
    pub name: String,
    pub policies: Vec<String>,
    #[serde(rename = "type")]
    pub group_type: String,
}

/// Response from executing
/// [ListGroupsById](crate::api::identity::requests::ListGroupsById)
#[derive(Deserialize, Debug, Serialize)]
pub struct ListGroupsByIdResponse {
    pub keys: Vec<String>,
}

/// Response from executing
/// [ReadGroupByNameRequest](crate::api::identity::requests::ReadGroupByNameRequest)
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadGroupByNameResponse {
    // TODO What's the type of Alias?
    // pub alias: Option<String>,
    pub creation_time: String,
    pub id: String,
    pub last_update_time: String,
    pub member_entity_ids: Vec<String>,
    pub member_group_ids: Option<Vec<String>>,
    pub metadata: Option<HashMap<String, String>>,
    pub modify_index: u64,
    pub name: String,
    pub policies: Vec<String>,
    #[serde(rename = "type")]
    pub group_type: String,
}

/// Response from executing
/// [ListGroupsByName](crate::api::identity::requests::ListGroupsByName)
#[derive(Deserialize, Debug, Serialize)]
pub struct ListGroupsByNameResponse {
    pub keys: Vec<String>,
}

/// Response from executing
/// [ReadGroupAliasByIdRequest](crate::api::identity::requests::ReadGroupAliasByIdRequest)
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadGroupAliasByIdResponse {
    pub canonical_id: String,
    pub creation_time: String,
    pub id: String,
    pub last_update_time: String,
    pub metadata: Option<HashMap<String, String>>,
    pub mount_accessor: String,
    pub mount_path: String,
    pub mount_type: String,
    pub name: String,
}

/// Response from executing
/// [ListGroupAliasesById](crate::api::identity::requests::ListGroupAliasesById)
#[derive(Deserialize, Debug, Serialize)]
pub struct ListGroupAliasesByIdResponse {
    pub keys: Vec<String>,
}
