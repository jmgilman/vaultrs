use super::responses::{ListUsersResponse, ReadUserResponse};
use rustify_derive::Endpoint;
use serde::Serialize;
use serde_with::skip_serializing_none;

/// ## Create/Update User
/// Create a new user or update an existing user.
///
/// * Path: /auth/{self.mount}/users/{self.username}
/// * Method: POST
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/auth/userpass#create-update-user
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "/auth/{self.mount}/users/{self.username}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct CreateUserRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub username: String,
    pub password: String,
    pub token_bound_cidrs: Option<Vec<String>>,
    pub token_explicit_max_ttl: Option<String>,
    pub token_no_default_policy: Option<bool>,
    pub token_num_uses: Option<u64>,
    pub token_ttl: Option<String>,
    pub token_max_ttl: Option<String>,
    pub token_period: Option<String>,
    pub token_policies: Option<Vec<String>>,
    pub token_type: Option<String>,
}

/// ## Read User
/// Reads the properties of an existing username.
///
/// * Path: /auth/{self.mount}/users/{self.username}
/// * Method: GET
/// * Response: [ReadUserResponse]
/// * Reference: https://www.vaultproject.io/api-docs/auth/userpass#read-user
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "/auth/{self.mount}/users/{self.username}",
    response = "ReadUserResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadUserRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub username: String,
}

/// ## Delete User
/// This endpoint deletes the user from the method.
///
/// * Path: /auth/{self.mount}/users/{self.username}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/auth/userpass#delete-user
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "/auth/{self.mount}/users/{self.username}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteUserRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub username: String,
}

/// ## Update Password on User
/// Update password for an existing user.
///
/// * Path: /auth/{self.mount}/users/{self.username}/password
/// * Method: POST
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/auth/userpass#update-password-on-user
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "/auth/{self.mount}/users/{self.username}/password",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct UpdatePasswordRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub username: String,
    pub password: String,
}

/// ## Update Policies on User
/// Update policies for an existing user.
///
/// * Path: /auth/{self.mount}/users/{self.username}/policies
/// * Method: POST
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/auth/userpass#update-policies-on-user
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "/auth/{self.mount}/users/{self.username}/policies",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct UpdatePoliciesRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub username: String,
    pub policies: String,
}

/// ## List Users
/// List available userpass users.
///
/// * Path: /auth/{self.mount}/users
/// * Method: LIST
/// * Response: [ListUsersResponse]
/// * Reference: https://www.vaultproject.io/api-docs/auth/userpass#list-users
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "/auth/{self.mount}/users",
    method = "LIST",
    response = "ListUsersResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListUsersRequest {
    #[serde(skip)]
    pub mount: String,
}

/// ## Login
/// Login with the username and password.
///
/// * Path: /auth/{self.mount}/login/{self.username}
/// * Method: POST
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/auth/userpass#login
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "/auth/{self.mount}/login/{self.username}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct LoginRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub username: String,
    pub password: String,
}
