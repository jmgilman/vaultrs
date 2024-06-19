use crate::{
    api::{
        self,
        auth::oidc::{
            requests::{JWTLoginRequest, OIDCAuthRequest, OIDCCallbackRequest},
            responses::OIDCAuthResponse,
        },
        AuthInfo,
    },
    client::Client,
    error::ClientError,
};

/// Obtain an authorization URL from Vault to start an OIDC login flow
///
/// See [OIDCAuthRequest]
pub async fn auth(
    client: &impl Client,
    mount: &str,
    redirect_uri: &str,
    role: Option<String>,
) -> Result<OIDCAuthResponse, ClientError> {
    let mut endpoint = OIDCAuthRequest::builder();
    if let Some(r) = role {
        endpoint.role(r);
    }
    api::exec_with_result(
        client,
        endpoint
            .mount(mount)
            .redirect_uri(redirect_uri)
            .build()
            .unwrap(),
    )
    .await
}

/// Exchange an authorization code for an OIDC ID Token
///
/// See [OIDCCallbackRequest]
pub async fn callback(
    client: &impl Client,
    mount: &str,
    state: &str,
    nonce: &str,
    code: &str,
) -> Result<AuthInfo, ClientError> {
    let endpoint = OIDCCallbackRequest::builder()
        .mount(mount)
        .state(state)
        .nonce(nonce)
        .code(code)
        .build()
        .unwrap();
    api::auth(client, endpoint).await
}

/// Fetch a token using a JWT token
///
/// See [JWTLoginRequest]
pub async fn login(
    client: &impl Client,
    mount: &str,
    jwt: &str,
    role: Option<String>,
) -> Result<AuthInfo, ClientError> {
    let mut endpoint = JWTLoginRequest::builder();
    if let Some(r) = role {
        endpoint.role(r);
    }
    api::auth(client, endpoint.mount(mount).jwt(jwt).build().unwrap()).await
}

pub mod config {
    use crate::{
        api::{
            self,
            auth::oidc::{
                requests::{
                    ReadConfigurationRequest, SetConfigurationRequest,
                    SetConfigurationRequestBuilder,
                },
                responses::ReadConfigurationResponse,
            },
        },
        client::Client,
        error::ClientError,
    };

    /// Read the configuration of the mounted KV engine
    ///
    /// See [ReadConfigurationResponse]
    pub async fn read(
        client: &impl Client,
        mount: &str,
    ) -> Result<ReadConfigurationResponse, ClientError> {
        let endpoint = ReadConfigurationRequest::builder()
            .mount(mount)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Update the configuration of the mounted KV engine
    ///
    /// See [SetConfigurationRequest]
    pub async fn set(
        client: &impl Client,
        mount: &str,
        opts: Option<&mut SetConfigurationRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = SetConfigurationRequest::builder();
        let endpoint = opts.unwrap_or(&mut t).mount(mount).build().unwrap();
        api::exec_with_empty(client, endpoint).await
    }
}

pub mod role {
    use crate::api;
    use crate::api::auth::oidc::{
        requests::{
            DeleteRoleRequest, ListRolesRequest, ReadRoleRequest, SetRoleRequest,
            SetRoleRequestBuilder,
        },
        responses::{ListRolesResponse, ReadRoleResponse},
    };
    use crate::client::Client;
    use crate::error::ClientError;

    /// Deletes a role
    ///
    /// See [DeleteRoleRequest]
    pub async fn delete(client: &impl Client, mount: &str, name: &str) -> Result<(), ClientError> {
        let endpoint = DeleteRoleRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Lists all roles
    ///
    /// See [ListRolesRequest]
    pub async fn list(client: &impl Client, mount: &str) -> Result<ListRolesResponse, ClientError> {
        let endpoint = ListRolesRequest::builder().mount(mount).build().unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Reads a role
    ///
    /// See [ReadRoleRequest]
    pub async fn read(
        client: &impl Client,
        mount: &str,
        name: &str,
    ) -> Result<ReadRoleResponse, ClientError> {
        let endpoint = ReadRoleRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Creates or updates a role
    ///
    /// See [SetRoleRequest]
    pub async fn set(
        client: &impl Client,
        mount: &str,
        name: &str,
        user_claim: &str,
        allowed_redirect_uris: Vec<String>,
        opts: Option<&mut SetRoleRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = SetRoleRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut t)
            .mount(mount)
            .name(name)
            .user_claim(user_claim)
            .allowed_redirect_uris(allowed_redirect_uris)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }
}
