use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Response from executing
/// [ListMountsRequest][crate::api::sys::requests::ListMountsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct MountResponse {
    pub accessor: String,
    pub config: MountConfigResponse,
    pub description: String,
    pub external_entropy_access: bool,
    pub local: bool,
    pub options: Option<HashMap<String, String>>,
    pub seal_wrap: bool,
    #[serde(rename = "type")]
    pub mount_type: String,
    pub uuid: String,
}

/// Response from executing
/// [ListMountsRequest][crate::api::sys::requests::ListMountsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct MountConfigResponse {
    pub default_lease_ttl: u64,
    pub force_no_cache: bool,
    pub max_lease_ttl: u64,
}

/// Response from executing
/// [GetConfigurationOfTheSecretEngineRequest][crate::api::sys::requests::GetConfigurationOfTheSecretEngineRequest ]
#[derive(Deserialize, Debug, Serialize)]
pub struct GetConfigurationOfTheSecretEngineResponse {
    pub accessor: String,
    pub config: MountConfigResponse,
    pub description: String,
    pub external_entropy_access: bool,
    pub local: bool,
    pub options: Option<HashMap<String, String>>,
    pub plugin_version: Option<String>,
    pub running_plugin_version: Option<String>,
    pub running_sha256: Option<String>,
    pub seal_wrap: bool,
    #[serde(rename = "type")]
    pub mount_type: String,
    pub uuid: String,
}

/// Response from executing
/// [ListAuthsRequest][crate::api::sys::requests::ListAuthsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct AuthResponse {
    pub accessor: String,
    pub config: AuthConfigResponse,
    pub description: String,
    pub external_entropy_access: bool,
    pub local: bool,
    pub options: Option<HashMap<String, String>>,
    pub seal_wrap: bool,
    #[serde(rename = "type")]
    pub mount_type: String,
    pub uuid: String,
}

/// Response from executing
/// [ListAuthsRequest][crate::api::sys::requests::ListAuthsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct AuthConfigResponse {
    pub default_lease_ttl: u64,
    pub force_no_cache: bool,
    pub max_lease_ttl: u64,
    pub token_type: String,
}

/// Response from executing
/// [reMountRequest][crate::api::sys::requests::ReMountRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct RemountResponse {
    pub migration_id: String,
}

/// Response from executing
/// [reMountRequest][crate::api::sys::requests::ReMountRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct RemountStatusResponse {
    pub migration_id: String,
    pub migration_info: MigrationInfo,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct MigrationInfo {
    pub source_mount: String,
    pub target_mount: String,
    pub status: String,
}

/// Response from executing
/// [WrappingLookupRequest][crate::api::sys::requests::WrappingLookupRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct WrappingLookupResponse {
    pub creation_path: String,
    pub creation_time: String,
    pub creation_ttl: u64,
}

/// Response from executing
/// [ReadHealthRequest][crate::api::sys::requests::ReadHealthRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadHealthResponse {
    pub initialized: bool,
    pub sealed: bool,
    pub standby: bool,
    pub performance_standby: bool,
    pub replication_performance_mode: String,
    pub replication_dr_mode: String,
    pub server_time_utc: i64,
    pub version: String,
    pub cluster_name: Option<String>,
    pub cluster_id: Option<String>,
    pub last_wal: Option<u64>,
    pub enterprise: bool,
    pub echo_duration_ms: i64,
    pub clock_skew_ms: i64,
    pub replication_primary_canary_age_ms: Option<i64>,
    pub removed_from_cluster: Option<bool>,
    pub ha_connection_healthy: Option<bool>,
    pub last_request_forwarding_heartbeat_ms: Option<i64>,
}

/// Response from executing
/// [StartInitializationRequest][crate::api::sys::requests::StartInitializationRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct StartInitializationResponse {
    pub keys: Vec<String>,
    pub keys_base64: Vec<String>,
    pub root_token: String,
}

/// Response from executing
/// [UnsealRequest][crate::api::sys::requests::UnsealRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct UnsealResponse {
    pub sealed: bool,
    #[serde(rename = "t")]
    pub threshold: u64,
    #[serde(rename = "n")]
    pub n_shares: u64,
    pub progress: u64,
    pub version: String,
    pub cluster_name: Option<String>,
    pub cluster_id: Option<String>,
}

/// Response from executing
/// [ListPoliciesRequest][crate::api::sys::requests::ListPoliciesRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListPoliciesResponse {
    pub policies: Vec<String>,
}

/// Response from executing
/// [ReadPolicyRequest][crate::api::sys::requests::ReadPolicyRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadPolicyResponse {
    pub name: String,
    pub rules: String,
}

/// Response from executing
/// [RandomRequest][crate::api::sys::requests::RandomRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct RandomResponse {
    pub random_bytes: String,
}

/// Response from executing
/// [RenewLeaseRequest][crate::api::sys::requests::RenewLeaseRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct RenewLeaseResponse {
    pub lease_id: String,
    pub renewable: bool,
    pub lease_duration: u32,
}
