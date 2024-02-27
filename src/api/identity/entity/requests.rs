use rustify_derive::Endpoint;
use serde::Serialize;
use std::{collections::HashMap, fmt::Debug};

use super::responses::{
    CreateEntityResponse, ListEntitiesByIdResponse, ListEntitiesByNameResponse,
    ReadEntityByIdResponse, ReadEntityByNameResponse,
};

/// ## Create an entity
///
/// This endpoint creates or updates an Entity.
///
/// Note that it's not possible to set the ID to update an existing entity, [`identity::entity::update_by_id`]
/// is the function to call for that use case.
///
/// * Path: identity/entity
/// * Method: POST
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/entity#create-an-entity>
///
/// [`identity::entity::update_by_id`]: crate::identity::entity::update_by_id
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "identity/entity",
    method = "POST",
    builder = "true",
    response = "CreateEntityResponse"
)]
#[builder(setter(into, strip_option), default)]
pub struct CreateEntityRequest {
    /// Name of the entity.
    pub name: Option<String>,
    /// Metadata to be associated with the entity.
    pub metadata: Option<HashMap<String, String>>,
    /// Policies to be tied to the entity.
    pub policies: Option<Vec<String>>,
    /// Whether the entity is disabled. Disabled entities' associated tokens cannot be used, but are not revoked.
    pub disabled: Option<bool>,
}

/// ## Read entity by ID
///
/// This endpoint queries the entity by its identifier.
///
/// * Path: identity/entity/id/{self.id}
/// * Method: GET
/// * Response: [ReadEntityByIdResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/entity#read-entity-by-id>
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "identity/entity/id/{self.id}",
    method = "GET",
    builder = "true",
    response = "ReadEntityByIdResponse"
)]
#[builder(setter(into))]
pub struct ReadEntityByIdRequest {
    /// Identifier of the entity.
    #[endpoint(skip)]
    pub id: String,
}

/// ## Update entity by ID
///
/// This endpoint is used to update an existing entity.
///
/// * Path: identity/entity/id/{self.id}
/// * Method: POST
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/entity#update-entity-by-id>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "identity/entity/id/{self.id}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct UpdateEntityByIdRequest {
    /// Identifier of the entity.
    #[endpoint(skip)]
    pub id: String,
    /// Name of the entity.
    pub name: Option<String>,
    /// Metadata to be associated with the entity.
    pub metadata: Option<HashMap<String, String>>,
    /// Policies to be tied to the entity.
    pub policies: Option<Vec<String>>,
    /// Whether the entity is disabled. Disabled entities' associated tokens cannot be used, but are not revoked.
    pub disabled: Option<bool>,
}

/// ## Delete entity by ID
///
/// This endpoint deletes an entity and all its associated aliases.
///
/// * Path: identity/entity/id/{self.id}
/// * Method: DELETE
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/entity#delete-entity-by-id>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "identity/entity/id/{self.id}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteEntityByIdRequest {
    /// Identifier of the entity.
    #[endpoint(skip)]
    pub id: String,
}

/// ## Batch delete entities
///
/// This endpoint deletes all entities provided.
///
/// * Path: identity/entity/batch-delete
/// * Method: POST
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/entity#batch-delete-entities>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "identity/entity/batch-delete",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct BatchDeleteRequest {
    /// List of entity identifiers to delete.
    pub entity_ids: Vec<String>,
}

/// ## List entities by ID
///
/// This endpoint returns a list of available entities by their identifiers.
///
/// * Path: identity/entity/id
/// * Method: LIST
/// * Response: [ListEntitiesByIdResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/entity#list-entities-by-id>
#[derive(Builder, Debug, Endpoint, Default)]
#[endpoint(
    path = "identity/entity/id",
    method = "LIST",
    builder = "true",
    response = "ListEntitiesByIdResponse"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListEntitiesByIdRequest {}

/// ## Create/Update an entity by name
///
/// This endpoint is used to create or update an entity by a given name.
///
/// * Path: identity/entity/name/{self.name}
/// * Method: POST
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/entity#create-update-entity-by-name>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "identity/entity/name/{self.name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct CreateEntityByNameRequest {
    /// Name of the entity.
    #[endpoint(skip)]
    pub name: String,
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
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/entity#read-entity-by-name>
#[derive(Builder, Debug, Endpoint, Default)]
#[endpoint(
    path = "identity/entity/name/{self.name}",
    method = "GET",
    builder = "true",
    response = "ReadEntityByNameResponse"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadEntityByNameRequest {
    /// Name of the entity.
    #[endpoint(skip)]
    pub name: String,
}

/// ## Delete entity by name
///
/// This endpoint deletes an entity and all its associated aliases, given the entity name.
///
/// * Path: identity/entity/name/{self.name}
/// * Method: DELETE
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/entity#delete-entity-by-name>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "identity/entity/name/{self.name}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteEntityByNameRequest {
    /// Name of the entity.
    #[endpoint(skip)]
    pub name: String,
}

/// ## List entities by name
///
/// This endpoint returns a list of available entities by their names.
///
/// * Path: identity/entity/name
/// * Method: LIST
/// * Response: [ListEntitiesByNameResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/entity#list-entities-by-name>
#[derive(Builder, Debug, Endpoint, Default)]
#[endpoint(
    path = "identity/entity/name",
    method = "LIST",
    builder = "true",
    response = "ListEntitiesByNameResponse"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListEntitiesByNameRequest {}

/// ## Merge entities
///
/// This endpoint merges many entities into one entity.
///
/// * Path: identity/entity/merge
/// * Method: POST
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/entity#merge-entities>
#[derive(Builder, Debug, Endpoint, Default)]
#[endpoint(path = "identity/entity/merge", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct MergeEntitiesRequest {
    /// Entity IDs which need to get merged.
    pub from_entity_ids: Vec<String>,
    /// Entity ID into which all the other entities need to get merged.
    pub to_entity_id: String,
    /// Setting this will follow the 'mine' strategy for merging MFA secrets.
    /// If there are secrets of the same type both in entities that are merged from and in entity into
    /// which all others are getting merged, secrets in the destination will be unaltered.
    /// If not set, this API will throw an error containing all the conflicts.
    pub force: Option<bool>,
    /// A list of entity aliases to keep in the case where the to-Entity and from-Entity have aliases
    /// with the same mount accessor. In the case where alias share mount accessors, the alias ID given
    /// in this list will be kept or merged, and the other alias will be deleted.
    /// Note that merges requiring this parameter must have only one from-Entity.
    pub conflicting_alias_ids_to_keep: Option<String>,
}
