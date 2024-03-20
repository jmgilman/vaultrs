use super::responses::{
    ListRolesResponse, OIDCAuthResponse, ReadConfigurationResponse, ReadRoleResponse,
};
use rustify_derive::Endpoint;
use std::{collections::HashMap, fmt::Debug};

// ## Configure
/// Configures the validation information to be used globally across all roles.
///
/// * Path: /auth/jwt/config
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docsauth/jwt#configure>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "/auth/{self.mount}/config", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct SetConfigurationRequest {
    #[endpoint(skip)]
    pub mount: String,
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

/// ## Read Config
/// Returns the previously configured config.
///
/// * Path: /auth/{self.mount}/config
/// * Method: GET
/// * Response: [ReadConfigurationResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docsauth/jwt#read-config>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/config",
    response = "ReadConfigurationResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadConfigurationRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Create Role
/// Registers a role in the method.
///
/// * Path: /auth/{self.mount}/role/{self.name}
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docsauth/jwt#create-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/role/{self.name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct SetRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
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

/// ## Read Role
/// Returns the previously registered role configuration.
///
/// * Path: /auth/{self.mount}/role/{self.name}
/// * Method: GET
/// * Response: [ReadRoleResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docsauth/jwt#read-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/role/{self.name}",
    response = "ReadRoleResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}

/// ## List Roles
/// Lists all the roles that are registered with the plugin.
///
/// * Path: /auth/{self.mount}/role
/// * Method: LIST
/// * Response: [ListRolesResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docsauth/jwt#list-roles>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/role",
    method = "LIST",
    response = "ListRolesResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListRolesRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Delete Role
/// Deletes the previously registered role.
///
/// * Path: /auth/{self.mount}/role/{self.name}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docsauth/jwt#delete-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/role/{self.name}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}

/// ## OIDC Authorization URL Request
/// Obtain an authorization URL from Vault to start an OIDC login flow.
///
/// * Path: /auth/{self.mount}/oidc/auth_url
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docsauth/jwt#oidc-authorization-url-request>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/oidc/auth_url",
    method = "POST",
    response = "OIDCAuthResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct OIDCAuthRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub redirect_uri: String,
    pub role: Option<String>,
}

/// ## OIDC Callback
/// Exchange an authorization code for an OIDC ID Token.
///
/// * Path: /auth/{self.mount}/oidc/callback
/// * Method: GET
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docsauth/jwt#oidc-callback>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "/auth/{self.mount}/oidc/callback", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct OIDCCallbackRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(query)]
    #[endpoint(skip)]
    pub state: String,
    #[endpoint(query)]
    #[endpoint(skip)]
    pub nonce: String,
    #[endpoint(query)]
    #[endpoint(skip)]
    pub code: String,
}

/// ## JWT Login
/// This endpoint takes a signed JSON Web Token (JWT) and a role name for some
// entity.
///
/// * Path: /auth/{self.mount}/login
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docsauth/jwt#jwt-login>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "/auth/{self.mount}/login", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct JWTLoginRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub jwt: String,
    pub role: Option<String>,
}
