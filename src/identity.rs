use crate::{
    api::{
        self,
        auth::identity::{
            requests::GenerateSignedIdTokenRequest, responses::GenerateSignedIdTokenResponse,
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
