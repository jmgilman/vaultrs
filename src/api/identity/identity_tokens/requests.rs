use super::responses::GenerateSignedIdTokenResponse;
use rustify_derive::Endpoint;

/// ## Create or update a role
/// Create or update a role. ID tokens are generated against a role and signed against a named key.
///
/// * Path: /identity/oidc/role/{self.name}
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/tokens#create-or-update-a-role>
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "/identity/oidc/role/{self.name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into))]
pub struct CreateOrUpdateTokenRoleRequest {
    /// The name of the role to create
    #[endpoint(skip)]
    pub name: String,
    pub key: String,
    pub ttl: String,
    pub template: Option<String>,
    pub client_id: Option<String>,
}

/// ## Generate a signed token ID
/// This endpoint generates a signed ID (OIDC) token.
///
/// * Path: /identity/oidc/token/{self.role}
/// * Method: GET
/// * Response: [GenerateSignedIdTokenResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/tokens#generate-a-signed-id-token>
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
