use super::responses::{
    GenerateCertificateResponse, GenerateIntermediateResponse, GenerateRootResponse,
    ListCertificatesResponse, ListRolesResponse, ReadCRLConfigResponse, ReadCertificateResponse,
    ReadRoleResponse, ReadURLsResponse, RevokeCertificateResponse, RotateCRLsResponse,
    SignCertificateResponse, SignIntermediateResponse, SignSelfIssuedResponse,
};
use rustify_derive::Endpoint;

/// ## Submit CA Information
/// This endpoint allows submitting the CA information for the backend via a PEM
/// file containing the CA certificate and its private key, concatenated.
///
/// * Path: {self.mount}/config/ca
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docssecret/pki#submit-ca-information>

#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "{self.mount}/config/ca", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct SubmitCARequest {
    #[endpoint(skip)]
    pub mount: String,
    pub pem_bundle: String,
}

/// ## Generate Root
/// <https://developer.hashicorp.com/vault/api-docssecret/pki#generate-root>
/// This endpoint generates a new self-signed CA certificate and private key. If
/// the path ends with exported, the private key will be returned in the
/// response; if it is internal the private key will not be returned and cannot
/// be retrieved later.
///
/// * Path: {self.mount}/root/generate/{self.cert_type}
/// * Method: POST
/// * Response: [`Option<GenerateRootResponse>`]
/// * Reference: <https://developer.hashicorp.com/vault/api-docssecret/pki#generate-root>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/root/generate/{self.cert_type}",
    method = "POST",
    response = "Option<GenerateRootResponse>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct GenerateRootRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub cert_type: String,
    pub alt_names: Option<String>,
    pub common_name: Option<String>,
    pub country: Option<Vec<String>>,
    pub exclude_cn_from_sans: Option<bool>,
    pub format: Option<String>,
    pub locality: Option<Vec<String>>,
    pub key_bits: Option<u64>,
    pub key_type: Option<String>,
    pub ip_sans: Option<String>,
    pub max_path_length: Option<i32>,
    pub organization: Option<Vec<String>>,
    pub other_sans: Option<Vec<String>>,
    pub ou: Option<Vec<String>>,
    pub permitted_dns_domains: Vec<String>,
    pub postal_code: Option<Vec<String>>,
    pub private_key_format: Option<String>,
    pub province: Option<Vec<String>>,
    pub serial_number: Option<String>,
    pub street_address: Option<Vec<String>>,
    pub ttl: Option<String>,
    pub uri_sans: Option<String>,
}

/// ## Delete Root
/// This endpoint deletes the current CA key (the old CA certificate will still
/// be accessible for reading until a new certificate/key is generated or
/// uploaded).
///
/// * Path: {self.mount}/root
/// * Method: DELETE
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docssecret/pki#delete-root>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "{self.mount}/root", method = "DELETE", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct DeleteRootRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Sign Certificate
/// This endpoint signs a new certificate based upon the provided CSR and the
/// supplied parameters, subject to the restrictions contained in the role named
/// in the endpoint. The issuing CA certificate is returned as well, so that
/// only the root CA need be in a client's trust store.
///
/// * Path: {self.mount}/sign/{self.role}
/// * Method: POST
/// * Response: [SignCertificateResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docssecret/pki#sign-certificate>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/sign/{self.role}",
    method = "POST",
    response = "SignCertificateResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct SignCertificateRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub role: String,
    pub alt_names: Option<String>,
    pub common_name: Option<String>,
    pub csr: Option<String>,
    pub exclude_cn_from_sans: Option<bool>,
    pub format: Option<String>,
    pub ip_sans: Option<String>,
    pub other_sans: Option<Vec<String>>,
    pub serial_number: Option<String>,
    pub ttl: Option<String>,
    pub uri_sans: Option<String>,
    pub remove_roots_from_chain: Option<bool>,
}

/// ## Sign Intermediate
/// This endpoint uses the configured CA certificate to issue a certificate with
/// appropriate values for acting as an intermediate CA.
///
/// * Path: {self.mount}/root/sign-intermediate
/// * Method: POST
/// * Response: [SignIntermediateResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docssecret/pki#sign-intermediate>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/root/sign-intermediate",
    method = "POST",
    response = "SignIntermediateResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct SignIntermediateRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub alt_names: Option<String>,
    pub common_name: Option<String>,
    pub country: Option<Vec<String>>,
    pub csr: Option<String>,
    pub exclude_cn_from_sans: Option<bool>,
    pub format: Option<String>,
    pub locality: Option<Vec<String>>,
    pub ip_sans: Option<String>,
    pub max_path_length: Option<i32>,
    pub organization: Option<Vec<String>>,
    pub other_sans: Option<Vec<String>>,
    pub ou: Option<Vec<String>>,
    pub permitted_dns_domains: Option<Vec<String>>,
    pub postal_code: Option<Vec<String>>,
    pub province: Option<Vec<String>>,
    pub serial_number: Option<String>,
    pub street_address: Option<Vec<String>>,
    pub ttl: Option<String>,
    pub uri_sans: Option<String>,
    pub use_csr_values: Option<bool>,
}

/// ## Sign Self-Issued
/// This endpoint uses the configured CA certificate to sign a self-issued
/// certificate (which will usually be a self-signed certificate as well).
///
/// * Path: {self.mount}/root/sign-self-issued
/// * Method: POST
/// * Response: [SignSelfIssuedResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docssecret/pki#sign-intermediate>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/root/sign-self-issued",
    method = "POST",
    response = "SignSelfIssuedResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct SignSelfIssuedRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub certificate: String,
}

/// ## List Certificates
/// This endpoint returns a list of the current certificates by serial number
/// only.
///
/// * Path: {self.mount}/certs
/// * Method: LIST
/// * Response: [ListCertificatesResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docssecret/pki#list-certificates>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/certs",
    method = "LIST",
    response = "ListCertificatesResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListCertificatesRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Read Certificate
/// This endpoint retrieves one of a selection of certificates. This endpoint
/// returns the certificate in PEM formatting in the certificate key of the JSON
/// object, which is a standard Vault response that is readable by the Vault
/// CLI.
///
/// * Path: {self.mount}/cert/{self.serial}
/// * Method: GET
/// * Response: [ReadCertificateResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docssecret/pki#read-certificate>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/cert/{self.serial}",
    response = "ReadCertificateResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadCertificateRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub serial: String,
}

/// ## Generate Certificate
/// This endpoint generates a new set of credentials (private key and
/// certificate) based on the role named in the endpoint. The issuing CA
/// certificate is returned as well, so that only the root CA need be in a
/// client's trust store.
///
/// * Path: {self.mount}/issue/{self.role}
/// * Method: POST
/// * Response: [GenerateCertificateResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docssecret/pki#read-certificate>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/issue/{self.role}",
    method = "POST",
    response = "GenerateCertificateResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct GenerateCertificateRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub role: String,
    pub alt_names: Option<String>,
    pub common_name: Option<String>,
    pub exclude_cn_from_sans: Option<bool>,
    pub format: Option<String>,
    pub ip_sans: Option<String>,
    pub other_sans: Option<Vec<String>>,
    pub private_key_format: Option<String>,
    pub ttl: Option<String>,
    pub uri_sans: Option<String>,
    pub remove_roots_from_chain: Option<bool>,
}

/// ## Revoke Certificate
/// This endpoint revokes a certificate using its serial number. This is an
/// alternative option to the standard method of revoking using Vault lease IDs.
/// A successful revocation will rotate the CRL.
///
/// * Path: {self.mount}/revoke
/// * Method: POST
/// * Response: [RevokeCertificateResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docssecret/pki#revoke-certificate>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/revoke",
    method = "POST",
    response = "RevokeCertificateResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct RevokeCertificateRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub serial_number: String,
}

/// ## Read CRL Configuration
/// This endpoint allows getting the duration for which the generated CRL should
/// be marked valid.
///
/// * Path: {self.mount}/config/crl
/// * Method: GET
/// * Response: [ReadCRLConfigResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docssecret/pki#read-crl-configuration>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/config/crl",
    response = "ReadCRLConfigResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadCRLConfigRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Set CRL Configuration
/// This endpoint allows setting the duration for which the generated CRL should
/// be marked valid. If the CRL is disabled, it will return a signed but
/// zero-length CRL for any request. If enabled, it will re-build the CRL.
///
/// * Path: {self.mount}/config/crl
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docssecret/pki#set-crl-configuration>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "{self.mount}/config/crl", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct SetCRLConfigRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub expiry: Option<String>,
    pub disable: Option<bool>,
}

/// ## Rotate CRLs
/// This endpoint forces a rotation of the CRL. This can be used by
/// administrators to cut the size of the CRL if it contains a number of
/// certificates that have now expired, but has not been rotated due to no
/// further certificates being revoked.
///
/// * Path: {self.mount}/crl/rotate
/// * Method: GET
/// * Response: [RotateCRLsResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docssecret/pki#rotate-crls>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/crl/rotate",
    response = "RotateCRLsResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct RotateCRLsRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Read URLs
/// This endpoint fetches the URLs to be encoded in generated certificates.
///
/// * Path: {self.mount}/config/urls
/// * Method: GET
/// * Response: [ReadURLsResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docssecret/pki#read-urls>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/config/urls",
    response = "ReadURLsResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadURLsRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Set URLs
/// This endpoint allows setting the issuing certificate endpoints, CRL
/// distribution points, and OCSP server endpoints that will be encoded into
/// issued certificates.
///
/// * Path: {self.mount}/config/urls
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docssecret/pki#set-urls>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "{self.mount}/config/urls", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct SetURLsRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub issuing_certificates: Option<Vec<String>>,
    pub crl_distribution_points: Option<Vec<String>>,
    pub ocsp_servers: Option<Vec<String>>,
}

/// ## Generate Intermediate
/// This endpoint generates a new private key and a CSR for signing. If using
/// Vault as a root, and for many other CAs, the various parameters on the final
/// certificate are set at signing time and may or may not honor the parameters
/// set here.
///
/// * Path: {self.mount}/intermediate/generate/{self.cert_type}
/// * Method: POST
/// * Response: [GenerateIntermediateResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docssecret/pki#generate-intermediate>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/intermediate/generate/{self.cert_type}",
    method = "POST",
    response = "GenerateIntermediateResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct GenerateIntermediateRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub cert_type: String,
    pub alt_names: Option<String>,
    pub common_name: Option<String>,
    pub country: Option<Vec<String>>,
    pub exclude_cn_from_sans: Option<bool>,
    pub format: Option<String>,
    pub locality: Option<Vec<String>>,
    pub key_type: Option<String>,
    pub key_name: Option<String>,
    pub key_ref: Option<String>,
    pub key_bits: Option<u64>,
    pub signature_bits: Option<u64>,
    pub key_format: Option<String>,
    pub ip_sans: Option<String>,
    pub organization: Option<Vec<String>>,
    pub other_sans: Option<Vec<String>>,
    pub ou: Option<Vec<String>>,
    pub postal_code: Option<Vec<String>>,
    pub private_key_format: Option<String>,
    pub province: Option<Vec<String>>,
    pub serial_number: Option<String>,
    pub street_address: Option<Vec<String>>,
    pub uri_sans: Option<String>,
    pub add_basic_constrains: Option<bool>,
}

/// ## Set Signed Intermediate
/// This endpoint allows submitting the signed CA certificate corresponding to a
/// private key generated via /pki/intermediate/generate. The certificate should
/// be submitted in PEM format.
///
/// * Path: {{self.mount}/intermediate/set-signed
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docssecret/pki#set-signed-intermediate>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/intermediate/set-signed",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct SetSignedIntermediateRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub certificate: String,
}

/// ## List Roles
/// This endpoint returns a list of available roles. Only the role names are
/// returned, not any values.
///
/// * Path: {self.mount}/roles
/// * Method: LIST
/// * Response: [ListRolesResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docssecret/pki#list-roles>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/roles",
    method = "LIST",
    response = "ListRolesResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListRolesRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Read Role
/// This endpoint queries the role definition.
///
/// * Path: {self.mount}/roles/{self.name}
/// * Method: GET
/// * Response: [ReadRoleResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docssecret/pki#read-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/roles/{self.name}",
    response = "ReadRoleResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}

/// ## Create/Update Role
/// This endpoint creates or updates the role definition.
///
/// * Path: {self.mount}/roles/{self.name}
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docssecret/pki#create-update-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/roles/{self.name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct SetRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
    pub allow_any_name: Option<bool>,
    pub allow_bare_domains: Option<bool>,
    pub allow_glob_domains: Option<bool>,
    pub allow_ip_sans: Option<bool>,
    pub allow_localhost: Option<bool>,
    pub allow_subdomains: Option<bool>,
    pub allow_token_displayname: Option<bool>,
    pub allowed_domains: Option<Vec<String>>,
    pub allowed_domains_template: Option<bool>,
    pub allowed_other_sans: Option<Vec<String>>,
    pub allowed_serial_numbers: Option<Vec<String>>,
    pub allowed_uri_sans: Option<Vec<String>>,
    pub basic_constraints_valid_for_non_ca: Option<bool>,
    pub client_flag: Option<bool>,
    pub code_signing_flag: Option<bool>,
    pub country: Option<Vec<String>>,
    pub email_protection_flag: Option<bool>,
    pub enforce_hostnames: Option<bool>,
    pub ext_key_usage: Option<Vec<String>>,
    pub ext_key_usage_oids: Option<Vec<String>>,
    pub generate_lease: Option<bool>,
    pub key_bits: Option<u64>,
    pub key_type: Option<String>,
    pub key_usage: Option<Vec<String>>,
    pub locality: Option<Vec<String>>,
    pub max_ttl: Option<u64>,
    pub no_store: Option<bool>,
    pub not_before_duration: Option<u64>,
    pub organization: Option<Vec<String>>,
    pub ou: Option<Vec<String>>,
    pub policy_identifiers: Option<Vec<String>>,
    pub postal_code: Option<Vec<String>>,
    pub province: Option<Vec<String>>,
    pub require_cn: Option<bool>,
    pub server_flag: Option<bool>,
    pub street_address: Option<Vec<String>>,
    pub ttl: Option<u64>,
    pub use_csr_common_name: Option<bool>,
    pub use_csr_sans: Option<bool>,
}

/// ## Delete Role
/// This endpoint deletes the role definition. Deleting a role does not revoke
/// certificates previously issued under this role.
///
/// * Path: {self.mount}/roles/{self.name}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docssecret/pki#delete-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/roles/{self.name}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}

/// ## Tidy
/// This endpoint allows tidying up the storage backend and/or CRL by removing
/// certificates that have expired and are past a certain buffer period beyond
/// their expiration time.
///
/// * Path: {self.mount}/tidy
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docssecret/pki#tidy>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/tidy",
    method = "POST",
    response = "()",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct TidyRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub tidy_cert_store: Option<bool>,
    pub tidy_revoked_certs: Option<bool>,
    pub safety_buffer: Option<String>,
}
