use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Debug, Serialize)]
pub struct MountConfigResponse {
    default_lease_ttl: u64,
    force_no_cache: bool,
    max_lease_ttl: u64,
}
