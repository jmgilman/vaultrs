use super::responses::{
    ListLDAPGroupsResponse, ListLDAPUsersResponse, ReadLDAPGroupResponse, ReadLDAPUserResponse,
};
use rustify_derive::Endpoint;

/// ## Configure Client
/// Configures LDAP endpoint.
///
/// * Path: /auth/{self.mount}/config
/// * Method: POST
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/auth/ldap#configure-ldap
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "/auth/{self.mount}/config", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct ConfigureClientRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub url: Option<String>,
    pub case_sensitive_names: Option<bool>,
    pub request_timeout: Option<String>,
    pub starttls: Option<bool>,
    pub tls_min_version: Option<String>,
    pub tls_max_version: Option<String>,
    pub insecure_tls: Option<bool>,
    pub certificate: Option<String>,
    pub client_tls_cert: Option<String>,
    pub client_tls_key: Option<String>,
    pub binddn: Option<String>,
    pub bindpass: Option<String>,
    pub userdn: Option<String>,
    pub userattr: Option<String>,
    pub discoverdn: Option<bool>,
    pub deny_null_bind: Option<bool>,
    pub upndomain: Option<String>,
    pub userfilter: Option<String>,
    pub anonymous_group_search: Option<bool>,
    pub groupfilter: Option<String>,
    pub groupdn: Option<String>,
    pub groupattr: Option<String>,
    pub username_as_alias: Option<bool>,
    pub token_ttl: Option<String>,
    pub token_max_ttl: Option<String>,
    pub token_policies: Option<String>,
    pub token_bound_cidrs: Option<String>,
    pub token_explicit_max_ttl: Option<String>,
    pub token_no_default_policy: Option<bool>,
    pub token_num_uses: Option<i64>,
    pub token_period: Option<String>,
    pub token_type: Option<String>,
}

/// ## List LDAP groups
/// Returns a list of existing groups.
///
/// * Path: /auth/{self.mount}/groups
/// * Method: LIST
/// * Response: [ListLDAPGroupsResponse]
/// * Reference: https://www.vaultproject.io/api-docs/auth/ldap#list-ldap-groups
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/groups",
    method = "LIST",
    response = "ListLDAPGroupsResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListLDAPGroupsRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Read LDAP Group
/// Reads the policies associated with a LDAP group.
///
/// * Path: /auth/{self.mount}/groups/{self.groupname}
/// * Method: GET
/// * Response: [ReadLDAPGroupResponse]
/// * Reference: https://www.vaultproject.io/api-docs/auth/ldap#read-ldap-group
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/groups/{self.groupname}",
    response = "ReadLDAPGroupResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadLDAPGroupRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub groupname: String,
}

/// ## Create/Update LDAP Group
/// Creates or updates LDAP group policies.
///
/// * Path: /auth/{self.mount}/users/{self.groupname}
/// * Method: POST
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/auth/ldap#create-update-ldap-group
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/groups/{self.groupname}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct CreateLDAPGroupRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub groupname: String,
    pub policies: String,
}

/// ## Delete LDAP Group
/// Deletes the LDAP group and policy association.
///
/// * Path: /auth/{self.mount}/groups/{self.groupname}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/auth/ldap#delete-ldap-group
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/groups/{self.groupname}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteLDAPGroupRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub groupname: String,
}

/// ## Create/Update LDAP User
/// This endpoint creates or updates LDAP users policies and group associations.
///
/// * Path: /auth/{self.mount}/users/{self.username}
/// * Method: POST
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/auth/ldap#create-update-ldap-user
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/users/{self.username}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct CreateLDAPUserRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub username: String,
    pub policies: String,
    pub groups: String,
}

/// ## Read LDAP User
/// Reads the properties of an existing username.
///
/// * Path: /auth/{self.mount}/users/{self.username}
/// * Method: GET
/// * Response: [ReadUserResponse]
/// * Reference: https://www.vaultproject.io/api-docs/auth/ldap#read-ldap-user
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/users/{self.username}",
    response = "ReadLDAPUserResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadLDAPUserRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub username: String,
}

/// ## List Users
/// List available LDAP users.
///
/// * Path: /auth/{self.mount}/users
/// * Method: LIST
/// * Response: [ListLDAPUsersResponse]
/// * Reference: https://www.vaultproject.io/api-docs/auth/ldap#list-ldap-users
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/users",
    method = "LIST",
    response = "ListLDAPUsersResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListLDAPUsersRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Delete User
/// This endpoint deletes the LDAP user and policy association.
///
/// * Path: /auth/{self.mount}/users/{self.username}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/auth/ldap#delete-ldap-user
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/users/{self.username}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteLDAPUserRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub username: String,
}

/// ## Login
/// Login with the username and password.
///
/// * Path: /auth/{self.mount}/login/{self.username}
/// * Method: POST
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/auth/ldap#login-with-ldap-user
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/login/{self.username}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct LoginRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub username: String,
    pub password: String,
}
