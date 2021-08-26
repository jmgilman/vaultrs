use crate::{
    api::{
        self,
        token::requests::{
            CreateOrphanTokenRequest, CreateOrphanTokenRequestBuilder, CreateRoleTokenRequest,
            CreateRoleTokenRequestBuilder, CreateTokenRequest, CreateTokenRequestBuilder,
        },
        AuthInfo,
    },
    client::VaultClient,
    error::ClientError,
};

/// Creates a new token
///
/// See [CreateTokenRequest]
pub async fn new(
    client: &VaultClient,
    opts: Option<&mut CreateTokenRequestBuilder>,
) -> Result<AuthInfo, ClientError> {
    let mut t = CreateTokenRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).build().unwrap();
    api::auth(client, endpoint).await
}

/// Creates a new orphan token
///
/// See [CreateOrphanTokenRequest]
pub async fn new_orphan(
    client: &VaultClient,
    opts: Option<&mut CreateOrphanTokenRequestBuilder>,
) -> Result<AuthInfo, ClientError> {
    let mut t = CreateOrphanTokenRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).build().unwrap();
    api::auth(client, endpoint).await
}

/// Creates a new token based on a role
///
/// See [CreateRoleTokenRequest]
pub async fn new_role(
    client: &VaultClient,
    role: &str,
    opts: Option<&mut CreateRoleTokenRequestBuilder>,
) -> Result<AuthInfo, ClientError> {
    let mut t = CreateRoleTokenRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).role_name(role).build().unwrap();
    api::auth(client, endpoint).await
}
