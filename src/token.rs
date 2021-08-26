use crate::{
    api::{
        self,
        token::{
            requests::{
                CreateOrphanTokenRequest, CreateOrphanTokenRequestBuilder, CreateRoleTokenRequest,
                CreateRoleTokenRequestBuilder, CreateTokenRequest, CreateTokenRequestBuilder,
                TokenLookupAccessorRequest, TokenLookupRequest, TokenLookupSelfRequest,
                TokenRenewRequest,
            },
            responses::TokenLookupResponse,
        },
        AuthInfo,
    },
    client::VaultClient,
    error::ClientError,
};

/// Looks up a token
///
/// See [TokenLookupRequest]
pub async fn lookup(client: &VaultClient, token: &str) -> Result<TokenLookupResponse, ClientError> {
    let endpoint = TokenLookupRequest::builder().token(token).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Looks up a token by its accessor ID
///
/// See [TokenLookupAccessorRequest]
pub async fn lookup_accessor(
    client: &VaultClient,
    accessor: &str,
) -> Result<TokenLookupResponse, ClientError> {
    let endpoint = TokenLookupAccessorRequest::builder()
        .accessor(accessor)
        .build()
        .unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Looks up the token being sent with this request
///
/// See [TokenLookupSelfRequest]
pub async fn lookup_self(client: &VaultClient) -> Result<TokenLookupResponse, ClientError> {
    let endpoint = TokenLookupSelfRequest::builder().build().unwrap();
    api::exec_with_result(client, endpoint).await
}

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

/// Renews a token
///
/// See [TokenRenewRequest]
pub async fn renew(
    client: &VaultClient,
    token: &str,
    increment: Option<&str>,
) -> Result<AuthInfo, ClientError> {
    let mut endpoint = TokenRenewRequest::builder();
    if let Some(inc) = increment {
        endpoint.increment(inc);
    }
    api::auth(client, endpoint.token(token).build().unwrap()).await
}
