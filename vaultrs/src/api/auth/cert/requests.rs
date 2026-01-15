use rustify_derive::Endpoint;

use super::responses::{ListCaCertificateRoleResponse, ReadCaCertificateRoleResponse};

/// ## Create/Update CA certificate role
/// Create or update a CA certificate role.
///
/// * Path: /auth/{self.mount}/certs/{self.name}
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/cert#create-ca-certificate-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/certs/{self.name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct CreateCaCertificateRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
    pub certificate: String,
    pub allowed_common_names: Option<Vec<String>>,
    pub allowed_dns_sans: Option<Vec<String>>,
    pub allowed_email_sans: Option<Vec<String>>,
    pub allowed_uri_sans: Option<Vec<String>>,
    pub allowed_organizational_units: Option<Vec<String>>,
    pub required_extensions: Option<Vec<String>>,
    pub allowed_metadata_extensions: Option<Vec<String>>,
    pub ocsp_enabled: Option<bool>,
    pub ocsp_ca_certificates: Option<String>,
    pub ocsp_servers_override: Option<Vec<String>>,
    pub ocsp_fail_open: Option<bool>,
    pub ocsp_query_all_servers: Option<bool>,
    pub display_name: Option<String>,
    pub token_ttl: Option<String>,
    pub token_max_ttl: Option<String>,
    pub token_policies: Option<Vec<String>>,
    pub token_bound_cidrs: Option<Vec<String>>,
    pub token_explicit_max_ttl: Option<String>,
    pub token_no_default_policy: Option<bool>,
    pub token_num_uses: Option<u64>,
    pub token_period: Option<String>,
    pub token_type: Option<String>,
}

/// ## Read CA certificate role
/// Reads the properties of an existing CA certificate role.
///
/// * Path: /auth/{self.mount}/certs/{self.name}
/// * Method: GET
/// * Response: [ReadCaCertificateRoleResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/cert#read-ca-certificate-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/certs/{self.name}",
    response = "ReadCaCertificateRoleResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadCaCertificateRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}

/// ## Delete CA certificate role
/// This endpoint deletes the CA certificate role.
///
/// * Path: /auth/{self.mount}/certs/{self.name}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/cert#delete-certificate-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/certs/{self.name}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteCaCertificateRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}

/// ## List CA certificate role
/// List available CA certificate roles.
///
/// * Path: /auth/{self.mount}/certs
/// * Method: LIST
/// * Response: [ListCaCertificateRoleResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/cert#list-certificate-roles>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/certs",
    method = "LIST",
    response = "ListCaCertificateRoleResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListCaCertificateRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Configure TLS certificate method
/// Configuration options for the method.
///
/// * Path: /auth/{self.mount}/config
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/cert#configure-tls-certificate-method>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "/auth/{self.mount}/config", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct ConfigureTlsCertificateMethod {
    #[endpoint(skip)]
    pub mount: String,
    /// If set, during renewal, skips the matching of presented client identity with the client identity used during login.
    disable_binding: Option<bool>,
    /// If set, metadata of the certificate including the metadata corresponding to allowed_metadata_extensions will be stored in the alias.
    enable_identity_alias_metadata: Option<bool>,
    /// The size of the OCSP response LRU cache. Note that this cache is used for all configured certificates.
    ocsp_cache_size: Option<u64>,
    /// The size of the role cache. Use -1 to disable role caching.
    role_cache_size: Option<u64>,
}

/// ## Login
/// Login with the TLS certificate method and authenticate against only the named
/// certificate role.
///
/// * Path: /auth/{self.mount}/login
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/cert#login-with-tls-certificate-method>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "/auth/{self.mount}/login", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct LoginRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub cert_name: String,
}
