use serde::{Deserialize, Serialize};

/// Response from executing
/// [ReadConnectionRequest][crate::api::database::requests::ReadConnectionRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadConnectionResponse {
    pub allowed_roles: Vec<String>,
    pub connection_details: ConnectionDetails,
    pub plugin_name: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ConnectionDetails {
    pub connection_url: String,
    pub username: String,
}

/// Response from executing
/// [ListConnectionsRequest][crate::api::database::requests::ListConnectionsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListConnectionsResponse {
    pub keys: Vec<String>,
}

/// Response from executing
/// [ReadRoleRequest][crate::api::database::requests::ReadRoleRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadRoleResponse {
    pub creation_statements: Vec<String>,
    pub db_name: String,
    pub default_ttl: u64,
    pub max_ttl: u64,
    pub renew_statements: Vec<String>,
    pub revocation_statements: Vec<String>,
    pub rollback_statements: Vec<String>,
}

/// Response from executing
/// [ListRolesRequest][crate::api::database::requests::ListRolesRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListRolesResponse {
    pub keys: Vec<String>,
}

/// Response from executing
#[derive(Deserialize, Debug, Serialize)]
pub struct GenerateCredentialsResponseData {
    pub username: String,
    pub password: String,
}

/// Response from executing
/// [GetSecretRequest][crate::api::database::requests::GenerateCredentialsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct GenerateCredentialsResponse {
    pub data: GenerateCredentialsResponseData,

    /// Auth is always null, official doc does not document this field
    pub auth: Option<String>,
    pub lease_duration: i32,
    pub lease_id: String,
    pub renewable: bool,
    pub request_id: String,
}

/// Response from executing
/// [ReadStaticRoleRequest][crate::api::database::requests::ReadStaticRoleRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadStaticRoleResponse {
    pub db_name: String,
    pub username: String,
    pub rotation_period: u64,
    pub rotation_statements: Vec<String>,
}

/// Response from executing
/// [ListStaticRolesRequest][crate::api::database::requests::ListStaticRolesRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListStaticRolesResponse {
    pub keys: Vec<String>,
}

/// Response from executing
/// [GetStaticCredentialsRequest][crate::api::database::requests::GetStaticCredentialsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct GetStaticCredentialsResponse {
    pub last_vault_rotation: String,
    pub password: String,
    pub rotation_period: u64,
    pub ttl: u64,
    pub username: String,
}
