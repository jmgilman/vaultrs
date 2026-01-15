use serde::{Deserialize, Serialize};

/// Response from executing
/// [ReadCaCertificateRoleRequest][crate::api::auth::cert::requests::ReadCaCertificateRoleRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadCaCertificateRoleResponse {
    pub allowed_common_names: Option<Vec<String>>,
    pub allowed_dns_sans: Option<Vec<String>>,
    pub allowed_email_sans: Option<Vec<String>>,
    pub allowed_metadata_extensions: Option<Vec<String>>,
    pub allowed_organizational_units: Option<Vec<String>>,
    pub allowed_uri_sans: Option<Vec<String>>,
    pub certificate: String,
    pub display_name: String,
    pub required_extensions: Option<Vec<String>>,
    pub token_bound_cidrs: Vec<String>,
    pub token_explicit_max_ttl: u64,
    pub token_max_ttl: u64,
    pub token_no_default_policy: bool,
    pub token_num_uses: u64,
    pub token_period: u64,
    pub token_policies: Vec<String>,
    pub token_ttl: u64,
    pub token_type: String,
}

/// Response from executing
/// [ListCaCertificateRoleRequest][crate::api::auth::cert::requests::ListCaCertificateRoleRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListCaCertificateRoleResponse {
    pub keys: Vec<String>,
}
