use super::responses::{GenerateSignedIdTokenResponse, IntrospectSignedIdTokenResponse};
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
    /// The name of the role against which to generate a signed ID token.
    #[endpoint(skip)]
    pub role: String,
}

/// ## Introspect a signed ID token
/// This endpoint can verify the authenticity and active state of a signed ID token.
///
/// * Path: /identity/oidc/introspect
/// * Method: POST
/// * Response: [IntrospectSignedIdTokenResponse]
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/tokens#introspect-a-signed-id-token
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "/identity/oidc/introspect",
    response = "IntrospectSignedIdTokenResponse",
    method = "POST",
    builder = "true"
)]
pub struct IntrospectSignedIdTokenRequest {
    /// A signed OIDC-compliant ID token.
    pub token: String,
    /// Specifying the client ID additionally requires the token to contain a matching `aud` claim.
    pub client_id: Option<String>,
}
