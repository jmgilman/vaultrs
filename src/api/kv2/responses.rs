use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Response from executing
/// [ReadConfigurationRequest][crate::api::kv2::requests::ReadConfigurationRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadConfigurationResponse {
    pub cas_required: bool,
    pub delete_version_after: String,
    pub max_versions: u64,
}

/// Response from executing
/// [ReadSecretRequest][crate::api::kv2::requests::ReadSecretRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadSecretResponse {
    pub data: Value,
    pub metadata: SecretVersionMetadata,
}

/// Response from executing
/// [ReadSecretRequest][crate::api::kv2::requests::ReadSecretRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct SecretVersionMetadata {
    pub created_time: String,
    pub deletion_time: String,
    pub destroyed: bool,
    pub version: u64,
}
