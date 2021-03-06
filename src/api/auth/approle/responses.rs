use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Response from executing
/// [ListRolesRequest][crate::api::auth::approle::requests::ListRolesRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListRolesResponse {
    pub keys: Vec<String>,
}

/// Response from executing
/// [ReadAppRoleRequest][crate::api::auth::approle::requests::ReadAppRoleRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadAppRoleResponse {
    pub bind_secret_id: bool,
    pub secret_id_bound_cidrs: Option<Vec<String>>,
    pub secret_id_num_uses: u64,
    pub secret_id_ttl: u64,
    pub token_ttl: u64,
    pub token_max_ttl: u64,
    pub token_policies: Vec<String>,
    pub token_bound_cidrs: Vec<String>,
    pub token_explicit_max_ttl: u64,
    pub token_no_default_policy: bool,
    pub token_num_uses: u64,
    pub token_period: u64,
    pub token_type: String,
}

/// Response from executing
/// [ReadRoleIDRequest][crate::api::auth::approle::requests::ReadRoleIDRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadRoleIDResponse {
    pub role_id: String,
}

/// Response from executing
/// [GenerateNewSecretIDRequest][crate::api::auth::approle::requests::GenerateNewSecretIDRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct GenerateNewSecretIDResponse {
    pub secret_id_accessor: String,
    pub secret_id: String,
    pub secret_id_ttl: u64,
}

/// Response from executing
/// [ListSecretIDRequest][crate::api::auth::approle::requests::ListSecretIDRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListSecretIDResponse {
    pub keys: Vec<String>,
}

/// Response from executing
/// [ReadSecretIDRequest][crate::api::auth::approle::requests::ReadSecretIDRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadSecretIDResponse {
    pub cidr_list: Vec<String>,
    pub creation_time: String,
    pub expiration_time: String,
    pub last_updated_time: String,
    pub metadata: Option<HashMap<String, String>>,
    pub secret_id_accessor: String,
    pub secret_id_num_uses: u64,
    pub secret_id_ttl: u64,
    pub token_bound_cidrs: Vec<String>,
}

/// Response from executing
/// [CreateCustomSecretIDRequest][crate::api::auth::approle::requests::CreateCustomSecretIDRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct CreateCustomSecretIDResponse {
    pub secret_id_accessor: String,
    pub secret_id: String,
}
