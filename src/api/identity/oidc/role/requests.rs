use crate::api::identity::oidc::role::responses::ReadRoleResponse;
use rustify_derive::Endpoint;

/// ## Create or update a role.
///
/// This endpoint creates or updates a role.
///
/// * Path: identity/oidc/role/{name}
/// * Method: POST
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/tokens#create-or-update-a-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "identity/oidc/role/{self.name}",
    method = "POST",
    builder = "true",
)]
#[builder(setter(into, strip_option), default)]
pub struct SetRoleRequest {
    /// Name of the role.
    #[endpoint(skip)]
    pub name: String,
    /// A configured named key, the key must already exist.
    pub key: String,
    /// The template string to use for generating tokens. This may be in string-ified JSON or base64 format.
    pub template: Option<String>,
    /// Optional client ID. A random ID will be generated if left unset.
    pub client_id: Option<String>,
    /// TTL of the tokens generated against the role. Uses duration format strings.
    pub ttl: Option<String>,
}

/// ## Read a role.
///
/// This endpoint reads a role.
///
/// * Path: identity/oidc/role/{name}
/// * Method: GET
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/tokens#read-a-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "identity/oidc/role/{self.name}",
    method = "GET",
    builder = "true",
    response = "ReadRoleResponse"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadRoleRequest {
    /// Name of the role.
    #[endpoint(skip)]
    pub name: String,
}

/// ## Delete a role.
///
/// This endpoint deletes a role.
///
/// * Path: identity/oidc/role/{name}
/// * Method: DELETE
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/tokens#delete-a-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "identity/oidc/role/{self.name}",
    method = "DELETE",
    builder = "true",
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteRoleRequest {
    /// Name of the role.
    #[endpoint(skip)]
    pub name: String,
}