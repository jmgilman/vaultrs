use super::responses::{
    GenerateCredentialsResponse, GetStaticCredentialsResponse, ListConnectionsResponse,
    ListRolesResponse, ListStaticRolesResponse, ReadConnectionResponse, ReadRoleResponse,
    ReadStaticRoleResponse,
};
use rustify_derive::Endpoint;
use std::fmt::Debug;

/// ## Configure Connection
/// This endpoint configures the connection string used to communicate with the
/// desired database.
/// * Path: {self.mount}/config/{self.name}
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docssecret/databases#configure-connection>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/config/{self.name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct PostgreSQLConnectionRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
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
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/databases#read-connection>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/config/{self.name}",
    response = "ReadConnectionResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadConnectionRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}

/// ## List Connections
/// This endpoint returns a list of available connections.
///
/// * Path: {self.mount}/config
/// * Method: LIST
/// * Response: [ListConnectionsResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/databases#list-connections>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/config",
    method = "LIST",
    response = "ListConnectionsResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListConnectionsRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Delete Connection
/// This endpoint deletes a connection.
///
/// * Path: {self.mount}/config/{self.name}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/databases#delete-connection>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/config/{self.name}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteConnectionRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}

/// ## Reset Connection
/// This endpoint closes a connection and it's underlying plugin and restarts it
/// with the configuration stored in the barrier.
///
/// * Path: {self.mount}/reset/{self.name}
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/databases#reset-connection>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/reset/{self.name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ResetConnectionRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}

/// ## Rotate Root Credentials
/// This endpoint is used to rotate the "root" user credentials stored for the
/// database connection.
///
/// * Path: {self.mount}/rotate-root/{self.name}
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/databases#rotate-root-credentials>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/rotate-root/{self.name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct RotateRootRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}

/// ## Create Role
/// This endpoint creates or updates a role definition.
///
/// * Path: {self.mount}/roles/{self.name}
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/databases#create-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/roles/{self.name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct SetRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
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
/// * Path: {self.mount}/roles/{self.name}
/// * Method: GET
/// * Response: [ReadRoleResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/databases#read-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/roles/{self.name}",
    response = "ReadRoleResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}

/// ## List Roles
/// This endpoint returns a list of available roles.
///
/// * Path: {self.mount}/roles
/// * Method: LIST
/// * Response: [ListRolesResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/databases#list-roles>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/roles",
    method = "LIST",
    response = "ListRolesResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListRolesRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Delete Role
/// This endpoint deletes the role definition.
///
/// * Path: {self.mount}/roles/{self.name}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/databases#delete-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/roles/{self.name}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}

/// ## Generate Credentials
/// This endpoint generates a new set of dynamic credentials based on the named
/// role.
///
/// * Path: {self.mount}/creds/{self.name}
/// * Method: GET
/// * Response: [GenerateCredentialsResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/database#generate-credentials>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/creds/{self.name}",
    response = "GenerateCredentialsResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct GenerateCredentialsRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}

/// ## Create Static Role
/// This endpoint creates or updates a static role definition.
///
/// * Path: {self.mount}/static-oles/{self.name}
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/databases#create-static-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/static-roles/{self.name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct SetStaticRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
    pub db_name: String,
    pub username: String,
    pub rotation_period: String,
    pub rotation_statements: Option<Vec<String>>,
}

/// ## Read Static Role
/// This endpoint queries the static role definition.
///
/// * Path: {self.mount}/static-roles/{self.name}
/// * Method: GET
/// * Response: [ReadStaticRoleResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/databases#read-static-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/static-roles/{self.name}",
    response = "ReadStaticRoleResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadStaticRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}

/// ## List Static Roles
/// This endpoint returns a list of available static roles.
///
/// * Path: {self.mount}/static-roles
/// * Method: LIST
/// * Response: [ListStaticRolesResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/databases#list-static-roles>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/static-roles",
    method = "LIST",
    response = "ListStaticRolesResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListStaticRolesRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Delete Static Role
/// This endpoint deletes the static role definition.
///
/// * Path: {self.mount}/static-roles/{self.name}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/databases#delete-static-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/static-roles/{self.name}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteStaticRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}

/// ## Get Static Credentials
/// This endpoint returns the current credentials based on the named static role.
///
/// * Path: {self.mount}/static-creds/{self.name}
/// * Method: GET
/// * Response: [GetStaticCredentialsResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/databases#get-static-credentials>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/static-creds/{self.name}",
    response = "GetStaticCredentialsResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct GetStaticCredentialsRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}

/// ## Rotate Static Role Credentials
/// This endpoint is used to rotate the Static Role credentials stored for a
/// given role name.
///
/// * Path: {self.mount}/rotate-role/{self.name}
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/databases#rotate-static-role-credentials>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/rotate-role/{self.name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct RotateStaticRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}
