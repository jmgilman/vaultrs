use crate::{
    api::{
        self,
        auth::approle::{
            requests::LoginWithApproleRequest,
        },
        AuthInfo,
    },
    client::VaultClient,
    error::ClientError,
};

// Fetch a token with policies in corresponding AppRole.
//
// See [LoginWithApproleRequest]
pub async fn login(
    client: &VaultClient,
    mount: &str,
    role_id: &str,
    secret_id: &str,
) -> Result<AuthInfo, ClientError> {
    let endpoint = LoginWithApproleRequest::builder()
        .mount(mount)
        .role_id(role_id)
        .secret_id(secret_id)
        .build()
        .unwrap();
    api::auth(client, endpoint).await
}

pub mod role {
    use crate::api;
    use crate::api::auth::approle::{
        requests::{
            ListRolesRequest, ReadAppRoleRequest, SetAppRoleRequest, DeleteAppRoleRequest,
            ReadRoleIDRequest, GenerateNewSecretIDRequest,
            SetAppRoleRequestBuilder, GenerateNewSecretIDRequestBuilder
        },
        responses::{
            ListRolesResponse, ReadAppRoleResponse, ReadRoleIDResponse,
            GenerateNewSecretIDResponse
        },
    };
    use crate::client::VaultClient;
    use crate::error::ClientError;

    /// Lists all AppRoles
    ///
    /// See [ListRolesRequest]
    pub async fn list(client: &VaultClient, mount: &str) -> Result<ListRolesResponse, ClientError> {
        let endpoint = ListRolesRequest::builder().mount(mount).build().unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Reads properties of an AppRole
    ///
    /// See [ReadAppRoleRequest]
    pub async fn read(
        client: &VaultClient,
        mount: &str,
        role_name: &str,
    ) -> Result<ReadAppRoleResponse, ClientError> {
        let endpoint = ReadAppRoleRequest::builder()
            .mount(mount)
            .role_name(role_name)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Creates or updates an AppRole
    ///
    /// See [SetAppRoleRequest]
    pub async fn set(
        client: &VaultClient,
        mount: &str,
        role_name: &str,
        opts: Option<&mut SetAppRoleRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = SetAppRoleRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut t)
            .mount(mount)
            .role_name(role_name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Deletes an existing AppRole
    ///
    /// See [DeleteAppRoleRequest]
    pub async fn delete(
        client: &VaultClient,
        mount: &str,
        role_name: &str,
    ) -> Result<(), ClientError> {
        let endpoint = DeleteAppRoleRequest::builder()
            .mount(mount)
            .role_name(role_name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Reads the RoleID of an existing AppRole
    ///
    /// See [DeleteAppRoleRequest]
    pub async fn read_role_id(
        client: &VaultClient,
        mount: &str,
        role_name: &str,
    ) -> Result<ReadRoleIDResponse, ClientError> {
        let endpoint = ReadRoleIDRequest::builder()
            .mount(mount)
            .role_name(role_name)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Generates and issues a new SecretID on an existing AppRole
    ///
    /// See [GenerateNewSecretIDRequest]
    pub async fn generate_secret_id(
        client: &VaultClient,
        mount: &str,
        role_name: &str,
        opts: Option<&mut GenerateNewSecretIDRequestBuilder>,
    ) -> Result<GenerateNewSecretIDResponse, ClientError> {
        let mut t = GenerateNewSecretIDRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut t)
            .mount(mount)
            .role_name(role_name)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }
}
