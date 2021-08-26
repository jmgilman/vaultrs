use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Response from executing
/// [ListAccessorRequest][crate::api::token::requests::ListAccessorRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListAccessorResponse {
    pub keys: Vec<String>,
}

/// Response from executing
/// [TokenLookupRequest][crate::api::token::requests::TokenLookupRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct TokenLookupResponse {
    pub accessor: String,
    pub creation_time: u64,
    pub creation_ttl: u64,
    pub display_name: String,
    pub entity_id: String,
    pub expire_time: Option<String>,
    pub explicit_max_ttl: u64,
    pub id: String,
    pub identity_policies: Option<Vec<String>>,
    pub issue_time: String,
    pub meta: Option<HashMap<String, String>>,
    pub num_uses: u64,
    pub orphan: bool,
    pub path: String,
    pub policies: Vec<String>,
    pub renewable: bool,
    pub ttl: u64,
}
