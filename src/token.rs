use crate::{
    api::{
        self,
        token::requests::{CreateTokenRequest, CreateTokenRequestBuilder},
        AuthInfo,
    },
    client::VaultClient,
    error::ClientError,
};

/// Creates or updates a role
///
/// See [SetRoleRequest]
pub async fn new(
    client: &VaultClient,
    opts: Option<&mut CreateTokenRequestBuilder>,
) -> Result<AuthInfo, ClientError> {
    let mut t = CreateTokenRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).build().unwrap();
    api::auth(client, endpoint).await
}
