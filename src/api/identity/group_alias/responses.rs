use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Response from executing
/// [CreateGroupAliasRequest](crate::api::identity::group_alias::requests::CreateGroupAliasRequest)
#[derive(Deserialize, Debug, Serialize)]
pub struct CreateGroupAliasResponse {
    pub canonical_id: String,
    pub id: String,
}

/// Response from executing
/// [ReadGroupAliasByIdRequest](crate::api::identity::group_alias::requests::ReadGroupAliasByIdRequest)
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
/// [ListGroupAliasesById](crate::api::identity::group_alias::requests::ListGroupAliasesByIdRequest)
#[derive(Deserialize, Debug, Serialize)]
pub struct ListGroupAliasesByIdResponse {
    pub keys: Vec<String>,
}
