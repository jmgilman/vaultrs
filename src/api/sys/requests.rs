use super::responses::{
    AuthResponse, ListPoliciesResponse, MountResponse, ReadHealthResponse, ReadPolicyResponse,
    UnsealResponse, WrappingLookupResponse,
};
use rustify_derive::Endpoint;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

/// ## Enable Secrets Engine
/// This endpoint enables a new secrets engine at the given path.
///
/// * Path: sys/mounts/{self.path}
/// * Method: POST
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/system/mounts#enable-secrets-engine

#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(path = "sys/mounts/{self.path}", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct EnableEngineRequest {
    #[endpoint(skip)]
    pub path: String,
    #[serde(rename = "type")]
    pub engine_type: Option<String>,
    pub description: Option<String>,
    pub config: Option<EnableEngineDataConfig>,
    pub options: Option<HashMap<String, String>>,
}

#[derive(Clone, Builder, Debug, Default, Serialize)]
#[builder(setter(into, strip_option), default)]
pub struct EnableEngineDataConfig {
    pub default_lease_ttl: Option<String>,
    pub max_lease_ttl: Option<String>,
    pub force_no_cache: Option<bool>,
    pub audit_non_hmac_request_keys: Option<Vec<String>>,
    pub audit_non_hmac_response_keys: Option<Vec<String>>,
    pub listing_visibility: Option<String>,
    pub passthrough_request_headers: Option<Vec<String>>,
    pub allowed_response_headers: Option<Vec<String>>,
}

/// ## List Mounted Secrets Engines
/// This endpoints lists all the mounted secrets engines.
///
/// * Path: sys/mounts
/// * Method: GET
/// * Response: [HashMap<String, MountResponse]
/// * Reference: https://www.vaultproject.io/api-docs/system/mounts#list-mounted-secrets-engines

#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "sys/mounts",
    response = "HashMap<String, MountResponse>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListMountsRequest {}

/// ## Enable Auth Method
/// This endpoint enables a new auth method.
///
/// * Path: sys/auth/{self.path}
/// * Method: POST
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/system/auth#enable-auth-method

#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(path = "sys/auth/{self.path}", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct EnableAuthRequest {
    #[endpoint(skip)]
    pub path: String,
    #[serde(rename = "type")]
    pub engine_type: Option<String>,
    pub description: Option<String>,
    pub config: Option<EnableAuthDataConfig>,
}

#[derive(Clone, Builder, Debug, Default, Serialize)]
#[builder(setter(into, strip_option), default)]
pub struct EnableAuthDataConfig {
    pub default_lease_ttl: Option<String>,
    pub max_lease_ttl: Option<String>,
    pub force_no_cache: Option<bool>,
    pub audit_non_hmac_request_keys: Option<Vec<String>>,
    pub audit_non_hmac_response_keys: Option<Vec<String>>,
    pub listing_visibility: Option<String>,
    pub passthrough_request_headers: Option<Vec<String>>,
    pub allowed_response_headers: Option<Vec<String>>,
}

/// ## List Auth Methods
/// This endpoint lists all enabled auth methods.
///
/// * Path: sys/auth
/// * Method: GET
/// * Response: [HashMap<String, MountResponse]
/// * Reference: https://www.vaultproject.io/api-docs/system/auth#list-auth-methods

#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "sys/auth",
    response = "HashMap<String, AuthResponse>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListAuthsRequest {}

/// ## Wrapping Unwrap
/// This endpoint returns the original response inside the given wrapping token.
///
/// * Path: /sys/wrapping/unwrap
/// * Method: POST
/// * Response: T
/// * Reference: https://www.vaultproject.io/api-docs/system/wrapping-unwrap#wrapping-unwrap

#[derive(Builder, Endpoint)]
#[endpoint(path = "/sys/wrapping/unwrap", method = "POST", response = "Value")]
#[builder(setter(into))]
pub struct UnwrapRequest {
    pub token: Option<String>,
}

/// ## Wrapping Lookup
/// This endpoint returns the wrapping token properties.
///
/// * Path: /sys/wrapping/lookup
/// * Method: POST
/// * Response: WrappingLookupResponse
/// * Reference: https://www.vaultproject.io/api-docs/system/wrapping-unwrap#wrapping-unwrap

#[derive(Builder, Default, Endpoint)]
#[endpoint(
    path = "/sys/wrapping/lookup",
    method = "POST",
    response = "WrappingLookupResponse",
    builder = "true"
)]
#[builder(setter(into), default)]
pub struct WrappingLookupRequest {
    pub token: String,
}

/// ## Read Health Information
/// This endpoint is used to check the health status of Vault.
///
/// * Path: /sys/health
/// * Method: GET
/// * Response: [ReadHealthResponse]
/// * Reference: https://www.vaultproject.io/api-docs/system/health#read-health-information

#[derive(Builder, Default, Endpoint)]
#[endpoint(
    path = "/sys/health",
    response = "ReadHealthResponse",
    builder = "true"
)]
#[builder(setter(into), default)]
pub struct ReadHealthRequest {}

/// ## Seal
/// This endpoint seals the Vault.
///
/// * Path: /sys/seal
/// * Method: PUT
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/system/seal#seal

#[derive(Builder, Default, Endpoint)]
#[endpoint(path = "/sys/seal", method = "PUT", builder = "true")]
#[builder(setter(into), default)]
pub struct SealRequest {}

/// ## Unseal
/// This endpoint unseals the Vault.
///
/// * Path: /sys/unseal
/// * Method: PUT
/// * Response: [UnsealResponse]
/// * Reference: https://www.vaultproject.io/api-docs/system/unseal

#[derive(Builder, Default, Endpoint)]
#[endpoint(
    path = "/sys/unseal",
    method = "PUT",
    response = "UnsealResponse",
    builder = "true"
)]
#[builder(setter(into), default)]
pub struct UnsealRequest {
    pub key: Option<String>,
    pub reset: Option<bool>,
    pub migrate: Option<bool>,
}

/// ## List Policies
/// This endpoint lists all configured policies.
///
/// * Path: /sys/policy
/// * Method: GET
/// * Response: [ListPoliciesResponse]
/// * Reference: https://www.vaultproject.io/api-docs/system/policy#list-policies

#[derive(Builder, Default, Endpoint)]
#[endpoint(
    path = "/sys/policy",
    response = "ListPoliciesResponse",
    builder = "true"
)]
#[builder(setter(into), default)]
pub struct ListPoliciesRequest {}

/// ## Read Policy
/// This endpoint retrieve the policy body for the named policy.
///
/// * Path: /sys/policy/{self.name}
/// * Method: GET
/// * Response: [ReadPolicyResponse]
/// * Reference: https://www.vaultproject.io/api-docs/system/policy#read-policy

#[derive(Builder, Default, Endpoint)]
#[endpoint(
    path = "/sys/policy/{self.name}",
    response = "ReadPolicyResponse",
    builder = "true"
)]
#[builder(setter(into), default)]
pub struct ReadPolicyRequest {
    pub name: String,
}

/// ## Create/Update Policy
/// This endpoint adds a new or updates an existing policy.
///
/// * Path: /sys/policy/{self.name}
/// * Method: PUT
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/system/policy#create-update-policy

#[derive(Builder, Default, Endpoint)]
#[endpoint(path = "/sys/policy/{self.name}", method = "PUT", builder = "true")]
#[builder(setter(into), default)]
pub struct CreatePolicyRequest {
    pub name: String,
    pub policy: String,
}

/// ## Delete Policy
/// This endpoint deletes the policy with the given name.
///
/// * Path: /sys/policy/{self.name}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/system/policy#delete-policy

#[derive(Builder, Default, Endpoint)]
#[endpoint(path = "/sys/policy/{self.name}", method = "DELETE", builder = "true")]
#[builder(setter(into), default)]
pub struct DeletePolicyRequest {
    pub name: String,
}
