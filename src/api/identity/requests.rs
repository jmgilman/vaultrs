use super::responses::{CreateEntityAliasResponse, CreateEntityResponse, ReadEntityByNameResponse};
use rustify_derive::Endpoint;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};

/// ## Create an entity
///
/// This endpoint creates or updates an Entity.
///
/// * Path: identity/entity
/// * Method: POST
/// * Response: [CreateEntityResponse]
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/entity#create-an-entity
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "identity/entity",
    response = "CreateEntityResponse",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct CreateEntityRequest {
    /// Name of the entity.
    pub name: String,
    /// ID of the entity. If set, updates the corresponding existing entity.
    pub id: Option<String>,
    /// Metadata to be associated with the entity.
    pub metadata: Option<HashMap<String, String>>,
    /// Policies to be tied to the entity.
    pub policies: Option<Vec<String>>,
    /// Whether the entity is disabled. Disabled entities' associated tokens cannot be used, but are not revoked.
    pub disabled: Option<bool>,
}

/// ## Read entity by name
/// This endpoint queries the entity by its name.
///
/// * Path: identity/entity/name/{self.name}
/// * Method: GET
/// * Response: [ReadEntityByNameResponse]
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/entity#read-entity-by-name
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "identity/entity/name/{self.name}",
    method = "GET",
    builder = "true",
    response = "ReadEntityByNameResponse"
)]
#[builder(setter(into))]
pub struct ReadEntityByNameRequest {
    pub name: String,
}

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
