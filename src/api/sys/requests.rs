use super::responses::{
    AuthResponse, GetConfigurationOfTheSecretEngineResponse, ListPoliciesResponse, MountResponse,
    RandomResponse, ReadHealthResponse, ReadPolicyResponse, RemountResponse, RemountStatusResponse,
    RenewLeaseResponse, StartInitializationResponse, UnsealResponse, WrappingLookupResponse,
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
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/system/mounts#enable-secrets-engine>

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

/// ## Disable Secrets Engine
/// This endpoint disables the mount point specified in the URL.
///
/// * Path: sys/mounts/{self.path}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/system/mounts#disable-secrets-engine>

#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(path = "sys/mounts/{self.path}", method = "DELETE", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct DisableEngineRequest {
    #[endpoint(skip)]
    pub path: String,
}

/// ## Get the configuration of a secret engine
/// This endpoint returns the configuration of a specific secret engine.
///
/// * Path: sys/mounts/{self.path}
/// * Method: GET
/// * Response: GetConfigurationOfTheSecretEngineResponse
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/system/mounts#get-the-configuration-of-a-secret-engine>

#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "sys/mounts/{self.path}",
    method = "GET",
    builder = "true",
    response = "GetConfigurationOfTheSecretEngineResponse"
)]
#[builder(setter(into, strip_option), default)]
pub struct GetConfigurationOfTheSecretEngineRequest {
    #[endpoint(skip)]
    pub path: String,
}

/// ## List Mounted Secrets Engines
/// This endpoints lists all the mounted secrets engines.
///
/// * Path: sys/mounts
/// * Method: GET
/// * Response: [HashMap<String, MountResponse>]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/system/mounts#list-mounted-secrets-engines>

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
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/system/auth#enable-auth-method>

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

/// ## Disable Auth Method
/// This endpoint disables the auth method at the given auth path.
///
/// * Path: sys/auth/{self.path}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/system/auth#disable-auth-method>

#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(path = "sys/auth/{self.path}", method = "DELETE", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct DisableAuthRequest {
    #[endpoint(skip)]
    pub path: String,
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
/// * Response: [HashMap<String, MountResponse>]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/system/auth#list-auth-methods>

#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "sys/auth",
    response = "HashMap<String, AuthResponse>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListAuthsRequest {}

/// ## Move backend
///
/// The `/sys/remount` endpoint moves an already-mounted backend to a new mount point.
/// Remounting works for both secret engines and auth methods.
///
/// * Path: sys/remount
/// * Method: POST
/// * Response: RemountResponse
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/system/remount#move-backend>

#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "sys/remount",
    response = "RemountResponse",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct RemountRequest {
    pub from: String,
    pub to: String,
}

/// ## Get the configuration of a secret engine
/// This endpoint returns the configuration of a specific secret engine.
///
/// * Path: sys/remount/status/{self.migration_id}
/// * Method: GET
/// * Response: RemountStatusResponse
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/system/remount#monitor-migration-status>

#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "sys/remount/status/{self.migration_id}",
    method = "GET",
    builder = "true",
    response = "RemountStatusResponse"
)]
#[builder(setter(into, strip_option), default)]
pub struct RemountStatusRequest {
    #[endpoint(skip)]
    pub migration_id: String,
}

/// ## Wrapping Unwrap
/// This endpoint returns the original response inside the given wrapping token.
///
/// * Path: /sys/wrapping/unwrap
/// * Method: POST
/// * Response: T
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/system/wrapping-unwrap#wrapping-unwrap>

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
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/system/wrapping-unwrap#wrapping-unwrap>

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
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/system/health#read-health-information>

#[derive(Builder, Default, Endpoint)]
#[endpoint(
    path = "/sys/health",
    response = "ReadHealthResponse",
    builder = "true"
)]
#[builder(setter(into), default)]
pub struct ReadHealthRequest {
    #[endpoint(query)]
    pub standbyok: Option<bool>,
    #[endpoint(query)]
    pub perfstandbyok: Option<bool>,
    #[endpoint(query)]
    pub activecode: Option<u16>,
    #[endpoint(query)]
    pub standbycode: Option<u16>,
    #[endpoint(query)]
    pub drsecondarycode: Option<u16>,
    #[endpoint(query)]
    pub haunhealthycode: Option<u16>,
    #[endpoint(query)]
    pub performancestandbycode: Option<u16>,
    #[endpoint(query)]
    pub removedcode: Option<u16>,
    #[endpoint(query)]
    pub sealedcode: Option<u16>,
    #[endpoint(query)]
    pub uninitcode: Option<u16>,
}

/// ## Start Initialization
///
/// This endpoint initializes a new Vault. The Vault must not have been previously initialized.
/// The recovery options, as well as the stored shares option, are only available when using Auto Unseal.
///
/// * Path: /sys/init
/// * Method: POST
/// * Response: [StartInitializationResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/system/init#start-initialization>
#[derive(Builder, Default, Endpoint)]
#[endpoint(
    path = "/sys/init",
    method = "POST",
    response = "StartInitializationResponse",
    builder = "true"
)]
#[builder(setter(into), default)]
pub struct StartInitializationRequest {
    /// Specifies an array of PGP public keys used to encrypt the output unseal keys. Ordering is preserved.
    /// The keys must be base64-encoded from their original binary representation. The size of this array must be the same as secret_shares.
    pgp_keys: Option<Vec<String>>,
    /// Specifies a PGP public key used to encrypt the initial root token. The key must be base64-encoded from its original binary representation.
    root_token_pgp_key: Option<String>,
    /// Specifies the number of shares to split the root key into.
    secret_shares: u64,
    /// Specifies the number of shares required to reconstruct the root key. This must be less than or equal secret_shares.
    secret_threshold: u64,

    /// Additionally, the following options are only supported using Auto Unseal:
    /// Specifies the number of shares that should be encrypted by the HSM and stored for auto-unsealing. Currently must be the same as secret_shares.
    stored_shares: Option<u64>,
    /// Specifies the number of shares to split the recovery key into. This is only available when using Auto Unseal.
    recovery_shares: Option<u64>,
    /// Specifies the number of shares required to reconstruct the recovery key. This must be less than or equal to recovery_shares.
    /// This is only available when using Auto Unseal.
    recovery_threshold: Option<u64>,
    /// Specifies an array of PGP public keys used to encrypt the output recovery keys. Ordering is preserved.
    /// The keys must be base64-encoded from their original binary representation. The size of this array must be the same as recovery_shares. This is only available when using Auto Unseal.
    recovery_pgp_keys: Option<Vec<String>>,
}

/// ## Seal
/// This endpoint seals the Vault.
///
/// * Path: /sys/seal
/// * Method: PUT
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/system/seal#seal>

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
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/system/unseal>

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
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/system/policy#list-policies>

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
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/system/policy#read-policy>

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
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/system/policy#create-update-policy>

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
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/system/policy#delete-policy>

#[derive(Builder, Default, Endpoint)]
#[endpoint(path = "/sys/policy/{self.name}", method = "DELETE", builder = "true")]
#[builder(setter(into), default)]
pub struct DeletePolicyRequest {
    pub name: String,
}

/// ## Generate random bytes
/// This endpoint returns high-quality random bytes of the specified length.
///
/// * Path: /sys/tools/random
/// * Method: POST
/// * Response: [RandomResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/system/tools#generate-random-bytes>

#[derive(Builder, Default, Endpoint)]
#[endpoint(
    path = "/sys/tools/random",
    method = "POST",
    response = "RandomResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct RandomRequest {
    pub bytes: Option<u64>,
    pub format: Option<String>,
    pub source: Option<String>,
}

/// ## Renew Lease
/// This endpoint renews a lease, requesting to extend the lease.
/// Token leases cannot be renewed using this endpoint, use instead the auth/token/renew endpoint.
///
/// * Path: /sys/leases/renew
/// * Method: POST
/// * Response: [RenewLeaseResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/system/leases#renew-lease>

#[derive(Builder, Default, Endpoint)]
#[endpoint(
    path = "/sys/leases/renew",
    method = "POST",
    response = "RenewLeaseResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct RenewLeaseRequest {
    pub lease_id: String,
    pub increment: Option<String>,
}
