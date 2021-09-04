use crate::{
    api::{
        self,
        token::{
            requests::{
                CreateOrphanTokenRequest, CreateOrphanTokenRequestBuilder, CreateRoleTokenRequest,
                CreateRoleTokenRequestBuilder, CreateTokenRequest, CreateTokenRequestBuilder,
                LookupTokenAccessorRequest, LookupTokenRequest, LookupTokenSelfRequest,
                RenewTokenAccessorRequest, RenewTokenRequest, RenewTokenSelfRequest,
                RevokeTokenAccessorRequest, RevokeTokenOrphanRequest, RevokeTokenRequest,
                RevokeTokenSelfRequest, TidyRequest,
            },
            responses::LookupTokenResponse,
        },
        AuthInfo,
    },
    client::Client,
    error::ClientError,
};

/// Looks up a token
///
/// See [LookupTokenResponse]
pub async fn lookup(client: &impl Client, token: &str) -> Result<LookupTokenResponse, ClientError> {
    let endpoint = LookupTokenRequest::builder().token(token).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Looks up a token by its accessor ID
///
/// See [LookupTokenAccessorRequest]
pub async fn lookup_accessor(
    client: &impl Client,
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
pub async fn lookup_self(client: &impl Client) -> Result<LookupTokenResponse, ClientError> {
    let endpoint = LookupTokenSelfRequest::builder().build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Creates a new token
///
/// See [CreateTokenRequest]
pub async fn new(
    client: &impl Client,
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
    client: &impl Client,
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
    client: &impl Client,
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
    client: &impl Client,
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
    client: &impl Client,
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
    client: &impl Client,
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
pub async fn revoke(client: &impl Client, token: &str) -> Result<(), ClientError> {
    let endpoint = RevokeTokenRequest::builder().token(token).build().unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// Revokes a token by its accessor ID
///
/// See [RevokeTokenAccessorRequest]
pub async fn revoke_accessor(client: &impl Client, accessor: &str) -> Result<(), ClientError> {
    let endpoint = RevokeTokenAccessorRequest::builder()
        .accessor(accessor)
        .build()
        .unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// Revokes a token excluding any child tokens
///
/// See [RevokeTokenOrphanRequest]
pub async fn revoke_orphan(client: &impl Client, token: &str) -> Result<(), ClientError> {
    let endpoint = RevokeTokenOrphanRequest::builder()
        .token(token)
        .build()
        .unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// Revokes the token being sent in the header of this request
///
/// See [RevokeTokenSelfRequest]
pub async fn revoke_self(client: &impl Client) -> Result<(), ClientError> {
    let endpoint = RevokeTokenSelfRequest::builder().build().unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// Tidy's up the token backend
///
/// See [TidyRequest]
pub async fn tidy(client: &impl Client) -> Result<(), ClientError> {
    let endpoint = TidyRequest::builder().build().unwrap();
    api::exec_with_empty_result(client, endpoint).await
}

pub mod role {
    use crate::{
        api::{
            self,
            token::{
                requests::{
                    DeleteTokenRoleRequest, ListTokenRolesRequest, ReadTokenRoleRequest,
                    SetTokenRoleRequest, SetTokenRoleRequestBuilder,
                },
                responses::{ListTokenRolesResponse, ReadTokenRoleResponse},
            },
        },
        client::Client,
        error::ClientError,
    };

    /// Deletes a token role
    ///
    /// See [DeleteTokenRoleRequest]
    pub async fn delete(client: &impl Client, role_name: &str) -> Result<(), ClientError> {
        let endpoint = DeleteTokenRoleRequest::builder()
            .role_name(role_name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// List token roles
    ///
    /// See [ListTokenRolesRequest]
    pub async fn list(client: &impl Client) -> Result<ListTokenRolesResponse, ClientError> {
        let endpoint = ListTokenRolesRequest::builder().build().unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Read a token role
    ///
    /// See [ReadTokenRoleRequest]
    pub async fn read(
        client: &impl Client,
        role_name: &str,
    ) -> Result<ReadTokenRoleResponse, ClientError> {
        let endpoint = ReadTokenRoleRequest::builder()
            .role_name(role_name)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Creates or updates a role
    ///
    /// See [SetTokenRoleRequest]
    pub async fn set(
        client: &impl Client,
        role_name: &str,
        opts: Option<&mut SetTokenRoleRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = SetTokenRoleRequest::builder();
        let endpoint = opts.unwrap_or(&mut t).role_name(role_name).build().unwrap();
        api::exec_with_empty(client, endpoint).await
    }
}
