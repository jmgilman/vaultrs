use crate::{
    api::{self, auth::ldap::requests::LoginRequest, AuthInfo},
    client::Client,
    error::ClientError,
};

// Fetch a token with policies corresponding to the username.
//
// See [LoginRequest]
#[instrument(skip(client, password), err)]
pub async fn login(
    client: &impl Client,
    mount: &str,
    username: &str,
    password: &str,
) -> Result<AuthInfo, ClientError> {
    let endpoint = LoginRequest::builder()
        .mount(mount)
        .username(username)
        .password(password)
        .build()
        .unwrap();
    api::auth(client, endpoint).await
}

pub mod user {
    use crate::{
        api::{
            self,
            auth::ldap::{
                requests::{
                    CreateLDAPUserRequest, CreateLDAPUserRequestBuilder, DeleteLDAPUserRequest,
                    ListLDAPUsersRequest, ReadLDAPUserRequest,
                },
                responses::{ListLDAPUsersResponse, ReadLDAPUserResponse},
            },
        },
        client::Client,
        error::ClientError,
    };

    /// Deletes a user.
    ///
    /// See [DeleteLDAPUserRequest]
    #[instrument(skip(client), err)]
    pub async fn delete(
        client: &impl Client,
        mount: &str,
        username: &str,
    ) -> Result<(), ClientError> {
        let endpoint = DeleteLDAPUserRequest::builder()
            .mount(mount)
            .username(username)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Lists users.
    ///
    /// See [ListLDAPUsersRequest]
    #[instrument(skip(client), err)]
    pub async fn list(
        client: &impl Client,
        mount: &str,
    ) -> Result<ListLDAPUsersResponse, ClientError> {
        let endpoint = ListLDAPUsersRequest::builder()
            .mount(mount)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Reads information about a user.
    ///
    /// See [ReadLDAPUserRequest]
    #[instrument(skip(client), err)]
    pub async fn read(
        client: &impl Client,
        mount: &str,
        username: &str,
    ) -> Result<ReadLDAPUserResponse, ClientError> {
        let endpoint = ReadLDAPUserRequest::builder()
            .mount(mount)
            .username(username)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Crates or updates a new user.
    ///
    /// See [CreateLDAPUserRequest]
    #[instrument(skip(client, opts), err)]
    pub async fn set(
        client: &impl Client,
        mount: &str,
        username: &str,
        policies: &str,
        groups: &str,
        opts: Option<&mut CreateLDAPUserRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = CreateLDAPUserRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut t)
            .mount(mount)
            .username(username)
            .policies(policies)
            .groups(groups)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Updates a user's groups.
    ///
    /// See [CreateLDAPUserRequest]
    #[instrument(skip(client), err)]
    pub async fn update_groups(
        client: &impl Client,
        mount: &str,
        username: &str,
        groups: &str,
    ) -> Result<(), ClientError> {
        let endpoint = CreateLDAPUserRequest::builder()
            .mount(mount)
            .username(username)
            .groups(groups)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Updates a user's policies.
    ///
    /// See [CreateLDAPUserRequest]
    #[instrument(skip(client), err)]
    pub async fn update_policies(
        client: &impl Client,
        mount: &str,
        username: &str,
        policies: &str,
    ) -> Result<(), ClientError> {
        let endpoint = CreateLDAPUserRequest::builder()
            .mount(mount)
            .username(username)
            .policies(policies)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }
}

pub mod group {
    use crate::{
        api::{
            self,
            auth::ldap::{
                requests::{
                    CreateLDAPGroupRequest, CreateLDAPGroupRequestBuilder, DeleteLDAPGroupRequest,
                    ListLDAPGroupsRequest, ReadLDAPGroupRequest,
                },
                responses::{ListLDAPGroupsResponse, ReadLDAPGroupResponse},
            },
        },
        client::Client,
        error::ClientError,
    };

    /// Deletes a user.
    ///
    /// See [DeleteLDAPGroupRequest]
    #[instrument(skip(client), err)]
    pub async fn delete(
        client: &impl Client,
        mount: &str,
        groupname: &str,
    ) -> Result<(), ClientError> {
        let endpoint = DeleteLDAPGroupRequest::builder()
            .mount(mount)
            .groupname(groupname)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Lists users.
    ///
    /// See [ListLDAPGroupsRequest]
    #[instrument(skip(client), err)]
    pub async fn list(
        client: &impl Client,
        mount: &str,
    ) -> Result<ListLDAPGroupsResponse, ClientError> {
        let endpoint = ListLDAPGroupsRequest::builder()
            .mount(mount)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Reads information about a user.
    ///
    /// See [ReadGroupRequest]
    #[instrument(skip(client), err)]
    pub async fn read(
        client: &impl Client,
        mount: &str,
        groupname: &str,
    ) -> Result<ReadLDAPGroupResponse, ClientError> {
        let endpoint = ReadLDAPGroupRequest::builder()
            .mount(mount)
            .groupname(groupname)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Crates or updates a new user.
    ///
    /// See [CreateLDAPGroupRequest]
    #[instrument(skip(client, opts), err)]
    pub async fn set(
        client: &impl Client,
        mount: &str,
        groupname: &str,
        policies: &str,
        opts: Option<&mut CreateLDAPGroupRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = CreateLDAPGroupRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut t)
            .mount(mount)
            .groupname(groupname)
            .policies(policies)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }
}
