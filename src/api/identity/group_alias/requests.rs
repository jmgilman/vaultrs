use super::responses::{
    CreateGroupAliasResponse, ListGroupAliasesByIdResponse, ReadGroupAliasByIdResponse,
};
use rustify_derive::Endpoint;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// ## Create a group alias
///
/// This endpoint creates or updates a group alias.
///
/// * Path: identity/group-alias
/// * Method: POST
/// * Response: [CreateGroupAliasResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/group-alias#create-a-group-alias>
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
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/group-alias#update-group-alias-by-id>
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
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/group-alias#read-group-alias-by-id>
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
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/group-alias#delete-group-alias-by-id>
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
/// * Response: [ListGroupAliasesByIdResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/group-alias#list-group-alias-by-id>
#[derive(Builder, Debug, Endpoint, Default)]
#[endpoint(
    path = "identity/group-alias/id",
    method = "LIST",
    builder = "true",
    response = "ListGroupAliasesByIdResponse"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListGroupAliasesByIdRequest {}
