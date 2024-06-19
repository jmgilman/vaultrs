use crate::{
    api::{
        self, auth::kubernetes::requests::ConfigureKubernetesAuthRequest,
        auth::kubernetes::requests::ConfigureKubernetesAuthRequestBuilder,
        auth::kubernetes::requests::LoginWithKubernetesRequest,
        auth::kubernetes::requests::ReadKubernetesAuthConfigRequest,
        auth::kubernetes::responses::ReadKubernetesAuthConfigResponse, AuthInfo,
    },
    client::Client,
    error::ClientError,
};

// Configure Kubernetes auth backend.
//
// See [ConfigureKubernetesAuthRequest]
pub async fn configure(
    client: &impl Client,
    mount: &str,
    kubernetes_host: &str,
    opts: Option<&mut ConfigureKubernetesAuthRequestBuilder>,
) -> Result<(), ClientError> {
    let mut t = ConfigureKubernetesAuthRequest::builder();
    let endpoint = opts
        .unwrap_or(&mut t)
        .mount(mount)
        .kubernetes_host(kubernetes_host)
        .build()
        .unwrap();

    api::exec_with_empty(client, endpoint).await
}

// Configure Kubernetes auth backend.
//
// See [ReadKubernetesAuthConfigResponse]
pub async fn read_config(
    client: &impl Client,
    mount: &str,
) -> Result<ReadKubernetesAuthConfigResponse, ClientError> {
    let endpoint = ReadKubernetesAuthConfigRequest::builder()
        .mount(mount)
        .build()
        .unwrap();

    api::exec_with_result(client, endpoint).await
}

// Fetch a <token with policies using a Kubernetes ServiceAccount.
//
// See [LoginWithKubernetesRequest]
pub async fn login(
    client: &impl Client,
    mount: &str,
    role: &str,
    jwt: &str,
) -> Result<AuthInfo, ClientError> {
    let endpoint = LoginWithKubernetesRequest::builder()
        .mount(mount)
        .role(role)
        .jwt(jwt)
        .build()
        .unwrap();
    api::auth(client, endpoint).await
}

pub mod role {
    use crate::api;
    use crate::api::auth::kubernetes::{
        requests::{
            CreateKubernetesRoleRequest, CreateKubernetesRoleRequestBuilder,
            DeleteKubernetesRoleRequest, ListRolesRequest, ReadKubernetesRoleRequest,
        },
        responses::{ListRolesResponse, ReadKubernetesRoleResponse},
    };
    use crate::client::Client;
    use crate::error::ClientError;

    /// Lists all Kubernetes roles.
    ///
    /// See [ListRolesRequest]
    ///
    pub async fn list(client: &impl Client, mount: &str) -> Result<ListRolesResponse, ClientError> {
        let endpoint = ListRolesRequest::builder().mount(mount).build().unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Reads properties of a Kubernetes role.
    ///
    /// See [ReadKubernetesRoleResponse]
    pub async fn read(
        client: &impl Client,
        mount: &str,
        name: &str,
    ) -> Result<ReadKubernetesRoleResponse, ClientError> {
        let endpoint = ReadKubernetesRoleRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Creates a Kubernetes role.
    ///
    /// See [CreateKubernetesRoleRequest]
    pub async fn create(
        client: &impl Client,
        mount: &str,
        name: &str,
        opts: Option<&mut CreateKubernetesRoleRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = CreateKubernetesRoleRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut t)
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Deletes an existing Kubernetes role.
    ///
    /// See [DeleteKubernetesRoleRequest]
    pub async fn delete(client: &impl Client, mount: &str, name: &str) -> Result<(), ClientError> {
        let endpoint = DeleteKubernetesRoleRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }
}
