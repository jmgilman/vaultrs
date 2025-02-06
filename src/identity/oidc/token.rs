use crate::api;
use crate::api::identity::oidc::token::requests::GeneratedSignedIdTokenRequest;
use crate::api::identity::oidc::token::responses::GeneratedSignedIdTokenResponse;
use crate::client::Client;
use crate::error::ClientError;

/// Generate a signed ID (OIDC) token.
///
/// See [GenerateSignedIdTokenRequest]
pub async fn generate_signed_id_token(
    client: &impl Client,
    name: &str,
) -> Result<GeneratedSignedIdTokenResponse, ClientError> {
    let endpoint = GeneratedSignedIdTokenRequest::builder()
        .name(name).build().unwrap();

    api::exec_with_result(client, endpoint).await
}
