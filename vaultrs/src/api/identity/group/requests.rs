use super::responses::{
    CreateGroupResponse, ListGroupsByIdResponse, ListGroupsByNameResponse, ReadGroupByIdResponse,
    ReadGroupByNameResponse,
};
use rustify_derive::Endpoint;
use serde::Serialize;
use std::{collections::HashMap, fmt::Debug};

/// ## Create an group
///
/// This endpoint creates or updates a group.
///
/// Note that it's not possible to set the ID or the name to update an existing group, [`identity::group::update_by_id`]
/// is the function to call for that use case.
///
/// * Path: identity/group
/// * Method: POST
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/group#create-a-group>
///
/// [`identity::group::update_by_id`]: crate::identity::group::update_by_id
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "identity/group",
    method = "POST",
    builder = "true",
    response = "CreateGroupResponse"
)]
#[builder(setter(into, strip_option), default)]
pub struct CreateGroupRequest {
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
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/group#read-group-by-id>
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
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/group#update-group-by-id>
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
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/group#delete-group-by-id>
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
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/group#list-groups-by-id>
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
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/group#create-a-group>
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
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/group#read-group-by-name>
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
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/group#delete-group-by-name>
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
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/group#list-groups-by-name>
#[derive(Builder, Debug, Endpoint, Default)]
#[endpoint(
    path = "identity/group/name",
    method = "LIST",
    builder = "true",
    response = "ListGroupsByNameResponse"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListGroupsByNameRequest {}
