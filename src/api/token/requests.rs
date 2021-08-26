use super::responses::{ListAccessorResponse, TokenLookupResponse};
use rustify_derive::Endpoint;
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::{collections::HashMap, fmt::Debug};

/// ## List Accessors
/// This endpoint lists token accessors.
///
/// * Path: /auth/token/accessors
/// * Method: LIST
/// * Response: [ListAccessorResponse]
/// * Reference: https://www.vaultproject.io/api-docs/auth/token#list-accessors
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
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
/// * Reference: https://www.vaultproject.io/api-docs/auth/token#create-token
#[skip_serializing_none]
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
/// * Reference: https://www.vaultproject.io/api-docs/auth/token#create-token
#[skip_serializing_none]
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
/// * Reference: https://www.vaultproject.io/api-docs/auth/token#create-token
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "/auth/token/create/{self.role_name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct CreateRoleTokenRequest {
    #[serde(skip)]
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
/// * Response: TokenLookupResponse
/// * Reference: https://www.vaultproject.io/api-docs/auth/token#lookup-a-token
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "/auth/token/lookup",
    method = "POST",
    response = "TokenLookupResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct TokenLookupRequest {
    pub token: String,
}

/// ## Lookup a Token (Self)
/// Returns information about the current client token.
///
/// * Path: /auth/token/lookup-self
/// * Method: GET
/// * Response: TokenLookupResponse
/// * Reference: https://www.vaultproject.io/api-docs/auth/token#lookup-a-token-self
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "/auth/token/lookup-self",
    response = "TokenLookupResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct TokenLookupSelfRequest {}

/// ## Lookup a Token (Accessor)
/// Returns information about the client token from the accessor.
///
/// * Path: /auth/token/lookup-accessor
/// * Method: POST
/// * Response: TokenLookupResponse
/// * Reference: https://www.vaultproject.io/api-docs/auth/token#lookup-a-token-accessor
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "/auth/token/lookup-accessor",
    method = "POST",
    response = "TokenLookupResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct TokenLookupAccessorRequest {
    pub accessor: String,
}

/// ## Renew a Token
/// Renews a lease associated with a token.
///
/// * Path: /auth/token/renew
/// * Method: POST
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/auth/token#renew-a-token
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(path = "/auth/token/renew", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct TokenRenewRequest {
    pub token: String,
    pub increment: Option<String>,
}

/// ## Renew a Token (Self)
/// Renews a lease associated with the calling token.
///
/// * Path: /auth/token/renew-self
/// * Method: POST
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/auth/token#renew-a-token-self
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(path = "	/auth/token/renew-self", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct TokenRenewSelfRequest {
    pub increment: Option<String>,
}

/// ## Renew a Token (Accessor)
///Renews a lease associated with a token using its accessor.
///
/// * Path: /auth/token/renew-accessor
/// * Method: POST
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/auth/token#renew-a-token-self
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(path = "	/auth/token/renew-accessor", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct TokenRenewAccessorRequest {
    pub accessor: String,
    pub increment: Option<String>,
}
