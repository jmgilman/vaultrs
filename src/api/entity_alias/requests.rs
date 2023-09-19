use super::responses::CreateEntityAliasResponse;
use rustify_derive::Endpoint;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// ### Create an entity alias
/// This endpoint creates a new alias for an entity.
///
/// * Path: identity/entity-alias
/// * Method: POST
/// * Response: [CreateEntityAliasResponse]
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/entity-alias#create-an-entity-alias
#[derive(Builder, Debug, Default, Endpoint, Deserialize, Serialize)]
#[endpoint(
    path = "identity/entity-alias",
    response = "CreateEntityAliasResponse",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into), default)]
pub struct CreateEntityAliasRequest {
    pub name: String,
    pub canonical_id: String,
    pub mount_accessor: String,
}
