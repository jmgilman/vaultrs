use super::responses::{
    CreateCustomSecretIDResponse, GenerateNewSecretIDResponse, ListRolesResponse,
    ListSecretIDResponse, ReadAppRoleResponse, ReadRoleIDResponse, ReadSecretIDResponse,
};
use rustify_derive::Endpoint;

/// ## Login with Approle
/// Issues a Vault token based on the presented credentials.
///
/// * Path: /auth/approle/login
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/approle#login-with-approle>
#[derive(Builder, Debug, Endpoint)]
#[endpoint(path = "/auth/{self.mount}/login", method = "POST", builder = "true")]
#[builder(setter(into))]
pub struct LoginWithApproleRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub role_id: String,
    pub secret_id: String,
}

/// ## List Roles
/// This endpoint returns a list the existing AppRoles in the method.
///
/// * Path: /auth/{self.mount}/role
/// * Method: LIST
/// * Response: [ListRolesResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/approle#list-roles>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/role",
    method = "LIST",
    response = "ListRolesResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListRolesRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Create/Update AppRole
/// Creates a new AppRole or updates an existing AppRole.
///
/// * Path: /auth/{self.mount}/role/{self.role_name}
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/approle#create-update-approle>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/role/{self.role_name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct SetAppRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
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
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/approle#read-approle>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/role/{self.role_name}",
    response = "ReadAppRoleResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadAppRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub role_name: String,
}

/// ## Delete AppRole
/// Deletes an existing AppRole.
///
/// * Path: /auth/{self.mount}/role/{self.role_name}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/approle#delete-approle>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/role/{self.role_name}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteAppRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub role_name: String,
}

/// ## Read AppRole RoleID
/// Reads the RoleID of an existing AppRole.
///
/// * Path: /auth/{self.mount}/role/{self.role_name}/role-id
/// * Method: GET
/// * Response: [ReadRoleIDResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/approle#read-approle-role-id>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/role/{self.role_name}/role-id",
    response = "ReadRoleIDResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadRoleIDRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub role_name: String,
}

/// ## Update AppRole Role ID
/// Reads the RoleID of an existing AppRole.
///
/// * Path: /auth/{self.mount}/role/{self.role_name}/role-id
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/approle#update-approle-role-id>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/role/{self.role_name}/role-id",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct UpdateRoleIDRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub role_name: String,
    pub role_id: String,
}

/// ## Generate New Secret ID
/// Generates and issues a new SecretID on an existing AppRole.
///
/// * Path: /auth/{self.mount}/role/{self.role_name}/secret-id
/// * Method: POST
/// * Response: [GenerateNewSecretIDResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/approle#generate-new-secret-id>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/role/{self.role_name}/secret-id",
    method = "POST",
    response = "GenerateNewSecretIDResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct GenerateNewSecretIDRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub role_name: String,
    pub metadata: Option<String>,
    pub cidr_list: Option<Vec<String>>,
    pub token_bound_cidrs: Option<Vec<String>>,
}

/// ## List Secret ID Accessors
/// Lists the accessors of all the SecretIDs issued against the AppRole.
///
/// * Path: /auth/{self.mount}/role/{self.role_name}/secret-id
/// * Method: LIST
/// * Response: [ListSecretIDResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/approle#list-secret-id-accessors>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/role/{self.role_name}/secret-id",
    method = "LIST",
    response = "ListSecretIDResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListSecretIDRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub role_name: String,
}

/// ## Read AppRole Secret ID
/// Reads out the properties of a SecretID.
///
/// * Path: /auth/{self.mount}/role/{self.role_name}/secret-id/lookup
/// * Method: POST
/// * Response: [ReadSecretIDResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/approle#read-approle-secret-id>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/role/{self.role_name}/secret-id/lookup",
    method = "POST",
    response = "ReadSecretIDResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadSecretIDRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub role_name: String,
    pub secret_id: String,
}

/// ## Destroy AppRole Secret ID
/// Destroy an AppRole secret ID.
///
/// * Path: /auth/{self.mount}/role/{self.role_name}/secret-id/destroy
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/approle#destroy-approle-secret-id>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/role/{self.role_name}/secret-id/destroy",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteSecretIDRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub role_name: String,
    pub secret_id: String,
}

/// ## Read AppRole Secret ID Accessor
/// Reads out the properties of a SecretID.
///
/// * Path: /auth/{self.mount}/role/{self.role_name}/secret-id-accessor/lookup
/// * Method: POST
/// * Response: [ReadSecretIDResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/approle#read-approle-secret-id-accessor>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/role/{self.role_name}/secret-id-accessor/lookup",
    method = "POST",
    response = "ReadSecretIDResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadSecretIDAccessorRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub role_name: String,
    pub secret_id_accessor: String,
}

/// ## Destroy AppRole Secret ID Accessor
/// Destroy an AppRole secret ID by its accessor.
///
/// * Path: /auth/{self.mount}/role/{self.role_name}/secret-id-accessor/destroy
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/approle#destroy-approle-secret-id-accessor>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/role/{self.role_name}/secret-id-accessor/destroy",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteSecretIDAccessorRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub role_name: String,
    pub secret_id_accessor: String,
}

/// ## Create Custom AppRole Secret ID
/// Assigns a "custom" SecretID against an existing AppRole.
///
/// * Path: /auth/{self.mount}/role/{self.role_name}/custom-secret-id
/// * Method: POST
/// * Response: [CreateCustomSecretIDResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/approle#create-custom-approle-secret-id>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/role/{self.role_name}/custom-secret-id",
    method = "POST",
    response = "CreateCustomSecretIDResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct CreateCustomSecretIDRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub role_name: String,
    pub secret_id: String,
    pub metadata: Option<String>,
    pub cidr_list: Option<Vec<String>>,
    pub token_bound_cidrs: Option<Vec<String>>,
}

/// ## Tidy Tokens
/// Performs some maintenance tasks to clean up invalid entries that may remain
/// in the token store.
///
/// * Path: /auth/{self.mount}/tidy/secret-id
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/approle#tidy-tokens>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/tidy/secret-id",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct TidyRequest {
    #[endpoint(skip)]
    pub mount: String,
}
