use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Response from executing
/// [ListMountsRequest][crate::api::sys::requests::ListMountsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct MountResponse {
    accessor: String,
    config: MountConfigResponse,
    description: String,
    external_entropy_access: bool,
    local: bool,
    options: Option<HashMap<String, String>>,
    seal_wrap: bool,
    #[serde(rename = "type")]
    mount_type: String,
    uuid: String,
}

/// Response from executing
/// [ListMountsRequest][crate::api::sys::requests::ListMountsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct MountConfigResponse {
    default_lease_ttl: u64,
    force_no_cache: bool,
    max_lease_ttl: u64,
}

/// Response from executing
/// [ListAuthsRequest][crate::api::sys::requests::ListAuthsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct AuthResponse {
    accessor: String,
    config: AuthConfigResponse,
    description: String,
    external_entropy_access: bool,
    local: bool,
    options: Option<HashMap<String, String>>,
    seal_wrap: bool,
    #[serde(rename = "type")]
    mount_type: String,
    uuid: String,
}

/// Response from executing
/// [ListAuthsRequest][crate::api::sys::requests::ListAuthsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct AuthConfigResponse {
    default_lease_ttl: u64,
    force_no_cache: bool,
    max_lease_ttl: u64,
    token_type: String,
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
    pub cluster_id: String,
    pub cluster_name: String,
    pub initialized: bool,
    pub performance_standby: bool,
    pub replication_dr_mode: Option<String>,
    pub replication_perf_mode: Option<String>,
    pub sealed: bool,
    pub server_time_utc: u64,
    pub standby: bool,
    pub version: String,
}
