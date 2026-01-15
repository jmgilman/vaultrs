use super::responses::{
    ListAccessorResponse, ListTokenRolesResponse, LookupTokenResponse, ReadTokenRoleResponse,
};
use rustify_derive::Endpoint;
use serde::Serialize;
use std::{collections::HashMap, fmt::Debug};

/// ## List Accessors
/// This endpoint lists token accessors.
///
/// * Path: /auth/token/accessors
/// * Method: LIST
/// * Response: [ListAccessorResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/token#list-accessors>

#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/token/accessors",
    method = "LIST",
    response = "ListAccessorResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListAccessorRequest {}

/// ## Create Token
/// Creates a new token.
///
/// * Path: /auth/token/create
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/token#create-token>
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(path = "/auth/token/create", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct CreateTokenRequest {
    pub display_name: Option<String>,
    pub entity_alias: Option<String>,
    pub explicit_max_ttl: Option<String>,
    pub id: Option<String>,
    pub lease: Option<String>,
    pub meta: Option<HashMap<String, String>>,
    pub no_default_policy: Option<bool>,
    pub no_parent: Option<bool>,
    pub num_uses: Option<u64>,
    pub policies: Option<Vec<String>>,
    pub period: Option<String>,
    pub renewable: Option<bool>,
    pub ttl: Option<String>,
    #[serde(rename = "type")]
    pub token_type: Option<String>,
}

/// ## Create Orphan Token
/// Creates a new orphan token.
///
/// * Path: /auth/token/create-orphan
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/token#create-token>
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(path = "/auth/token/create-orphan", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct CreateOrphanTokenRequest {
    pub display_name: Option<String>,
    pub entity_alias: Option<String>,
    pub explicit_max_ttl: Option<String>,
    pub id: Option<String>,
    pub lease: Option<String>,
    pub meta: Option<HashMap<String, String>>,
    pub no_default_policy: Option<bool>,
    pub no_parent: Option<bool>,
    pub num_uses: Option<u64>,
    pub policies: Option<Vec<String>>,
    pub period: Option<String>,
    pub renewable: Option<bool>,
    pub ttl: Option<String>,
    #[serde(rename = "type")]
    pub token_type: Option<String>,
}

/// ## Create Role Token
/// Creates a new role token.
///
/// * Path: /auth/token/create/{self.role_name}
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/token#create-token>
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "/auth/token/create/{self.role_name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct CreateRoleTokenRequest {
    #[endpoint(skip)]
    pub role_name: String,
    pub display_name: Option<String>,
    pub entity_alias: Option<String>,
    pub explicit_max_ttl: Option<String>,
    pub id: Option<String>,
    pub lease: Option<String>,
    pub meta: Option<HashMap<String, String>>,
    pub no_default_policy: Option<bool>,
    pub no_parent: Option<bool>,
    pub num_uses: Option<u64>,
    pub policies: Option<Vec<String>>,
    pub period: Option<String>,
    pub renewable: Option<bool>,
    pub ttl: Option<String>,
    #[serde(rename = "type")]
    pub token_type: Option<String>,
}

/// ## Lookup a Token
/// Returns information about the client token.
///
/// * Path: /auth/token/lookup
/// * Method: POST
/// * Response: [LookupTokenResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/token#lookup-a-token>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/token/lookup",
    method = "POST",
    response = "LookupTokenResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct LookupTokenRequest {
    pub token: String,
}

/// ## Lookup a Token (Self)
/// Returns information about the current client token.
///
/// * Path: /auth/token/lookup-self
/// * Method: GET
/// * Response: [LookupTokenResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/token#lookup-a-token-self>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/token/lookup-self",
    response = "LookupTokenResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct LookupTokenSelfRequest {}

/// ## Lookup a Token (Accessor)
/// Returns information about the client token from the accessor.
///
/// * Path: /auth/token/lookup-accessor
/// * Method: POST
/// * Response: [LookupTokenResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/token#lookup-a-token-accessor>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/token/lookup-accessor",
    method = "POST",
    response = "LookupTokenResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct LookupTokenAccessorRequest {
    pub accessor: String,
}

/// ## Renew a Token
/// Renews a lease associated with a token.
///
/// * Path: /auth/token/renew
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/token#renew-a-token>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "/auth/token/renew", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct RenewTokenRequest {
    pub token: String,
    pub increment: Option<String>,
}

/// ## Renew a Token (Self)
/// Renews a lease associated with the calling token.
///
/// * Path: /auth/token/renew-self
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/token#renew-a-token-self>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "	/auth/token/renew-self", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct RenewTokenSelfRequest {
    pub increment: Option<String>,
}

/// ## Renew a Token (Accessor)
/// Renews a lease associated with a token using its accessor.
///
/// * Path: /auth/token/renew-accessor
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/token#renew-a-token-self>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "/auth/token/renew-accessor", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct RenewTokenAccessorRequest {
    pub accessor: String,
    pub increment: Option<String>,
}

/// ## Revoke a Token
/// Revokes a token and all child tokens
///
/// * Path: /auth/token/revoke
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/token#revoke-a-token>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "/auth/token/revoke", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct RevokeTokenRequest {
    pub token: String,
}

/// ## Revoke a Token (Self)
/// Revokes the token used to call it and all child tokens.
///
/// * Path: /auth/token/revoke-self
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/token#revoke-a-token-self>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "	/auth/token/revoke-self", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct RevokeTokenSelfRequest {}

/// ## Revoke a Token Accessor
/// Revoke the token associated with the accessor and all the child tokens.
///
/// * Path: /auth/token/revoke-accessor
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/token#revoke-a-token-accessor>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/token/revoke-accessor",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct RevokeTokenAccessorRequest {
    pub accessor: String,
}

/// ## Revoke Token and Orphan Children
/// Revokes a token but not its child tokens.
///
/// * Path: /auth/token/revoke-orphan
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/token#revoke-token-and-orphan-children>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "/auth/token/revoke-orphan", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct RevokeTokenOrphanRequest {
    pub token: String,
}

/// ## Read Token Role
/// Fetches the named role configuration.
///
/// * Path: /auth/token/roles/{self.role_name}
/// * Method: GET
/// * Response: [ReadTokenRoleResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/token#read-token-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/token/roles/{self.role_name}",
    response = "ReadTokenRoleResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadTokenRoleRequest {
    #[endpoint(skip)]
    pub role_name: String,
}

/// ## List Token Roles
/// List available token roles.
///
/// * Path: /auth/token/roles
/// * Method: GET
/// * Response: [ListTokenRolesResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/token#list-token-roles>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/token/roles",
    method = "LIST",
    response = "ListTokenRolesResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListTokenRolesRequest {}

/// ## Create/Update Token Role
/// List available token roles.
///
/// * Path: /auth/token/roles/:role_name
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/token#create-update-token-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/token/roles/{self.role_name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct SetTokenRoleRequest {
    #[endpoint(skip)]
    pub role_name: String,
    pub allowed_entity_aliases: Option<Vec<String>>,
    pub allowed_policies: Option<Vec<String>>,
    pub disallowed_policies: Option<Vec<String>>,
    pub orphan: Option<bool>,
    pub path_suffix: Option<String>,
    pub renewable: Option<bool>,
    pub token_bound_cidrs: Option<Vec<String>>,
    pub token_explicit_max_ttl: Option<String>,
    pub token_no_default_policy: Option<bool>,
    pub token_num_uses: Option<u64>,
    pub token_period: Option<String>,
    pub token_type: Option<String>,
}

/// ## Delete Token Role
/// This endpoint deletes the named token role.
///
/// * Path: /auth/token/roles/{self.role_name}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/token#delete-token-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/token/roles/{self.role_name}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteTokenRoleRequest {
    #[endpoint(skip)]
    pub role_name: String,
}

/// ## Tidy Tokens
/// Performs some maintenance tasks to clean up invalid entries that may remain
// in the token store.
///
/// * Path: /auth/token/tidy
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/token#tidy-tokens>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "/auth/token/tidy", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct TidyRequest {}
