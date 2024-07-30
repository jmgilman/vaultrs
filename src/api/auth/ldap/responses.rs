use serde::{Deserialize, Serialize};

/// Response from executing
/// [ListLDAPGroupsRequest][crate::api::auth::ldap::requests::ListLDAPGroupsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListLDAPGroupsResponse {
    pub keys: Vec<String>,
}

/// Response from executing
/// [ReadLDAPGroupRequest][crate::api::auth::ldap::requests::ReadLDAPGroupRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadLDAPGroupResponse {
    pub policies: Vec<String>,
}

/// Response from executing
/// [ListLDAPUsersRequest][crate::api::auth::ldap::requests::ListLDAPUsersRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListLDAPUsersResponse {
    pub keys: Vec<String>,
}

/// Response from executing
/// [ReadLDAPUserRequest][crate::api::auth::ldap::requests::ReadLDAPUserRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadLDAPUserResponse {
    pub policies: Vec<String>,
    pub groups: String,
}
