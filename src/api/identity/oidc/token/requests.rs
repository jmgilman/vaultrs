use crate::api::identity::oidc::token::responses::GeneratedSignedIdTokenResponse;
use rustify_derive::Endpoint;

/// ## Generate a signed ID (OIDC) token.
///
/// This endpoint generates a signed ID (OIDC) token.
///
/// * Path: identity/oidc/token/{name}
/// * Method: GET
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/tokens#generate-a-signed-id-token>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "identity/oidc/token/{self.name}",
    method = "GET",
    builder = "true",
    response = "GeneratedSignedIdTokenResponse"
)]
#[builder(setter(into, strip_option), default)]
pub struct GeneratedSignedIdTokenRequest {
    /// Name of the role against which to generate a signed ID token.
    #[endpoint(skip)]
    pub name: String,
}
