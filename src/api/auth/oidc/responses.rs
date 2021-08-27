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
    pub bound_subject: Option<String>,
    pub bound_claims: Option<HashMap<String, String>>,
    pub bound_claims_type: Option<String>,
    pub bound_audiences: Option<Vec<String>>,
    pub claim_mappings: Option<HashMap<String, String>>,
    pub clock_skew_leeway: Option<String>,
    pub expiration_leeway: Option<String>,
    pub groups_claim: Option<String>,
    pub max_age: Option<String>,
    pub not_before_leeway: Option<String>,
    pub oidc_scopes: Option<Vec<String>>,
    pub role_type: Option<String>,
    pub token_bound_cidrs: Option<Vec<String>>,
    pub token_explicit_max_ttl: Option<String>,
    pub token_no_default_policy: Option<bool>,
    pub token_num_uses: Option<u64>,
    pub token_period: Option<String>,
    pub token_policies: Option<Vec<String>>,
    pub token_ttl: Option<String>,
    pub token_max_ttl: Option<String>,
    pub token_type: Option<String>,
    pub verbose_oidc_logging: Option<bool>,
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
