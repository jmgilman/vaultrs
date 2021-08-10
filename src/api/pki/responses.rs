use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GenerateCertificateResponse {
    pub ca_chain: Vec<String>,
    pub certificate: String,
    pub issuing_ca: String,
    pub private_key: String,
    pub private_key_type: String,
    pub serial_number: String,
}

#[derive(Deserialize, Debug)]
pub struct GenerateIntermediateResponse {
    pub csr: String,
    pub private_key: String,
    pub private_key_type: String,
}

#[derive(Deserialize, Debug)]
pub struct GenerateRootResponse {
    pub certificate: String,
    pub issuing_ca: String,
    pub serial_number: String,
}

#[derive(Deserialize, Debug)]
pub struct ListCertificatesResponse {
    pub keys: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct ListRolesResponse {
    pub keys: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct ReadCertificateResponse {
    pub certificate: String,
}

#[derive(Deserialize, Debug)]
pub struct ReadCRLConfigResponse {
    pub disable: bool,
    pub expiry: String,
}

#[derive(Deserialize, Debug)]
pub struct RevokeCertificateResponse {
    pub revocation_time: u64,
}

#[derive(Deserialize, Debug)]
pub struct RotateCRLsResponse {
    pub success: bool,
}

#[derive(Deserialize, Debug)]
pub struct ReadURLsResponse {
    pub issuing_certificates: Vec<String>,
    pub crl_distribution_points: Vec<String>,
    pub ocsp_servers: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct ReadRoleResponse {
    pub allow_any_name: bool,
    pub allow_bare_domains: bool,
    pub allow_glob_domains: bool,
    pub allow_ip_sans: bool,
    pub allow_localhost: bool,
    pub allow_subdomains: bool,
    pub allow_token_displayname: bool,
    pub allowed_domains: Vec<String>,
    pub allowed_domains_template: bool,
    pub allowed_other_sans: Vec<String>,
    pub allowed_serial_numbers: Vec<String>,
    pub allowed_uri_sans: Vec<String>,
    pub basic_constraints_valid_for_non_ca: bool,
    pub client_flag: bool,
    pub code_signing_flag: bool,
    pub country: Vec<String>,
    pub email_protection_flag: bool,
    pub enforce_hostnames: bool,
    pub ext_key_usage: Vec<String>,
    pub ext_key_usage_oids: Vec<String>,
    pub generate_lease: bool,
    pub key_bits: u64,
    pub key_type: String,
    pub key_usage: Vec<String>,
    pub locality: Vec<String>,
    pub max_ttl: u64,
    pub no_store: bool,
    pub not_before_duration: u64,
    pub organization: Vec<String>,
    pub ou: Vec<String>,
    pub policy_identifiers: Vec<String>,
    pub postal_code: Vec<String>,
    pub province: Vec<String>,
    pub require_cn: bool,
    pub server_flag: bool,
    pub street_address: Vec<String>,
    pub ttl: u64,
    pub use_csr_common_name: bool,
    pub use_csr_sans: bool,
}

#[derive(Deserialize, Debug)]
pub struct SignCertificateResponse {
    pub ca_chain: Vec<String>,
    pub certificate: String,
    pub issuing_ca: String,
    pub serial_number: String,
}

#[derive(Deserialize, Debug)]
pub struct SignIntermediateResponse {
    pub ca_chain: Vec<String>,
    pub certificate: String,
    pub issuing_ca: String,
    pub serial_number: String,
}

#[derive(Deserialize, Debug)]
pub struct SignSelfIssuedResponse {
    pub certificate: String,
    pub issuing_ca: String,
}
