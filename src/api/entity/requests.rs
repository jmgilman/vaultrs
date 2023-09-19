use crate::api::entity::responses::CreateEntityResponse;
use crate::api::entity::responses::ReadEntityByNameResponse;

use rustify_derive::Endpoint;
use std::fmt::Debug;

/// ## Create an entity
/// This endpoint creates or updates an Entity.
///
/// * Path: identity/entity
/// * Method: POST
/// * Response: CreateEntityResponse
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/entity#create-an-entity
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "identity/entity",
    response = "CreateEntityResponse",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into), default)]
pub struct CreateEntityRequest {
    pub name: String,
    pub policies: String,
}

/// ## Read entity by name
/// This endpoint queries the entity by its name.
///
/// * Path: identity/entity/name/{self.name}
/// * Method: GET
/// * Response: ReadEntityResponse
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
