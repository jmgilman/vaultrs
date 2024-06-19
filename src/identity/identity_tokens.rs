use crate::{
    api::{
        self,
        identity::identity_tokens::{
            requests::{CreateOrUpdateTokenRoleRequest, GenerateSignedIdTokenRequest},
            responses::GenerateSignedIdTokenResponse,
        },
    },
    client::Client,
    error::ClientError,
};

/// Creates or updates an OIDC role.
#[instrument(skip(client), err)]
pub async fn create_or_update_role(
    client: &impl Client,
    name: &str,
    key: &str,
    ttl: &str,
    template: Option<String>,
    client_id: Option<String>,
) -> Result<(), ClientError> {
    let req = CreateOrUpdateTokenRoleRequest::builder()
        .name(name)
        .key(key)
        .ttl(ttl)
        .template(template)
        .client_id(client_id)
        .build()
        .unwrap();
    api::exec_with_empty_result(client, req).await
}

/// Generates a signed ID (OIDC) token against a specific role.
#[instrument(skip(client), err)]
pub async fn generate_signed_id_token(
    client: &impl Client,
    role: &str,
) -> Result<GenerateSignedIdTokenResponse, ClientError> {
    let req = GenerateSignedIdTokenRequest::builder()
        .role(role)
        .build()
        .unwrap();

    api::exec_with_result(client, req).await
}
