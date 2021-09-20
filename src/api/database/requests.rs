use super::responses::{
    GenerateCredentialsResponse, GetStaticCredentialsResponse, ListConnectionsResponse,
    ListRolesResponse, ListStaticRolesResponse, ReadConnectionResponse, ReadRoleResponse,
    ReadStaticRoleResponse,
};
use rustify_derive::Endpoint;
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::fmt::Debug;

/// ## Configure Connection
/// This endpoint configures the connection string used to communicate with the
/// desired database.
/// * Path: {self.mount}/config/{self.name}
/// * Method: POST
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api/secret/databases#configure-connection
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/config/{self.name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct PostgreSQLConnectionRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub name: String,
    pub connection_url: String,
    pub plugin_name: String,
    pub allowed_roles: Option<Vec<String>>, // Default parameters
    pub password_policy: Option<String>,
    pub root_rotation_statements: Option<Vec<String>>,
    pub verify_connection: Option<bool>,
    pub max_connection_lifetime: Option<String>, // PostgresSQL specific parameters
    pub max_idle_connections: Option<u64>,
    pub max_open_connections: Option<u64>,
    pub password: Option<String>,
    pub username: Option<String>,
    pub username_template: Option<String>,
}

/// ## Read Connection
/// This endpoint returns the configuration settings for a connection.
///
/// * Path: {self.mount}/config/{self.name}
/// * Method: GET
/// * Response: [ReadConnectionResponse]]
/// * Reference: https://www.vaultproject.io/api-docs/secret/databases#read-connection
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/config/{self.name}",
    response = "ReadConnectionResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadConnectionRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub name: String,
}

/// ## List Connections
/// This endpoint returns a list of available connections.
///
/// * Path: {self.mount}/config
/// * Method: LIST
/// * Response: [ListConnectionsResponse]
/// * Reference: https://www.vaultproject.io/api-docs/secret/databases#list-connections
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/config",
    method = "LIST",
    response = "ListConnectionsResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListConnectionsRequest {
    #[serde(skip)]
    pub mount: String,
}

/// ## Delete Connection
/// This endpoint deletes a connection.
///
/// * Path: {{self.mount}/config/{self.name}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/secret/databases#delete-connection
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/config/{self.name}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteConnectionRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub name: String,
}

/// ## Reset Connection
/// This endpoint closes a connection and it's underlying plugin and restarts it
/// with the configuration stored in the barrier.
///
/// * Path: {{self.mount}/reset/{self.name}
/// * Method: POST
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/secret/databases#reset-connection
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/reset/{self.name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ResetConnectionRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub name: String,
}

/// ## Rotate Root Credentials
/// This endpoint is used to rotate the "root" user credentials stored for the
/// database connection.
///
/// * Path: {{self.mount}/rotate-root/{self.name}
/// * Method: POST
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/secret/databases#rotate-root-credentials
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/rotate-root/{self.name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct RotateRootRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub name: String,
}

/// ## Create Role
/// This endpoint creates or updates a role definition.
///
/// * Path: {{self.mount}/roles/{self.name}
/// * Method: POST
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/secret/databases#create-role
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/roles/{self.name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct CreateRoleRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub name: String,
    pub creation_statements: Option<Vec<String>>,
    pub db_name: String,
    pub default_ttl: Option<String>,
    pub max_ttl: Option<String>,
    pub renew_statements: Option<Vec<String>>,
    pub revocation_statements: Option<Vec<String>>,
    pub rollback_statements: Option<Vec<String>>,
}

/// ## Read Role
/// This endpoint queries the role definition.
///
/// * Path: {{self.mount}/roles/{self.name}
/// * Method: GET
/// * Response: [ReadRoleResponse]
/// * Reference: https://www.vaultproject.io/api-docs/secret/databases#read-role
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/roles/{self.name}",
    response = "ReadRoleResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadRoleRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub name: String,
}

/// ## List Roles
/// This endpoint returns a list of available roles.
///
/// * Path: {self.mount}/roles
/// * Method: LIST
/// * Response: [ListRolesResponse]
/// * Reference: https://www.vaultproject.io/api-docs/secret/databases#list-roles
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/roles",
    method = "LIST",
    response = "ListRolesResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListRolesRequest {
    #[serde(skip)]
    pub mount: String,
}

/// ## Delete Role
/// This endpoint deletes the role definition.
///
/// * Path: {{self.mount}/roles/{self.name}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/secret/databases#delete-role
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/roles/{self.name}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteRoleRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub name: String,
}

/// ## Generate Credentials
/// This endpoint generates a new set of dynamic credentials based on the named
/// role.
///
/// * Path: {{self.mount}/creds/{self.name}
/// * Method: GET
/// * Response: [GenerateCredentialsResponse]
/// * Reference: https://www.vaultproject.io/api-docs/secret/databases#read-role
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/creds/{self.name}",
    response = "GenerateCredentialsResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct GenerateCredentialsRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub name: String,
}

/// ## Create Static Role
/// This endpoint creates or updates a static role definition.
///
/// * Path: {{self.mount}/static-oles/{self.name}
/// * Method: POST
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/secret/databases#create-static-role
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/static-roles/{self.name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct CreateStaticRoleRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub name: String,
    pub db_name: String,
    pub username: String,
    pub rotation_period: Option<String>,
    pub rotation_statements: Option<Vec<String>>,
}

/// ## Read Static Role
/// This endpoint queries the static role definition.
///
/// * Path: {{self.mount}/static-roles/{self.name}
/// * Method: GET
/// * Response: [ReadStaticRoleResponse]
/// * Reference: https://www.vaultproject.io/api-docs/secret/databases#read-static-role
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/static-roles/{self.name}",
    response = "ReadStaticRoleResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadStaticRoleRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub name: String,
}

/// ## List Static Roles
/// This endpoint returns a list of available static roles.
///
/// * Path: {self.mount}/static-roles
/// * Method: LIST
/// * Response: [ListStaticRolesResponse]
/// * Reference: https://www.vaultproject.io/api-docs/secret/databases#list-static-roles
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/static-roles",
    method = "LIST",
    response = "ListStaticRolesResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListStaticRolesRequest {
    #[serde(skip)]
    pub mount: String,
}

/// ## Delete Static Role
/// This endpoint deletes the static role definition.
///
/// * Path: {{self.mount}/static-roles/{self.name}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/secret/databases#delete-static-role
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/static-roles/{self.name}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteStaticRoleRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub name: String,
}

/// ## Get Static Credentials
/// This endpoint returns the current credentials based on the named static role.
///
/// * Path: {self.mount}/static-creds/{self.name}
/// * Method: GET
/// * Response: [GetStaticCredentialsResponse]
/// * Reference: https://www.vaultproject.io/api-docs/secret/databases#get-static-credentials
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/static-creds/{self.name}",
    response = "GetStaticCredentialsResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct GetStaticCredentialsRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub name: String,
}

/// ## Rotate Static Role Credentials
/// This endpoint is used to rotate the Static Role credentials stored for a
/// given role name.
///
/// * Path: {{self.mount}/rotate-role/{self.name}
/// * Method: POST
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/secret/databases#rotate-static-role-credentials
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/rotate-role/{self.name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct RotateStaticRoleRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub name: String,
}
