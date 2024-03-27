use crate::{
    api::{self, auth::userpass::requests::LoginRequest, AuthInfo},
    client::Client,
    error::ClientError,
};

// Fetch a token with policies corresponding to the username.
//
// See [LoginRequest]
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
            auth::userpass::{
                requests::{
                    CreateUserRequest, CreateUserRequestBuilder, DeleteUserRequest,
                    ListUsersRequest, ReadUserRequest, UpdatePasswordRequest,
                    UpdatePoliciesRequest,
                },
                responses::{ListUsersResponse, ReadUserResponse},
            },
        },
        client::Client,
        error::ClientError,
    };

    /// Deletes a user.
    ///
    /// See [DeleteUserRequest]
    pub async fn delete(
        client: &impl Client,
        mount: &str,
        username: &str,
    ) -> Result<(), ClientError> {
        let endpoint = DeleteUserRequest::builder()
            .mount(mount)
            .username(username)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Lists users.
    ///
    /// See [ListUsersRequest]
    pub async fn list(client: &impl Client, mount: &str) -> Result<ListUsersResponse, ClientError> {
        let endpoint = ListUsersRequest::builder().mount(mount).build().unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Reads information about a user.
    ///
    /// See [ReadUserRequest]
    pub async fn read(
        client: &impl Client,
        mount: &str,
        username: &str,
    ) -> Result<ReadUserResponse, ClientError> {
        let endpoint = ReadUserRequest::builder()
            .mount(mount)
            .username(username)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Crates or updates a new user.
    ///
    /// See [CreateUserRequest]
    pub async fn set(
        client: &impl Client,
        mount: &str,
        username: &str,
        password: &str,
        opts: Option<&mut CreateUserRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = CreateUserRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut t)
            .mount(mount)
            .username(username)
            .password(password)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Updates a user's password.
    ///
    /// See [UpdatePasswordRequest]
    pub async fn update_password(
        client: &impl Client,
        mount: &str,
        username: &str,
        password: &str,
    ) -> Result<(), ClientError> {
        let endpoint = UpdatePasswordRequest::builder()
            .mount(mount)
            .username(username)
            .password(password)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Updates a user's policies.
    ///
    /// See [UpdatePoliciesRequest]
    pub async fn update_policies(
        client: &impl Client,
        mount: &str,
        username: &str,
        policies: &str,
    ) -> Result<(), ClientError> {
        let endpoint = UpdatePoliciesRequest::builder()
            .mount(mount)
            .username(username)
            .policies(policies)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }
}
