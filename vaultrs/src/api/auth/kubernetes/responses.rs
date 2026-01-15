use serde::{Deserialize, Serialize};

/// Response from executing
/// [ReadKubernetesAuthConfigRequest][crate::api::auth::kubernetes::requests::ReadKubernetesAuthConfigRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadKubernetesAuthConfigResponse {
    pub kubernetes_host: String,
    pub kubernetes_ca_cert: Option<String>,
    pub pem_keys: Option<Vec<String>>,
    pub issuer: Option<String>,
    pub disable_iss_validation: bool,
    pub disable_local_ca_jwt: bool,
}

/// Response from executing
/// [ListRolesRequest][crate::api::auth::kubernetes::requests::ListRolesRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListRolesResponse {
    pub keys: Vec<String>,
}

/// Response from executing
/// [ReadKubernetesRoleRequest][crate::api::auth::kubernetes::requests::ReadKubernetesRoleRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadKubernetesRoleResponse {
    pub bound_service_account_names: Vec<String>,
    pub bound_service_account_namespaces: Vec<String>,
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
