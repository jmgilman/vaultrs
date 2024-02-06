use rustify_derive::Endpoint;

/// ## Login
/// Login with the TLS certificate method and authenticate against only the named
/// certificate role.
///
/// * Path: /auth/{self.mount}/login
/// * Method: POST
/// * Response: N/A
/// * Reference: https://developer.hashicorp.com/vault/api-docs/auth/cert#login-with-tls-certificate-method
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "/auth/{self.mount}/login", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct LoginRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub cert_name: String,
}
