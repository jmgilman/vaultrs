use super::responses::{
    GenerateCertificateResponse, GenerateIntermediateResponse, GenerateRootResponse,
    ListCertificatesResponse, ListRolesResponse, ReadCRLConfigResponse, ReadCertificateResponse,
    ReadRoleResponse, ReadURLsResponse, RevokeCertificateResponse, RotateCRLsResponse,
    SignCertificateResponse, SignIntermediateResponse, SignSelfIssuedResponse,
};
use crate::api::strip;
use rustify_derive::Endpoint;
use serde::Serialize;
use serde_with::skip_serializing_none;

// Submit CA bundle
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(path = "{self.mount}/config/ca", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct SubmitCARequest {
    #[serde(skip)]
    pub mount: String,
    pub pem_bundle: String,
}

// Generate root certificate
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/root/generate/{self.cert_type}",
    method = "POST",
    result = "Option<GenerateRootResponse>",
    transform = "strip::<GenerateRootResponse>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct GenerateRootRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
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

// Delete root certificate
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(path = "{self.mount}/root", method = "DELETE", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct DeleteRootRequest {
    #[serde(skip)]
    pub mount: String,
}

// Sign certificate
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/sign/{self.role}",
    method = "POST",
    result = "SignCertificateResponse",
    transform = "strip::<SignCertificateResponse>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct SignCertificateRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
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
}

// Sign intermediate certificate
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/root/sign-intermediate",
    method = "POST",
    result = "SignIntermediateResponse",
    transform = "strip::<SignIntermediateResponse>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct SignIntermediateRequest {
    #[serde(skip)]
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

// Sign self-issued certificate
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/root/sign-self-issued",
    method = "POST",
    result = "SignSelfIssuedResponse",
    transform = "strip::<SignSelfIssuedResponse>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct SignSelfIssuedRequest {
    #[serde(skip)]
    pub mount: String,
    pub certificate: String,
}

// List certificates
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/certs",
    method = "LIST",
    result = "ListCertificatesResponse",
    transform = "strip::<ListCertificatesResponse>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListCertificatesRequest {
    #[serde(skip)]
    pub mount: String,
}

// Read certificate
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/cert/{self.serial}",
    result = "ReadCertificateResponse",
    transform = "strip::<ReadCertificateResponse>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadCertificateRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub serial: String,
}

// Generate certificate
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/issue/{self.role}",
    method = "POST",
    result = "GenerateCertificateResponse",
    transform = "strip::<GenerateCertificateResponse>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct GenerateCertificateRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
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
}

// Revoke certificate
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/revoke",
    method = "POST",
    result = "RevokeCertificateResponse",
    transform = "strip::<RevokeCertificateResponse>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct RevokeCertificateRequest {
    #[serde(skip)]
    pub mount: String,
    pub serial_number: String,
}

// Read CRL config
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/config/crl",
    result = "ReadCRLConfigResponse",
    transform = "strip::<ReadCRLConfigResponse>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadCRLConfigRequest {
    #[serde(skip)]
    pub mount: String,
}

// Set CRL config
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(path = "{self.mount}/config/crl", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct SetCRLConfigRequest {
    #[serde(skip)]
    pub mount: String,
    pub expiry: Option<String>,
    pub disable: Option<bool>,
}

// Rotate CRLs
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/crl/rotate",
    result = "RotateCRLsResponse",
    transform = "strip::<RotateCRLsResponse>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct RotateCRLsRequest {
    #[serde(skip)]
    pub mount: String,
}

// Read URLs
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/config/urls",
    result = "ReadURLsResponse",
    transform = "strip::<ReadURLsResponse>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadURLsRequest {
    #[serde(skip)]
    pub mount: String,
}

// Set URLs
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(path = "{self.mount}/config/urls", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct SetURLsRequest {
    #[serde(skip)]
    pub mount: String,
    pub issuing_certificates: Option<Vec<String>>,
    pub crl_distribution_points: Option<Vec<String>>,
    pub ocsp_servers: Option<Vec<String>>,
}

// Generate intermediate certificate
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/intermediate/generate/{self.cert_type}",
    method = "POST",
    result = "GenerateIntermediateResponse",
    transform = "strip::<GenerateIntermediateResponse>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct GenerateIntermediateRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub cert_type: String,
    pub alt_names: Option<String>,
    pub common_name: Option<String>,
    pub country: Option<Vec<String>>,
    pub exclude_cn_from_sans: Option<bool>,
    pub format: Option<String>,
    pub locality: Option<Vec<String>>,
    pub key_bits: Option<u64>,
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
}

// Set signed intermediate certificate
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/intermediate/set-signed",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct SetSignedIntermediateRequest {
    #[serde(skip)]
    pub mount: String,
    pub certificate: String,
}

// List roles
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/roles",
    method = "LIST",
    result = "ListRolesResponse",
    transform = "strip::<ListRolesResponse>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListRolesRequest {
    #[serde(skip)]
    pub mount: String,
}

// Read role
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/roles/{self.name}",
    result = "ReadRoleResponse",
    transform = "strip::<ReadRoleResponse>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadRoleRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub name: String,
}

// Set role
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/roles/{self.name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct SetRoleRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
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

// Delete role
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/roles/{self.name}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteRoleRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub name: String,
}

// Tidy
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(path = "{self.mount}/tidy", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct TidyRequest {
    #[serde(skip)]
    pub mount: String,
    pub tidy_cert_store: Option<bool>,
    pub tidy_revoked_certs: Option<bool>,
    pub safety_buffer: Option<String>,
}
