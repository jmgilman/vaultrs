use crate::{
    api::{
        self,
        auth::identity::{
            requests::GenerateSignedIdTokenRequest, responses::IntrospectSignedIdTokenResponse,
        },
        auth::identity::{
            requests::IntrospectSignedIdTokenRequest, responses::GenerateSignedIdTokenResponse,
        },
    },
    client::Client,
    error::ClientError,
};

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
