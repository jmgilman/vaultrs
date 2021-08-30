use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Response from executing
/// [ReadRoleRequest][crate::api::ssh::requests::ReadRoleRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadRoleResponse {
    pub key_type: String,
    pub algorithm_signer: String,
    pub allow_bare_domains: bool,
    pub allow_host_certificates: bool,
    pub allow_subdomains: bool,
    pub allow_user_certificates: bool,
    pub allow_user_key_ids: bool,
    pub allowed_user_key_lengths: Option<HashMap<String, u64>>,
    pub allowed_critical_options: Option<HashMap<String, String>>,
    pub allowed_domains: String,
    pub allowed_extensions: String,
    pub allowed_users: String,
    pub allowed_users_template: bool,
    pub admin_user: String,
    pub cidr_list: String,
    pub efault_critical_options: Option<HashMap<String, String>>,
    pub default_user: String,
    pub exclude_cidr_list: String,
    pub install_script: String,
    pub key: String,
    pub key_bits: u64,
    pub key_id_format: String,
    pub key_option_specs: String,
    pub max_ttl: String,
    pub port: u64,
    pub ttl: String,
}

/// Response from executing
/// [ListRolesRequest][crate::api::ssh::requests::ListRolesRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListRolesResponse {
    pub keys: Vec<String>,
    pub key_info: HashMap<String, KeyInfo>,
}

/// Response from executing
/// [ListRolesRequest][crate::api::ssh::requests::ListRolesRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct KeyInfo {
    pub key_type: String,
}

/// Response from executing
/// [ListZeroAddressRolesRequest][crate::api::ssh::requests::ListZeroAddressRolesRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListZeroAddressRolesResponse {
    pub roles: Vec<String>,
}

/// Response from executing
/// [GenerateSSHCredsRequest][crate::api::ssh::requests::GenerateSSHCredsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct GenerateSSHCredsResponse {
    pub allowed_users: String,
    pub admin_user: String,
    pub cidr_list: String,
    pub default_user: String,
    pub exclude_cidr_list: String,
    pub install_script: String,
    pub key: String,
    pub key_bits: u64,
    pub key_option_specs: String,
    pub key_type: String,
    pub port: u64,
}

/// Response from executing
/// [ListRolesByIPRequest][crate::api::ssh::requests::ListRolesByIPRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListRolesByIPResponse {
    pub roles: Vec<String>,
}

/// Response from executing
/// [VerifySSHOTPRequest][crate::api::ssh::requests::VerifySSHOTPRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct VerifySSHOTPResponse {
    pub ip: String,
    pub username: String,
}

/// Response from executing
/// [SubmitCAInfoRequest][crate::api::ssh::requests::SubmitCAInfoRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct SubmitCAInfoResponse {
    pub public_key: String,
}

/// Response from executing
/// [ReadPublicKeyRequest][crate::api::ssh::requests::ReadPublicKeyRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadPublicKeyResponse {
    pub public_key: String,
}

/// Response from executing
/// [SignSSHKeyRequest][crate::api::ssh::requests::SignSSHKeyRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct SignSSHKeyResponse {
    pub serial_number: String,
    pub signed_key: String,
}
