use std::collections::HashMap;

use super::responses::{
    GenerateSSHCredsResponse, ListRolesByIPResponse, ListRolesResponse,
    ListZeroAddressRolesResponse, ReadPublicKeyResponse, ReadRoleResponse, SignSSHKeyResponse,
    SubmitCAInfoResponse, VerifySSHOTPResponse,
};
use rustify_derive::Endpoint;

/// ## Create/Update Key
/// This endpoint creates or updates a named key.
///
/// * Path: {self.mount}/keys/{self.name}
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/ssh#create-update-key>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/keys/{self.name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct SetKeyRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
    pub key: String,
}

/// ## Delete Key
/// This endpoint deletes a named key.
///
/// * Path: {self.mount}/keys/{self.name}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/ssh#delete-key>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/keys/{self.name}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteKeyRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}

/// ## Create Role
/// This endpoint creates or updates a named role.
///
/// * Path: {self.mount}/roles/{self.name}
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/ssh#create-role>
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
    pub name: String,
    pub key_type: String,
    pub algorithm_signer: Option<String>,
    pub allow_bare_domains: Option<bool>,
    pub allow_host_certificates: Option<bool>,
    pub allow_subdomains: Option<bool>,
    pub allow_user_certificates: Option<bool>,
    pub allow_user_key_ids: Option<bool>,
    pub allowed_user_key_lengths: Option<HashMap<String, u64>>,
    pub allowed_critical_options: Option<HashMap<String, String>>,
    pub allowed_domains: Option<String>,
    pub allowed_extensions: Option<String>,
    pub allowed_users: Option<String>,
    pub allowed_users_template: Option<bool>,
    pub admin_user: Option<String>,
    pub cidr_list: Option<String>,
    pub efault_critical_options: Option<HashMap<String, String>>,
    pub default_user: Option<String>,
    pub exclude_cidr_list: Option<String>,
    pub install_script: Option<String>,
    pub key: Option<String>,
    pub key_bits: Option<u64>,
    pub key_id_format: Option<String>,
    pub key_option_specs: Option<String>,
    pub max_ttl: Option<String>,
    pub port: Option<u64>,
    pub ttl: Option<String>,
}

/// ## Read Role
/// This endpoint queries a named role.
///
/// * Path: {self.mount}/roles/{self.name}
/// * Method: GET
/// * Response: [ReadRoleResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/ssh#read-role>
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

/// ## List Roles
/// This endpoint returns a list of available roles.
///
/// * Path: {self.mount}/roles
/// * Method: LIST
/// * Response: [ListRolesResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/ssh#list-roles>
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

/// ## Delete Role
/// This endpoint deletes a named role.
///
/// * Path: {self.mount}/roles/{self.name}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/ssh#delete-role>
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

/// ## List Zero-Address Roles
/// This endpoint returns the list of configured zero-address roles.
///
/// * Path: {self.mount}/config/zeroaddress
/// * Method: GET
/// * Response: [ListZeroAddressRolesResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/ssh#list-zero-address-roles>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/config/zeroaddress",
    response = "ListZeroAddressRolesResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListZeroAddressRolesRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Configure Zero-Address Roles
/// This endpoint configures zero-address roles.
///
/// * Path: {self.mount}/config/zeroaddress
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/ssh#configure-zero-address-roles>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/config/zeroaddress",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ConfigureZeroAddressRolesRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub roles: Vec<String>,
}

/// ## Delete Zero-Address Role
/// This endpoint deletes the zero-address roles configuration.
///
/// * Path: {self.mount}/roles/{self.name}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/ssh#delete-zero-address-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/roles/zeroaddress",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteZeroAddressRolesRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Generate SSH Credentials
/// This endpoint creates credentials for a specific username and IP with the
/// parameters defined in the given role.
///
/// * Path: {self.mount}/creds/{self.name}
/// * Method: POST
/// * Response: [GenerateSSHCredsResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/ssh#generate-ssh-credentials>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/creds/{self.name}",
    method = "POST",
    response = "GenerateSSHCredsResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct GenerateSSHCredsRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
    pub ip: String,
    pub username: Option<String>,
}

/// ## List Roles by IP
/// This endpoint lists all of the roles with which the given IP is associated.
///
/// * Path: {self.mount}/lookup
/// * Method: POST
/// * Response: [ListRolesByIPResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/ssh#list-roles-by-ip>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/lookup",
    method = "POST",
    response = "ListRolesByIPResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListRolesByIPRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub ip: String,
}

/// ## Verify SSH OTP
/// This endpoint verifies if the given OTP is valid.
///
/// * Path: {self.mount}/verify
/// * Method: POST
/// * Response: [VerifySSHOTPResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/ssh#verify-ssh-otp>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/verify",
    method = "POST",
    response = "VerifySSHOTPResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct VerifySSHOTPRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub otp: String,
}

/// ## Submit CA Information
/// This endpoint allows submitting the CA information for the secrets engine
/// via an SSH key pair.
///
/// * Path: {self.mount}/config/ca
/// * Method: POST
/// * Response: [SubmitCAInfoResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/ssh#submit-ca-information>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/config/ca",
    method = "POST",
    response = "SubmitCAInfoResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct SubmitCAInfoRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub generate_signing_key: Option<bool>,
    pub private_key: Option<String>,
    pub public_key: Option<String>,
}

/// ## Delete CA Information
/// This endpoint deletes the CA information for the backend via an SSH key pair.
///
/// * Path: {self.mount}/config/ca
/// * Method: DELETE
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/ssh#delete-ca-information>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "{self.mount}/config/ca", method = "DELETE", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct DeleteCAInfoRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Read Public Key
/// This endpoint reads the configured/generated public key.
///
/// * Path: {self.mount}/config/ca
/// * Method: GET
/// * Response: [ReadPublicKeyResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/ssh#read-public-key-authenticated>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/config/ca",
    response = "ReadPublicKeyResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadPublicKeyRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Sign SSH Key
/// This endpoint signs an SSH public key based on the supplied parameters,
/// subject to the restrictions contained in the role named in the endpoint.
///
/// * Path: {self.mount}/sign/{self.name}
/// * Method: POST
/// * Response: [SignSSHKeyResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/ssh#sign-ssh-key>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/sign/{self.name}",
    method = "POST",
    response = "SignSSHKeyResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct SignSSHKeyRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
    pub cert_type: Option<String>,
    pub critical_options: Option<HashMap<String, String>>,
    pub extensions: Option<HashMap<String, String>>,
    pub key_id: Option<String>,
    pub public_key: String,
    pub ttl: Option<String>,
    pub valid_principals: Option<String>,
}
