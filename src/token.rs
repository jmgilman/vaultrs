use crate::{
    api::{
        self,
        token::{
            requests::{
                CreateOrphanTokenRequest, CreateOrphanTokenRequestBuilder, CreateRoleTokenRequest,
                CreateRoleTokenRequestBuilder, CreateTokenRequest, CreateTokenRequestBuilder,
                LookupTokenAccessorRequest, LookupTokenRequest, LookupTokenSelfRequest,
                RenewTokenAccessorRequest, RenewTokenRequest, RenewTokenSelfRequest,
                RevokeTokenAccessorRequest, RevokeTokenRequest, RevokeTokenSelfRequest,
            },
            responses::LookupTokenResponse,
        },
        AuthInfo,
    },
    client::VaultClient,
    error::ClientError,
};

/// Looks up a token
///
/// See [LookupTokenResponse]
pub async fn lookup(client: &VaultClient, token: &str) -> Result<LookupTokenResponse, ClientError> {
    let endpoint = LookupTokenRequest::builder().token(token).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Looks up a token by its accessor ID
///
/// See [LookupTokenAccessorRequest]
pub async fn lookup_accessor(
    client: &VaultClient,
    accessor: &str,
) -> Result<LookupTokenResponse, ClientError> {
    let endpoint = LookupTokenAccessorRequest::builder()
        .accessor(accessor)
        .build()
        .unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Looks up the token being sent in the header of this request
///
/// See [LookupTokenSelfRequest]
pub async fn lookup_self(client: &VaultClient) -> Result<LookupTokenResponse, ClientError> {
    let endpoint = LookupTokenSelfRequest::builder().build().unwrap();
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
/// See [RenewTokenRequest]
pub async fn renew(
    client: &VaultClient,
    token: &str,
    increment: Option<&str>,
) -> Result<AuthInfo, ClientError> {
    let mut endpoint = RenewTokenRequest::builder();
    if let Some(inc) = increment {
        endpoint.increment(inc);
    }
    api::auth(client, endpoint.token(token).build().unwrap()).await
}

/// Renews the token by its accessor ID
///
/// See [RenewTokenAccessorRequest]
pub async fn renew_accessor(
    client: &VaultClient,
    accessor: &str,
    increment: Option<&str>,
) -> Result<AuthInfo, ClientError> {
    let mut endpoint = RenewTokenAccessorRequest::builder();
    if let Some(inc) = increment {
        endpoint.increment(inc);
    }
    api::auth(client, endpoint.accessor(accessor).build().unwrap()).await
}

/// Renews the token being sent in the header of this request
///
/// See [RenewTokenSelfRequest]
pub async fn renew_self(
    client: &VaultClient,
    increment: Option<&str>,
) -> Result<AuthInfo, ClientError> {
    let mut endpoint = RenewTokenSelfRequest::builder();
    if let Some(inc) = increment {
        endpoint.increment(inc);
    }
    api::auth(client, endpoint.build().unwrap()).await
}

/// Revokes a token
///
/// See [RevokeTokenRequest]
pub async fn revoke(client: &VaultClient, token: &str) -> Result<(), ClientError> {
    let endpoint = RevokeTokenRequest::builder().token(token).build().unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// Revokes a token by its accessor ID
///
/// See [RevokeTokenAccessorRequest]
pub async fn revoke_accessor(client: &VaultClient, accessor: &str) -> Result<(), ClientError> {
    let endpoint = RevokeTokenAccessorRequest::builder()
        .accessor(accessor)
        .build()
        .unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// Revokes the token being sent in the header of this request
///
/// See [RevokeTokenSelfRequest]
pub async fn revoke_self(client: &VaultClient) -> Result<(), ClientError> {
    let endpoint = RevokeTokenSelfRequest::builder().build().unwrap();
    api::exec_with_empty(client, endpoint).await
}
