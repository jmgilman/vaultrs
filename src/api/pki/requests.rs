use crate::{
    api::endpoint::{EmptyEndpointData, EmptyEndpointResult, Endpoint, EndpointResult},
    enums::RequestType,
};

use super::responses::{
    GenerateCertificateResponse, GenerateIntermediateResponse, GenerateRootResponse,
    ListCertificatesResponse, ListRolesResponse, ReadCRLConfigResponse, ReadCertificateResponse,
    ReadRoleResponse, ReadURLsResponse, RevokeCertificateResponse, RotateCRLsResponse,
    SignCertificateResponse, SignIntermediateResponse, SignSelfIssuedResponse,
};
use serde::Serialize;
use serde_with::skip_serializing_none;
use vaultrs_derive::VaultEndpoint;

// Submit CA bundle
#[derive(VaultEndpoint, Debug)]
#[endpoint(path = "config/ca")]
pub struct SubmitCARequest {
    pub data: SubmitCAData,
}

#[skip_serializing_none]
#[derive(Default, Builder, Debug, Serialize)]
#[builder(setter(into, strip_option), default)]
pub struct SubmitCAData {
    pem_bundle: Option<String>,
}

// Generate root certificate
#[derive(VaultEndpoint, Debug)]
#[endpoint(
    path = "root/generate/{self.cert_type}",
    result = "EndpointResult<GenerateRootResponse>"
)]
pub struct GenerateRootRequest {
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
#[derive(VaultEndpoint, Debug)]
#[endpoint(path = "root")]
pub struct DeleteRootRequest {}

// Sign certificate
#[derive(VaultEndpoint, Debug)]
#[endpoint(path = "sign", result = "EndpointResult<SignCertificateResponse>")]
pub struct SignCertificateRequest {
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
#[derive(VaultEndpoint, Debug)]
#[endpoint(
    path = "root/sign-intermediate",
    result = "EndpointResult<SignIntermediateResponse>"
)]
pub struct SignIntermediateRequest {
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
#[derive(VaultEndpoint, Debug)]
#[endpoint(
    path = "root/sign-self-issued",
    result = "EndpointResult<SignSelfIssuedResponse>"
)]
pub struct SignSelfIssuedRequest {
    pub data: SignSelfIssuedData,
}

#[skip_serializing_none]
#[derive(Default, Builder, Debug, Serialize)]
#[builder(setter(into, strip_option), default)]
pub struct SignSelfIssuedData {
    pub certificate: String,
}

// List certificates
#[derive(VaultEndpoint, Debug)]
#[endpoint(
    path = "certs",
    method = "RequestType::LIST",
    result = "EndpointResult<ListCertificatesResponse>"
)]
pub struct ListCertificatesRequest {}

// Read certificate
#[derive(VaultEndpoint, Debug)]
#[endpoint(
    path = "cert/{self.serial}",
    result = "EndpointResult<ReadCertificateResponse>"
)]
pub struct ReadCertificateRequest {
    pub serial: String,
}

// Generate certificate
#[derive(VaultEndpoint, Debug)]
#[endpoint(
    path = "issue/{self.role}",
    result = "EndpointResult<GenerateCertificateResponse>"
)]
pub struct GenerateCertificateRequest {
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
#[derive(VaultEndpoint, Debug)]
#[endpoint(path = "revoke", result = "EndpointResult<RevokeCertificateResponse>")]
pub struct RevokeCertificateRequest {
    pub data: RevokeCertificateData,
}

#[skip_serializing_none]
#[derive(Default, Builder, Debug, Serialize)]
#[builder(setter(into, strip_option), default)]
pub struct RevokeCertificateData {
    pub serial: Option<String>,
}

// Read CRL config
#[derive(VaultEndpoint, Debug)]
#[endpoint(path = "config/crl", result = "EndpointResult<ReadCRLConfigResponse>")]
pub struct ReadCRLConfigRequest {}

// Set CRL config
#[derive(VaultEndpoint, Debug)]
#[endpoint(path = "config/crl")]
pub struct SetCRLConfigRequest {
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
#[derive(VaultEndpoint, Debug)]
#[endpoint(path = "crl/rotate", result = "EndpointResult<RotateCRLsResponse>")]
pub struct RotateCRLsRequest {}

// Read URLs
#[derive(VaultEndpoint, Debug)]
#[endpoint(path = "config/urls", result = "EndpointResult<ReadURLsResponse>")]
pub struct ReadURLsRequest {}

// Set URLs
#[derive(VaultEndpoint, Debug)]
#[endpoint(path = "config/urls")]
pub struct SetURLsRequest {
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
#[derive(VaultEndpoint, Debug)]
#[endpoint(
    path = "intermediate/generate/{self.cert_type}",
    result = "EndpointResult<GenerateIntermediateResponse>"
)]
pub struct GenerateIntermediateRequest {
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
#[derive(VaultEndpoint, Debug)]
#[endpoint(path = "intermediate/set-signed")]
pub struct SetSignedIntermediateRequest {
    pub data: SubmitSignedIntermediateData,
}

#[skip_serializing_none]
#[derive(Default, Builder, Debug, Serialize)]
#[builder(setter(into, strip_option), default)]
pub struct SubmitSignedIntermediateData {
    certificate: Option<String>,
}

// List roles
#[derive(VaultEndpoint, Debug)]
#[endpoint(
    path = "roles",
    method = "RequestType::LIST",
    result = "EndpointResult<ListRolesResponse>"
)]
pub struct ListRolesRequest {}

// Read role
#[derive(VaultEndpoint, Debug)]
#[endpoint(
    path = "roles/{self.name}",
    result = "EndpointResult<ReadRoleResponse>"
)]
pub struct ReadRoleRequest {
    pub name: String,
}

// Set role
#[derive(VaultEndpoint, Debug)]
#[endpoint(path = "roles/{self.name}")]
pub struct SetRoleRequest {
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
#[derive(VaultEndpoint, Debug)]
#[endpoint(path = "roles/{self.name}", method = "RequestType::DELETE")]
pub struct DeleteRoleRequest {
    pub name: String,
}

// Tidy
#[derive(VaultEndpoint, Debug)]
#[endpoint(path = "tidy")]
pub struct TidyRequest {
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
