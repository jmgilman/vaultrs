use super::responses::{
    ListRolesResponse, ReadAppRoleResponse, ReadRoleIDResponse,
    GenerateNewSecretIDResponse,
};
use rustify_derive::Endpoint;
use serde::Serialize;
use serde_with::skip_serializing_none;

/// ## Login with Approle
/// Issues a Vault token based on the presented credentials.
///
/// * Path: /auth/approle/login
/// * Method: POST
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/auth/approle#login-with-approle
#[skip_serializing_none]
#[derive(Builder, Debug, Endpoint, Serialize)]
#[endpoint(path = "/auth/{self.mount}/login", method = "POST", builder = "true")]
#[builder(setter(into))]
pub struct LoginWithApproleRequest {
    #[serde(skip)]
    pub mount: String,
    pub role_id: String,
    pub secret_id: String,
}

/// ## List Roles
/// This endpoint returns a list the existing AppRoles in the method.
///
/// * Path: /auth/{self.mount}/role
/// * Method: LIST
/// * Response: [ListRoleResponse]
/// * Reference: https://www.vaultproject.io/api-docs/auth/approle#list-roles
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "/auth/{self.mount}/role",
    method = "LIST",
    response = "ListRolesResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListRolesRequest {
    #[serde(skip)]
    pub mount: String,
}

/// ## Create/Update AppRole
/// Creates a new AppRole or updates an existing AppRole.
///
/// * Path: /auth/{self.mount}/role/{self.role_name}
/// * Method: POST
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/auth/approle#create-update-approle
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "/auth/{self.mount}/role/{self.role_name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct SetAppRoleRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub role_name: String,
    pub bind_secret_id: Option<bool>,
    pub secret_id_bound_cidrs: Option<Vec<String>>,
    pub secret_id_num_uses: Option<u64>,
    pub secret_id_ttl: Option<String>,
    pub enable_local_secret_ids: Option<bool>,
    pub token_ttl: Option<String>,
    pub token_max_ttl: Option<String>,
    pub token_policies: Option<Vec<String>>,
    pub token_bound_cidrs: Option<Vec<String>>,
    pub token_explicit_max_ttl: Option<String>,
    pub token_no_default_policy: Option<bool>,
    pub token_num_uses: Option<u64>,
    pub token_period: Option<String>,
    pub token_type: Option<String>,
}

/// ## Read AppRole
/// Reads the properties of an existing AppRole.
///
/// * Path: /auth/{self.mount}/role/{self.role_name}
/// * Method: GET
/// * Response: [ReadAppRoleResponse]
/// * Reference: https://www.vaultproject.io/api-docs/auth/approle#read-approle
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "/auth/{self.mount}/role/{self.role_name}",
    response = "ReadAppRoleResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadAppRoleRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub role_name: String,
}

/// ## Delete AppRole
/// Deletes an existing AppRole.
///
/// * Path: /auth/{self.mount}/role/{self.role_name}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/auth/approle#delete-approle
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "/auth/{self.mount}/role/{self.role_name}",
    method = "DELETE",
    builder = "true",
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteAppRoleRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub role_name: String,
}

/// ## Read AppRole RoleID
/// Reads the RoleID of an existing AppRole.
///
/// * Path: /auth/{self.mount}/role/{self.role_name}/role-id
/// * Method: GET
/// * Response: [ReadRoleIDResponse]
/// * Reference: https://www.vaultproject.io/api-docs/auth/approle#read-approle-role-id
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "/auth/{self.mount}/role/{self.role_name}/role-id",
    response = "ReadRoleIDResponse",
    builder = "true",
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadRoleIDRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub role_name: String,
}

/// ## Generate New Secret ID
/// Generates and issues a new SecretID on an existing AppRole.
///
/// * Path: /auth/{self.mount}/role/{self.role_name}/secret-id
/// * Method: POST
/// * Response: [GenerateNewSecretIDResponse]
/// * Reference: https://www.vaultproject.io/api-docs/auth/approle#generate-new-secret-id
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "/auth/{self.mount}/role/{self.role_name}/secret-id",
    method = "POST",
    response = "GenerateNewSecretIDResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct GenerateNewSecretIDRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub role_name: String,
    pub metadata: Option<String>,
    pub cidr_list: Option<Vec<String>>,
    pub token_bound_cidrs: Option<Vec<String>>,
}
