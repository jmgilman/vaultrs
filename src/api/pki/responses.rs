use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Response from executing
/// [GenerateCertificateRequest][crate::api::pki::requests::GenerateCertificateRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct GenerateCertificateResponse {
    pub ca_chain: Option<Vec<String>>,
    pub certificate: String,
    pub expiration: Option<u64>,
    pub issuing_ca: String,
    pub private_key: String,
    pub private_key_type: String,
    pub serial_number: String,
}

/// Response from executing
/// [GenerateIntermediateRequest][crate::api::pki::requests::GenerateIntermediateRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct GenerateIntermediateResponse {
    pub csr: String,
    pub private_key: Option<String>,
    pub private_key_type: Option<String>,
}

/// Response from executing
/// [CrossSignRequest][crate::api::pki::requests::CrossSignRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct CrossSignResponse {
    pub csr: String,
    pub key_id: Option<String>,
}

/// Response from executing
/// [GenerateRootRequest][crate::api::pki::requests::GenerateRootRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct GenerateRootResponse {
    pub certificate: String,
    pub issuing_ca: String,
    pub serial_number: String,
}

/// Response from executing
/// [ListCertificatesRequest][crate::api::pki::requests::ListCertificatesRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListCertificatesResponse {
    pub keys: Vec<String>,
}

/// Response from executing
/// [ListRolesRequest][crate::api::pki::requests::ListRolesRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListRolesResponse {
    pub keys: Vec<String>,
}

/// Response from executing
/// [ReadCertificateRequest][crate::api::pki::requests::ReadCertificateRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadCertificateResponse {
    pub certificate: String,
    pub revocation_time: i64,
    pub ca_chain: Option<String>,
}

/// Response from executing
/// [ReadCRLConfigRequest][crate::api::pki::requests::ReadCRLConfigRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadCRLConfigResponse {
    pub disable: bool,
    pub expiry: String,
}

/// Response from executing
/// [RevokeCertificateRequest][crate::api::pki::requests::RevokeCertificateRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct RevokeCertificateResponse {
    pub revocation_time: u64,
}

/// Response from executing
/// [RotateCRLsRequest][crate::api::pki::requests::RotateCRLsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct RotateCRLsResponse {
    pub success: bool,
}

/// Response from executing
/// [ReadURLsRequest][crate::api::pki::requests::ReadURLsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadURLsResponse {
    pub issuing_certificates: Vec<String>,
    pub crl_distribution_points: Vec<String>,
    pub ocsp_servers: Vec<String>,
}

/// Response from executing
/// [ReadRoleRequest][crate::api::pki::requests::ReadRoleRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadRoleResponse {
    pub allow_any_name: bool,
    pub allow_bare_domains: bool,
    pub allow_glob_domains: bool,
    pub allow_ip_sans: bool,
    pub allow_localhost: bool,
    pub allow_subdomains: bool,
    pub allow_token_displayname: bool,
    pub allow_wildcard_certificates: bool,
    pub allowed_domains: Vec<String>,
    pub allowed_domains_template: bool,
    pub allowed_other_sans: Vec<String>,
    pub allowed_serial_numbers: Vec<String>,
    pub allowed_uri_sans: Vec<String>,
    pub allowed_uri_sans_template: bool,
    pub allowed_user_ids: Vec<String>,
    pub basic_constraints_valid_for_non_ca: bool,
    pub client_flag: bool,
    pub cn_validations: Vec<String>,
    pub code_signing_flag: bool,
    pub country: Vec<String>,
    pub email_protection_flag: bool,
    pub enforce_hostnames: bool,
    pub ext_key_usage: Vec<String>,
    pub ext_key_usage_oids: Vec<String>,
    pub generate_lease: bool,
    pub issuer_ref: String,
    pub key_bits: u64,
    pub key_type: String,
    pub key_usage: Vec<String>,
    pub locality: Vec<String>,
    pub max_ttl: u64,
    pub no_store: bool,
    pub not_after: String,
    pub not_before_duration: u64,
    pub organization: Vec<String>,
    pub ou: Vec<String>,
    pub policy_identifiers: Vec<String>,
    pub postal_code: Vec<String>,
    pub province: Vec<String>,
    pub require_cn: bool,
    pub server_flag: bool,
    pub signature_bits: u16,
    pub street_address: Vec<String>,
    pub ttl: u64,
    pub use_csr_common_name: bool,
    pub use_csr_sans: bool,
    pub use_pss: bool,
}

/// Response from executing
/// [SignCertificateRequest][crate::api::pki::requests::SignCertificateRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct SignCertificateResponse {
    pub ca_chain: Option<Vec<String>>,
    pub certificate: String,
    pub issuing_ca: String,
    pub serial_number: String,
    pub expiration: u64,
}

/// Response from executing
/// [SignIntermediateRequest][crate::api::pki::requests::SignIntermediateRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct SignIntermediateResponse {
    pub ca_chain: Option<Vec<String>>,
    pub certificate: String,
    pub issuing_ca: String,
    pub serial_number: String,
}

/// Response from executing
/// [SignSelfIssuedRequest][crate::api::pki::requests::SignSelfIssuedRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct SignSelfIssuedResponse {
    pub certificate: String,
    pub issuing_ca: String,
}

/// Response from executing
/// [ReadIssuerCertificateRequest][crate::api::pki::requests::ReadIssuerCertificateRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadIssuerCertificateResponse {
    pub certificate: String,
    pub ca_chain: Vec<String>,
    pub issuer_id: String,
    pub issuer_name: String,
}

/// Response from executing
/// [SignIntermediateIssuerRequest][crate::api::pki::requests::SignIntermediateIssuerRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct SignIntermediateIssuerResponse {
    pub certificate: String,
    pub ca_chain: Vec<String>,
    pub issuing_ca: String,
    pub serial_number: String,
    pub expiration: u64,
}

/// Response from executing
/// [ImportIssuerRequest][crate::api::pki::requests::ImportIssuerRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ImportIssuerResponse {
    pub imported_issuers: Option<Vec<String>>,
    pub imported_keys: Option<Vec<String>>,
    pub existing_issuers: Option<Vec<String>>,
    pub existing_keys: Option<Vec<String>>,
    pub mapping: Option<HashMap<String, String>>,
}

/// Response from executing
/// [SetDefaultIssuerRequest][crate::api::pki::requests::SetDefaultIssuerRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct SetDefaultIssuerResponse {
    pub default: String,
    pub default_follows_latest_issuer: bool,
}

/// Response from executing
/// [GenerateIntermediateCSRRequest][crate::api::pki::requests::GenerateIntermediateCSRRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct GenerateIntermediateCSRResponse {
    pub csr: String,
    pub key_id: Option<String>,
}
