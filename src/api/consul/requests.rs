use super::responses::{GenerateConsulCredsResponse, ListRolesResponse, ReadRoleResponse};
use rustify_derive::Endpoint;

/// ## Create/Update access Config
/// This endpoint creates or updates a consul secret engines access configuration.
///
/// * Path: {self.mount}/config/access
/// * Method: POST
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/secret/consul#configure-access
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "{self.mount}/config/access", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct SetAccessConfigRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub address: String,
    pub schema: Option<String>,
    pub token: Option<String>,
    pub ca_cert: Option<String>,
    pub client_cert: Option<String>,
    pub client_key: Option<String>,
}

/// ## Create Role
/// This endpoint creates or updates a named role.
///
/// * Path: {self.mount}/roles/{self.name}
/// * Method: POST
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/secret/consul#configure-access
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
    pub name: String,
    pub token_type: Option<String>, // DEPRECATED since consul version 1.4 and removed in 1.11
    pub partition: Option<String>,
    pub node_identities: Option<Vec<String>>,
    pub consul_namespace: Option<String>,
    pub service_identities: Option<Vec<String>>,
    pub consul_roles: Option<Vec<String>>,
    pub consul_policies: Option<Vec<String>>,
    pub policy: Option<String>, // DEPRECATED since consul version 1.4 and removed in 1.11
    pub policies: Option<Vec<String>>, // DEPRECATED since consul version 1.4 and removed in 1.11
    pub local: Option<bool>,
    pub max_ttl: Option<String>,
    pub ttl: Option<String>,
}

/// ## Read Role
/// This endpoint queries a named role.
///
/// * Path: {self.mount}/roles/{self.name}
/// * Method: GET
/// * Response: [ReadRoleResponse]
/// * Reference: https://www.vaultproject.io/api-docs/secret/consul#read-role
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
/// * Reference: https://www.vaultproject.io/api-docs/secret/consul#list-roles
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
/// This endpoint deletes a named role.
///
/// * Path: {self.mount}/roles/{self.name}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/secret/consul#delete-role
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

/// ## Generate Consul Credentials
/// This endpoint creates credentials with the parameters defined in the given role.
///
/// * Path: {self.mount}/creds/{self.name}
/// * Method: POST
/// * Response: [GenerateConsulCredsResponse]
/// * Reference: https://www.vaultproject.io/api-docs/secret/consul#generate-credential
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/creds/{self.name}",
    method = "POST",
    response = "GenerateConsulCredsResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct GenerateConsulCredsRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}
