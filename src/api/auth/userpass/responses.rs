use serde::{Deserialize, Serialize};

/// Response from executing
/// [ReadUserRequest][crate::api::auth::userpass::requests::ReadUserRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadUserResponse {
    pub token_bound_cidrs: Vec<String>,
    pub token_explicit_max_ttl: u64,
    pub token_no_default_policy: bool,
    pub token_num_uses: u64,
    pub token_ttl: u64,
    pub token_max_ttl: u64,
    pub token_period: u64,
    pub token_policies: Vec<String>,
    pub token_type: String,
}

/// Response from executing
/// [ListUsersRequest][crate::api::auth::userpass::requests::ListUsersRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListUsersResponse {
    pub keys: Vec<String>,
}
