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
    display_name: Option<String>,
    entity_alias: Option<String>,
    explicit_max_ttl: Option<String>,
    id: Option<String>,
    lease: Option<String>,
    meta: Option<HashMap<String, String>>,
    no_default_policy: Option<bool>,
    no_parent: Option<bool>,
    num_uses: Option<u64>,
    policies: Option<Vec<String>>,
    period: Option<String>,
    renewable: Option<bool>,
    ttl: Option<String>,
    #[serde(rename = "type")]
    token_type: Option<String>,
}
