use super::responses::ListAccessorResponse;
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
    result = "ListAccessorResponse",
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
