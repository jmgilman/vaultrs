use serde::{Deserialize, Serialize};

/// Response from executing
/// [ReadRoleRequest][crate::api::consul::requests::ReadRoleRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadRoleResponse {
    pub token_type: Option<String>, // DEPRECATED since consul version 1.4 and removed in 1.11
    pub partition: Option<String>,
    pub node_identities: Option<Vec<String>>,
    pub consul_namespace: Option<String>,
    pub service_identities: Option<Vec<String>>,
    pub consul_roles: Option<Vec<String>>,
    pub policy: Option<String>, // DEPRECATED since consul version 1.4 and removed in 1.11
    pub policies: Option<Vec<String>>, // DEPRECATED since consul version 1.4 and removed in 1.11
    pub consul_policies: Option<Vec<String>>,
    pub local: bool,
    pub max_ttl: u64,
    pub ttl: u64,
}

/// Response from executing
/// [ListRolesRequest][crate::api::consul::requests::ListRolesRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListRolesResponse {
    pub keys: Vec<String>,
}

/// Response from executing
/// [GenerateConsulCredsRequest][crate::api::consul::requests::GenerateConsulCredsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct GenerateConsulCredsResponse {
    pub token: String,
}
