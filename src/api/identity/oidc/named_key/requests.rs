use rustify_derive::Endpoint;

/// ## Create a named key.
///
/// This endpoint creates a named key.
///
/// * Path: identity/oidc/key/{name}
/// * Method: POST
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/tokens#create-a-named-key>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "identity/oidc/key/{self.name}",
    method = "POST",
    builder = "true",
)]
#[builder(setter(into, strip_option), default)]
pub struct CreateNamedKeyRequest {
    /// Name of the named key.
    #[endpoint(skip)]
    pub name: String,
    /// How often to generate a new signing key. Uses duration format strings.
    pub rotation_period: Option<String>,
    /// Controls how long the public portion of a signing key will be available for verification after being rotated. Uses duration format strings.
    pub verification_ttl: Option<String>,
    /// Array of role client ids allowed to use this key for signing. If empty, no roles are allowed. If "*", all roles are allowed.
    pub allowed_client_ids: Vec<String>,
    /// Signing algorithm to use. Allowed values are: RS256 (default), RS384, RS512, ES256, ES384, ES512, EdDSA.
    pub signing_algorithm: Option<String>,
}

/// ## Delete a named key.
///
/// This endpoint deletes a named key.
///
/// * Path: identity/oidc/key/{name}
/// * Method: DELETE
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/identity/tokens#delete-a-named-key>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "identity/oidc/key/{self.name}",
    method = "DELETE",
    builder = "true",
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteNamedKeyRequest {
    /// Name of the named key.
    #[endpoint(skip)]
    pub name: String,
}