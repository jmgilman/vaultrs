use super::responses::{
    CreateEntityAliasResponse, CreateGroupAliasResponse, ListEntitiesByIdResponse,
    ListEntitiesByNameResponse, ListEntityAliasesByIdResponse, ListGroupAliasesByIdResponse,
    ListGroupsByIdResponse, ListGroupsByNameResponse, ReadEntityAliasByIdResponse,
    ReadEntityByIdResponse, ReadEntityByNameResponse, ReadGroupAliasByIdResponse,
    ReadGroupByIdResponse, ReadGroupByNameResponse,
};
use rustify_derive::Endpoint;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};

/// ## Create an entity
///
/// This endpoint creates or updates an Entity.
///
/// * Path: identity/entity
/// * Method: POST
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/entity#create-an-entity
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "identity/entity", method = "POST", builder = "true")]
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

/// ## Read entity by ID
///
/// This endpoint queries the entity by its identifier.
///
/// * Path: identity/entity/id/{self.id}
/// * Method: GET
/// * Response: [ReadEntityByIdResponse]
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/entity#read-entity-by-id
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
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/entity#update-entity-by-id
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
    pub disables: Option<bool>,
}

/// ## Delete entity by ID
///
/// This endpoint deletes an entity and all its associated aliases.
///
/// * Path: identity/entity/id/{self.id}
/// * Method: DELETE
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/entity#delete-entity-by-id
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
/// TThis endpoint deletes all entities provided.
///
/// * Path: identity/entity/batch-delete
/// * Method: POST
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/entity#batch-delete-entities
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
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/entity#list-entities-by-id
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
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/entity#create-update-entity-by-name
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
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/entity#read-entity-by-name
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

/// ## Delete entity by nsm
///
/// This endpoint deletes an entity and all its associated aliases, given the entity name.
///
/// * Path: identity/entity/name/{self.name}
/// * Method: DELETE
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/entity#delete-entity-by-name
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
/// This endpoint returns a list of available entities by their identifiers.
///
/// * Path: identity/entity/name
/// * Method: LIST
/// * Response: [ListEntitiesByNameResponse]
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/entity#list-entities-by-name
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
/// This endpoint returns a list of available entities by their identifiers.
///
/// * Path: identity/entity/merge
/// * Method: POST
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/entity#merge-entities
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

/// ## Create an entity alias
///
/// This endpoint creates a new alias for an entity.
///
/// * Path: identity/entity-alias
/// * Method: POST
/// * Response: [Option<CreateEntityAliasResponse>]
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/entity-alias#create-an-entity-alias
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
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/entity-alias#read-entity-alias-by-id
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
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/entity-alias#update-entity-alias-by-id
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
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/entity-alias#delete-entity-alias-by-id
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
/// * Response: [ListEntityAliasesByIdResponse ]
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/entity-alias#list-entity-alias-by-id
#[derive(Builder, Debug, Endpoint, Default)]
#[endpoint(
    path = "identity/entity-alias/id",
    method = "LIST",
    builder = "true",
    response = "ListEntityAliasesByIdResponse"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListEntityAliasesByIdRequest {}

/// ## Create an group
///
/// This endpoint creates or updates a group.
///
/// * Path: identity/group
/// * Method: POST
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/group#create-a-group
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(path = "identity/group", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct CreateGroupRequest {
    /// Name of the group. If set (and ID is not set), updates the corresponding existing group.
    pub name: String,
    /// ID of the group. If set, updates the corresponding existing group.
    pub id: Option<String>,
    /// Type of the group, internal or external. Defaults to internal.
    #[serde(rename = "type")]
    pub group_type: Option<String>,
    /// Metadata to be associated with the entity.
    pub metadata: Option<HashMap<String, String>>,
    /// Policies to be tied to the group.
    pub policies: Option<Vec<String>>,
    /// Group IDs to be assigned as group members.
    pub member_group_ids: Option<Vec<String>>,
    /// Entity IDs to be assigned as group members.
    pub member_entity_ids: Option<Vec<String>>,
}

/// ## Read group by ID
///
/// This endpoint queries the group by its identifier.
///
/// * Path: identity/group/id/{self.id}
/// * Method: GET
/// * Response: [ReadGroupByIdResponse]
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/group#read-group-by-id
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "identity/group/id/{self.id}",
    method = "GET",
    builder = "true",
    response = "ReadGroupByIdResponse"
)]
#[builder(setter(into))]
pub struct ReadGroupByIdRequest {
    /// Identifier of the group.
    #[endpoint(skip)]
    pub id: String,
}

/// ## Update group by ID
///
/// This endpoint is used to update a existing group.
///
/// * Path: identity/group/id/{self.id}
/// * Method: POST
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/group#update-group-by-id
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "identity/group/id/{self.id}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct UpdateGroupByIdRequest {
    /// Identifier of the group.
    #[endpoint(skip)]
    pub id: String,
    /// Name of the group.
    pub name: Option<String>,
    /// Type of the group, internal or external. Defaults to internal.
    #[serde(rename = "type")]
    pub group_type: Option<String>,
    /// Metadata to be associated with the group.
    pub metadata: Option<HashMap<String, String>>,
    /// Policies to be tied to the group.
    pub policies: Option<Vec<String>>,
    /// Group IDs to be assigned as group members.
    pub member_group_ids: Option<Vec<String>>,
    /// Entity IDs to be assigned as group members.
    pub member_entity_ids: Option<Vec<String>>,
}

/// ## Delete group by ID
///
/// This endpoint deletes a group.
///
/// * Path: identity/group/id/{self.id}
/// * Method: DELETE
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/group#delete-group-by-id
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "identity/group/id/{self.id}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteGroupByIdRequest {
    /// Identifier of the group.
    #[endpoint(skip)]
    pub id: String,
}

/// ## List groups by ID
///
/// This endpoint returns a list of available groups by their identifiers.
///
/// * Path: identity/group/id
/// * Method: LIST
/// * Response: [ListGroupsByIdResponse]
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/group#list-groups-by-id
#[derive(Builder, Debug, Endpoint, Default)]
#[endpoint(
    path = "identity/group/id",
    method = "LIST",
    builder = "true",
    response = "ListGroupsByIdResponse"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListGroupsByIdRequest {}

/// ## Create an group
///
/// This endpoint creates or updates a group.
///
/// * Path: identity/group/name/{self.name}
/// * Method: POST
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/group#create-a-group
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "identity/group/name/{self.name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct CreateGroupByNameRequest {
    /// Name of the group.
    pub name: String,
    /// Type of the group, internal or external. Defaults to internal.
    #[serde(rename = "type")]
    pub group_type: Option<String>,
    /// Metadata to be associated with the entity.
    pub metadata: Option<HashMap<String, String>>,
    /// Policies to be tied to the group.
    pub policies: Option<Vec<String>>,
    /// Group IDs to be assigned as group members.
    pub member_group_ids: Option<Vec<String>>,
    /// Entity IDs to be assigned as group members.
    pub member_entity_ids: Option<Vec<String>>,
}

/// ## Read group by name
///
/// This endpoint queries the group by its identifier.
///
/// * Path: identity/group/name/{self.name}
/// * Method: GET
/// * Response: [ReadGroupByNameResponse]
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/group#read-group-by-name
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "identity/group/name/{self.name}",
    method = "GET",
    builder = "true",
    response = "ReadGroupByNameResponse"
)]
#[builder(setter(into))]
pub struct ReadGroupByNameRequest {
    /// Name of the group.
    #[endpoint(skip)]
    pub name: String,
}

/// ## Delete group by name
///
/// This endpoint deletes a group.
///
/// * Path: identity/group/name/{self.name}
/// * Method: DELETE
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/group#delete-group-by-name
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "identity/group/name/{self.name}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteGroupByNameRequest {
    /// Identifier of the group.
    #[endpoint(skip)]
    pub name: String,
}

/// ## List groups by name
///
/// This endpoint returns a list of available groups by their identifiers.
///
/// * Path: identity/group/name
/// * Method: LIST
/// * Response: [ListGroupsByNameResponse]
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/group#list-groups-by-name
#[derive(Builder, Debug, Endpoint, Default)]
#[endpoint(
    path = "identity/group/name",
    method = "LIST",
    builder = "true",
    response = "ListGroupsByNameResponse"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListGroupsByNameRequest {}

/// ## Create a group alias
///
/// This endpoint creates or updates a group alias.
///
/// * Path: identity/group-alias
/// * Method: POST
/// * Response: [CreateGroupAliasResponse]
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/group-alias#create-a-group-alias
#[derive(Builder, Debug, Default, Endpoint, Deserialize, Serialize)]
#[endpoint(
    path = "identity/group-alias",
    method = "POST",
    builder = "true",
    response = "CreateGroupAliasResponse"
)]
#[builder(setter(into, strip_option), default)]
pub struct CreateGroupAliasRequest {
    /// Name of the alias.
    pub name: String,
    /// ID of the group alias. If set, updates the corresponding group alias.
    pub id: Option<String>,
    /// Mount accessor which this alias belongs to.
    pub mount_accessor: String,
    /// ID of the group to which this is an alias.
    pub canonical_id: Option<String>,
}

/// ## Update group alias by ID
///
/// This endpoint is used to update a existing group alias.
///
/// * Path: identity/group-alias/id/{self.id}
/// * Method: POST
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/group-alias#update-group-alias-by-id
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "identity/group-alias/id/{self.id}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct UpdateGroupAliasByIdRequest {
    /// Identifier of the group alias.
    #[endpoint(skip)]
    pub id: String,
    /// Name of the group alias.
    pub name: Option<String>,
    /// Mount accessor which this alias belongs to.
    pub mount_accessor: String,
    /// ID of the group to which this is an alias.
    pub canonical_id: Option<String>,
}

/// ## Read group alias by ID
///
/// This endpoint queries the group alias by its identifier.
///
/// * Path: identity/group-alias/id/{self.id}
/// * Method: GET
/// * Response: [ReadGroupAliasByIdResponse]
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/group-alias#read-group-alias-by-id
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "identity/group-alias/id/{self.id}",
    method = "GET",
    builder = "true",
    response = "ReadGroupAliasByIdResponse"
)]
#[builder(setter(into))]
pub struct ReadGroupAliasByIdRequest {
    /// Identifier of the group alias.
    #[endpoint(skip)]
    pub id: String,
}

/// ## Delete group alias by ID
///
/// This endpoint deletes a group alias.
///
/// * Path: identity/group-alias/id/{self.id}
/// * Method: DELETE
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/group-alias#delete-group-alias-by-id
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "identity/group-alias/id/{self.id}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteGroupAliasByIdRequest {
    /// ID of the group alias.
    #[endpoint(skip)]
    pub id: String,
}

/// ## List group alias by ID
///
/// This endpoint returns a list of available group aliases by their identifiers.
///
/// * Path: identity/group-alias/id
/// * Method: LIST
/// * Response: [ListEntityAliasesByIdResponse ]
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/group-alias#list-group-alias-by-id
#[derive(Builder, Debug, Endpoint, Default)]
#[endpoint(
    path = "identity/group-alias/id",
    method = "LIST",
    builder = "true",
    response = "ListGroupAliasesByIdResponse"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListGroupAliasesByIdRequest {}
