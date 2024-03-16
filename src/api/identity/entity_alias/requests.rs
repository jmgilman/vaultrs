use super::responses::{
    CreateEntityAliasResponse, ListEntityAliasesByIdResponse, ReadEntityAliasByIdResponse,
};
use rustify_derive::Endpoint;
use serde::Serialize;
use std::{collections::HashMap, fmt::Debug};

/// ## Create an entity alias
///
/// This endpoint creates a new alias for an entity.
///
/// * Path: identity/entity-alias
/// * Method: POST
/// * Response: [`Option<CreateEntityAliasResponse>`]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/entity-alias#create-an-entity-alias>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "identity/entity-alias",
    response = "CreateEntityAliasResponse",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct CreateEntityAliasRequest {
    /// Name of the alias. Name should be the identifier of the client in the authentication source.
    /// For example, if the alias belongs to userpass backend, the name should be a valid username within userpass auth method.
    /// If the alias belongs to GitHub, it should be the GitHub username.
    /// If the alias belongs to an approle auth method, the name should be a valid RoleID.
    pub name: String,
    /// Entity ID to which this alias belongs to.
    pub canonical_id: String,
    ///  Accessor of the mount to which the alias should belong to.
    pub mount_accessor: String,
    /// ID of the entity alias. If set, updates the corresponding entity alias.
    pub id: Option<String>,
    /// A map of arbitrary string to string valued user-provided metadata meant to describe the alias.
    pub custom_metadata: Option<HashMap<String, String>>,
}

/// ## Read entity alias by ID
///
/// This endpoint queries the entity alias by its identifier.
///
/// * Path: identity/entity-alias/id/{self.id}
/// * Method: GET
/// * Response: [ReadEntityAliasByIdResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/entity-alias#read-entity-alias-by-id>
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "identity/entity-alias/id/{self.id}",
    method = "GET",
    builder = "true",
    response = "ReadEntityAliasByIdResponse"
)]
#[builder(setter(into))]
pub struct ReadEntityAliasByIdRequest {
    /// Identifier of the entity alias.
    #[endpoint(skip)]
    pub id: String,
}

/// ## Update entity alias by ID
///
/// This endpoint is used to update an existing entity alias.
///
/// * Path: identity/entity-alias/id/{self.id}
/// * Method: POST
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/entity-alias#update-entity-alias-by-id>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "identity/entity-alias/id/{self.id}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct UpdateEntityAliasByIdRequest {
    /// Identifier of the entity alias.
    #[endpoint(skip)]
    pub id: String,
    /// Name of the alias. Name should be the identifier of the client in the authentication source.
    /// For example, if the alias belongs to userpass backend, the name should be a valid username within userpass backend.
    /// If alias belongs to GitHub, it should be the GitHub username.
    pub name: Option<String>,
    /// Entity ID to which this alias belongs to.
    pub canonical_id: Option<String>,
    /// Accessor of the mount to which the alias should belong to.
    pub mount_accessor: Option<String>,
    /// A map of arbitrary string to string valued user-provided metadata meant to describe the alias.
    pub custom_metadata: Option<HashMap<String, String>>,
}

/// ## Delete entity alias by ID
///
/// This endpoint deletes an alias from its corresponding entity.
///
/// * Path: identity/entity-alias/id/{self.id}
/// * Method: DELETE
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/entity-alias#delete-entity-alias-by-id>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "identity/entity-alias/id/{self.id}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteEntityAliasByIdRequest {
    /// Identifier of the entity alias.
    #[endpoint(skip)]
    pub id: String,
}

/// ## List entity alias by ID
///
/// The list by ID endpoint returns the available entity aliases and key data by their identifiers.
///
/// * Path: identity/entity-alias/id
/// * Method: LIST
/// * Response: [ListEntityAliasesByIdResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/entity-alias#list-entity-alias-by-id>
#[derive(Builder, Debug, Endpoint, Default)]
#[endpoint(
    path = "identity/entity-alias/id",
    method = "LIST",
    builder = "true",
    response = "ListEntityAliasesByIdResponse"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListEntityAliasesByIdRequest {}
