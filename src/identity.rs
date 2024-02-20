use crate::{
    api::{
        self,
        auth::identity::{
            requests::{
                ConfigureIdentityTokensBackendRequest, CreatedNamedKeyRequest,
                GenerateSignedIdTokenRequest, IntrospectSignedIdTokenRequest,
                ReadConfigurationsIdentityTokensBackendRequest,
            },
            responses::{
                ConfigureIdentityTokensBackendResponse, GenerateSignedIdTokenResponse,
                IntrospectSignedIdTokenResponse, ReadConfigurationsIdentityTokensBackendResponse,
            },
        },
    },
    client::Client,
    error::ClientError,
};

/// TODO
#[instrument(skip(client), err)]
pub async fn configure_identify_tokens_backend(
    client: &impl Client,
    issuer: String,
) -> Result<ConfigureIdentityTokensBackendResponse, ClientError> {
    let req = ConfigureIdentityTokensBackendRequest::builder()
        .issuer(issuer)
        .build()
        .unwrap();

    api::exec_with_result(client, req).await
}

/// TODO
#[instrument(skip(client), err)]
pub async fn read_configurations_identity_tokens_backend(
    client: &impl Client,
) -> Result<ReadConfigurationsIdentityTokensBackendResponse, ClientError> {
    let req = ReadConfigurationsIdentityTokensBackendRequest;

    api::exec_with_result(client, req).await
}

/// TODO
#[instrument(skip(client), err)]
pub async fn create_named_key(client: &impl Client) -> Result<(), ClientError> {
    let req = CreatedNamedKeyRequest::builder().build().unwrap();

    api::exec_with_no_result(client, req).await
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

/// Introspects a signed ID (OIDC) token.
pub async fn introspect_signed_id_token(
    client: &impl Client,
    token: String,
    client_id: Option<String>,
) -> Result<IntrospectSignedIdTokenResponse, ClientError> {
    let req = IntrospectSignedIdTokenRequest::builder()
        .token(token)
        .client_id(client_id)
        .build()
        .unwrap();

    api::exec_with_result(client, req).await
}
