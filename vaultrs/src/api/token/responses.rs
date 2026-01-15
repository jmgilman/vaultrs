use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Response from executing
/// [ListAccessorRequest][crate::api::token::requests::ListAccessorRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListAccessorResponse {
    pub keys: Vec<String>,
}

/// Response from executing
/// [LookupTokenRequest][crate::api::token::requests::LookupTokenRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct LookupTokenResponse {
    pub accessor: String,
    pub creation_time: u64,
    pub creation_ttl: u64,
    pub display_name: String,
    pub entity_id: String,
    pub expire_time: Option<String>,
    pub explicit_max_ttl: u64,
    pub id: String,
    pub identity_policies: Option<Vec<String>>,
    pub issue_time: Option<String>,
    pub meta: Option<HashMap<String, String>>,
    pub num_uses: u64,
    pub orphan: bool,
    pub path: String,
    pub policies: Vec<String>,
    pub renewable: Option<bool>,
    pub role: Option<String>,
    pub ttl: u64,
}

/// Response from executing
/// [ReadTokenRoleRequest][crate::api::token::requests::ReadTokenRoleRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadTokenRoleResponse {
    pub allowed_entity_aliases: Option<Vec<String>>,
    pub allowed_policies: Vec<String>,
    pub disallowed_policies: Vec<String>,
    pub explicit_max_ttl: u64,
    pub name: String,
    pub orphan: bool,
    pub path_suffix: String,
    pub period: u64,
    pub renewable: bool,
    pub token_explicit_max_ttl: u64,
    pub token_period: u64,
    pub token_type: String,
}

/// Response from executing
/// [ListTokenRolesRequest][crate::api::token::requests::ListTokenRolesRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListTokenRolesResponse {
    pub keys: Vec<String>,
}
