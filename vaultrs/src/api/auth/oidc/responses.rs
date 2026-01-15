use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Response from executing
/// [ReadConfigurationRequest][crate::api::auth::oidc::requests::ReadConfigurationRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadConfigurationResponse {
    pub bound_issuer: Option<String>,
    pub default_role: Option<String>,
    pub jwks_ca_pem: Option<String>,
    pub jwt_supported_algs: Option<Vec<String>>,
    pub jwks_url: Option<String>,
    pub jwt_validation_pubkeys: Option<Vec<String>>,
    pub namespace_in_state: Option<bool>,
    pub oidc_discovery_ca_pem: Option<String>,
    pub oidc_discovery_url: Option<String>,
    pub oidc_client_id: Option<String>,
    pub oidc_client_secret: Option<String>,
    pub oidc_response_mode: Option<String>,
    pub oidc_response_types: Option<Vec<String>>,
    pub provider_config: Option<HashMap<String, String>>,
}

/// Response from executing
/// [ReadRoleRequest][crate::api::auth::oidc::requests::ReadRoleRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadRoleResponse {
    pub allowed_redirect_uris: Vec<String>,
    pub user_claim: String,
    pub bound_subject: String,
    pub bound_claims: Option<HashMap<String, String>>,
    pub bound_claims_type: String,
    pub bound_audiences: Option<Vec<String>>,
    pub claim_mappings: Option<HashMap<String, String>>,
    pub clock_skew_leeway: u64,
    pub expiration_leeway: u64,
    pub groups_claim: String,
    pub max_age: u64,
    pub not_before_leeway: u64,
    pub oidc_scopes: Option<Vec<String>>,
    pub role_type: String,
    pub token_bound_cidrs: Vec<String>,
    pub token_explicit_max_ttl: u64,
    pub token_no_default_policy: bool,
    pub token_num_uses: u64,
    pub token_period: u64,
    pub token_policies: Vec<String>,
    pub token_ttl: u64,
    pub token_max_ttl: u64,
    pub token_type: String,
    pub verbose_oidc_logging: bool,
}

/// Response from executing
/// [ListRolesRequest][crate::api::auth::oidc::requests::ListRolesRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListRolesResponse {
    pub keys: Vec<String>,
}

/// Response from executing
/// [OIDCAuthRequest][crate::api::auth::oidc::requests::OIDCAuthRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct OIDCAuthResponse {
    pub auth_url: String,
}
