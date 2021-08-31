use serde::{Deserialize, Serialize};

/// Response from executing
/// [ReadUserRequest][crate::api::auth::userpass::requests::ReadUserRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadUserResponse {
    pub max_ttl: u64,
    pub policies: Vec<String>,
    pub ttl: u64,
}

/// Response from executing
/// [ListUsersRequest][crate::api::auth::userpass::requests::ListUsersRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListUsersResponse {
    pub keys: Vec<String>,
}
