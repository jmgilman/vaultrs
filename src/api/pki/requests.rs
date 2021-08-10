use super::responses::{
    GenerateCertificateResponse, GenerateIntermediateResponse, GenerateRootResponse,
    ListCertificatesResponse, ListRolesResponse, ReadCRLConfigResponse, ReadCertificateResponse,
    ReadRoleResponse, ReadURLsResponse, RevokeCertificateResponse, RotateCRLsResponse,
    SignCertificateResponse, SignIntermediateResponse, SignSelfIssuedResponse,
};
use crate::api::EndpointResult;
use rustify_derive::Endpoint;
use serde::Serialize;
use serde_with::skip_serializing_none;

// Submit CA bundle
#[derive(Endpoint, Debug)]
#[endpoint(path = "{self.mount}/config/ca")]
pub struct SubmitCARequest {
    pub mount: String,
    pub data: SubmitCAData,
}

#[skip_serializing_none]
#[derive(Default, Builder, Debug, Serialize)]
#[builder(setter(into, strip_option), default)]
pub struct SubmitCAData {
    pem_bundle: Option<String>,
}

// Generate root certificate
#[derive(Endpoint, Debug)]
#[endpoint(
    path = "{self.mount}/root/generate/{self.cert_type}",
    result = "EndpointResult<GenerateRootResponse>"
)]
pub struct GenerateRootRequest {
    pub mount: String,
    pub cert_type: String,
    pub data: GenerateRootData,
}

#[skip_serializing_none]
#[derive(Default, Builder, Debug, Serialize)]
#[builder(setter(into, strip_option), default)]
pub struct GenerateRootData {
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
#[derive(Endpoint, Debug)]
#[endpoint(path = "{self.mount}/root")]
pub struct DeleteRootRequest {
    pub mount: String,
}

// Sign certificate
#[derive(Endpoint, Debug)]
#[endpoint(
    path = "{self.mount}/sign",
    result = "EndpointResult<SignCertificateResponse>"
)]
pub struct SignCertificateRequest {
    pub mount: String,
    pub data: SignCertificateData,
}

#[skip_serializing_none]
#[derive(Default, Builder, Debug, Serialize)]
#[builder(setter(into, strip_option), default)]
pub struct SignCertificateData {
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
#[derive(Endpoint, Debug)]
#[endpoint(
    path = "{self.mount}/root/sign-intermediate",
    result = "EndpointResult<SignIntermediateResponse>"
)]
pub struct SignIntermediateRequest {
    pub mount: String,
    pub data: SignIntermediateData,
}

#[skip_serializing_none]
#[derive(Default, Builder, Debug, Serialize)]
#[builder(setter(into, strip_option), default)]
pub struct SignIntermediateData {
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
#[derive(Endpoint, Debug)]
#[endpoint(
    path = "{self.mount}/root/sign-self-issued",
    result = "EndpointResult<SignSelfIssuedResponse>"
)]
pub struct SignSelfIssuedRequest {
    pub mount: String,
    pub data: SignSelfIssuedData,
}

#[skip_serializing_none]
#[derive(Default, Builder, Debug, Serialize)]
#[builder(setter(into, strip_option), default)]
pub struct SignSelfIssuedData {
    pub certificate: String,
}

// List certificates
#[derive(Endpoint, Debug)]
#[endpoint(
    path = "{self.mount}/certs",
    method = "RequestType::LIST",
    result = "EndpointResult<ListCertificatesResponse>"
)]
pub struct ListCertificatesRequest {
    pub mount: String,
}

// Read certificate
#[derive(Endpoint, Debug)]
#[endpoint(
    path = "{self.mount}/cert/{self.serial}",
    result = "EndpointResult<ReadCertificateResponse>"
)]
pub struct ReadCertificateRequest {
    pub mount: String,
    pub serial: String,
}

// Generate certificate
#[derive(Endpoint, Debug)]
#[endpoint(
    path = "{self.mount}/issue/{self.role}",
    result = "EndpointResult<GenerateCertificateResponse>"
)]
pub struct GenerateCertificateRequest {
    pub mount: String,
    pub role: String,
    pub data: GenerateCertificateData,
}

#[skip_serializing_none]
#[derive(Default, Builder, Debug, Serialize)]
#[builder(setter(into, strip_option), default)]
pub struct GenerateCertificateData {
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
#[derive(Endpoint, Debug)]
#[endpoint(
    path = "{self.mount}/revoke",
    result = "EndpointResult<RevokeCertificateResponse>"
)]
pub struct RevokeCertificateRequest {
    pub mount: String,
    pub data: RevokeCertificateData,
}

#[skip_serializing_none]
#[derive(Default, Builder, Debug, Serialize)]
#[builder(setter(into, strip_option), default)]
pub struct RevokeCertificateData {
    pub serial: Option<String>,
}

// Read CRL config
#[derive(Endpoint, Debug)]
#[endpoint(
    path = "{self.mount}/config/crl",
    result = "EndpointResult<ReadCRLConfigResponse>"
)]
pub struct ReadCRLConfigRequest {
    pub mount: String,
}

// Set CRL config
#[derive(Endpoint, Debug)]
#[endpoint(path = "{self.mount}/config/crl")]
pub struct SetCRLConfigRequest {
    pub mount: String,
    pub data: SetCRLConfigData,
}

#[skip_serializing_none]
#[derive(Default, Builder, Debug, Serialize)]
#[builder(setter(into, strip_option), default)]
pub struct SetCRLConfigData {
    pub expiry: Option<String>,
    pub disable: Option<bool>,
}

// Rotate CRLs
#[derive(Endpoint, Debug)]
#[endpoint(
    path = "{self.mount}/crl/rotate",
    result = "EndpointResult<RotateCRLsResponse>"
)]
pub struct RotateCRLsRequest {
    pub mount: String,
}

// Read URLs
#[derive(Endpoint, Debug)]
#[endpoint(
    path = "{self.mount}/config/urls",
    result = "EndpointResult<ReadURLsResponse>"
)]
pub struct ReadURLsRequest {
    pub mount: String,
}

// Set URLs
#[derive(Endpoint, Debug)]
#[endpoint(path = "{self.mount}/config/urls")]
pub struct SetURLsRequest {
    pub mount: String,
    pub data: SetURLsData,
}

#[skip_serializing_none]
#[derive(Default, Builder, Debug, Serialize)]
#[builder(setter(into, strip_option), default)]
pub struct SetURLsData {
    pub issuing_certificates: Option<Vec<String>>,
    pub crl_distribution_points: Option<Vec<String>>,
    pub ocsp_servers: Option<Vec<String>>,
}

// Generate intermediate certificate
#[derive(Endpoint, Debug)]
#[endpoint(
    path = "{self.mount}/intermediate/generate/{self.cert_type}",
    result = "EndpointResult<GenerateIntermediateResponse>"
)]
pub struct GenerateIntermediateRequest {
    pub mount: String,
    pub cert_type: String,
    pub data: GenerateIntermediateData,
}

#[skip_serializing_none]
#[derive(Default, Builder, Debug, Serialize)]
#[builder(setter(into, strip_option), default)]
pub struct GenerateIntermediateData {
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
#[derive(Endpoint, Debug)]
#[endpoint(path = "{self.mount}/intermediate/set-signed")]
pub struct SetSignedIntermediateRequest {
    pub mount: String,
    pub data: SubmitSignedIntermediateData,
}

#[skip_serializing_none]
#[derive(Default, Builder, Debug, Serialize)]
#[builder(setter(into, strip_option), default)]
pub struct SubmitSignedIntermediateData {
    certificate: Option<String>,
}

// List roles
#[derive(Endpoint, Debug)]
#[endpoint(
    path = "{self.mount}/roles",
    method = "RequestType::LIST",
    result = "EndpointResult<ListRolesResponse>"
)]
pub struct ListRolesRequest {
    pub mount: String,
}

// Read role
#[derive(Endpoint, Debug)]
#[endpoint(
    path = "{self.mount}/roles/{self.name}",
    result = "EndpointResult<ReadRoleResponse>"
)]
pub struct ReadRoleRequest {
    pub mount: String,
    pub name: String,
}

// Set role
#[derive(Endpoint, Debug)]
#[endpoint(path = "{self.mount}/roles/{self.name}")]
pub struct SetRoleRequest {
    pub mount: String,
    pub name: String,
    pub data: SetRoleData,
}

#[skip_serializing_none]
#[derive(Default, Builder, Debug, Serialize)]
#[builder(setter(into, strip_option), default)]
pub struct SetRoleData {
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
#[derive(Endpoint, Debug)]
#[endpoint(
    path = "{self.mount}/roles/{self.name}",
    method = "RequestType::DELETE"
)]
pub struct DeleteRoleRequest {
    pub mount: String,
    pub name: String,
}

// Tidy
#[derive(Endpoint, Debug)]
#[endpoint(path = "{self.mount}/tidy")]
pub struct TidyRequest {
    pub mount: String,
    pub data: TidyData,
}

#[skip_serializing_none]
#[derive(Default, Builder, Debug, Serialize)]
#[builder(setter(into, strip_option), default)]
pub struct TidyData {
    pub tidy_cert_store: Option<bool>,
    pub tidy_revoked_certs: Option<bool>,
    pub safety_buffer: Option<String>,
}
