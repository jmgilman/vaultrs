use super::responses::{
    ConfigureIdentityTokensBackendResponse, CreatedNamedKeyResponse, GenerateSignedIdTokenResponse,
    IntrospectSignedIdTokenResponse, ReadConfigurationsIdentityTokensBackendResponse,
};
use rustify_derive::Endpoint;

/// TODO
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "/identity/oidc/config",
    method = "POST",
    response = "ConfigureIdentityTokensBackendResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct ConfigureIdentityTokensBackendRequest {
    /// TODO
    pub issuer: String,
}

/// TODO
#[derive(Debug, Endpoint)]
#[endpoint(
    path = "/identity/oidc/config",
    method = "GET",
    response = "ReadConfigurationsIdentityTokensBackendResponse"
)]
pub struct ReadConfigurationsIdentityTokensBackendRequest;

pub enum Algorithm {
    EdDSA,
    ES256,
    ES384,
    ES512,
    RS256,
    RS384,
    RS512,
}

/// TODO
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "/identity/oidc/key/{self.name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into))]
pub struct CreatedNamedKeyRequest {
    /// TODO
    #[endpoint(skip)]
    pub name: String,
    /// TODO
    pub rotation_period: String,
    /// TODO
    pub verification_ttl: String,
    /// TODO
    pub allowed_client_ids: Vec<String>,
    /// TODO
    #[builder(default = "Algorithm::RS256")]
    pub algorithm: Algorithm,
}

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
#[builder(setter(into))]
pub struct IntrospectSignedIdTokenRequest {
    /// A signed OIDC-compliant ID token.
    pub token: String,
    /// Specifying the client ID additionally requires the token to contain a matching `aud` claim.
    pub client_id: Option<String>,
}
