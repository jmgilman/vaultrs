use super::responses::GenerateSignedIdTokenResponse;
use rustify_derive::Endpoint;

/// ## Generate a signed token ID
/// This endpoint generates a signed ID (OIDC) token.
///
/// * Path: /identity/oidc/token/{self.role}
/// * Method: GET
/// * Response: [GenerateSignedIdTokenResponse]
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/tokens#generate-a-signed-id-token
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "/identity/oidc/token/{self.role}",
    response = "GenerateSignedIdTokenResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct GenerateSignedIdTokenRequest {
    #[endpoint(skip)]
    pub role: String,
}
