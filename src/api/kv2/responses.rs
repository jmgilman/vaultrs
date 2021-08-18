use std::collections::HashMap;

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

/// Response from executing
/// [ListSecretsRequest][crate::api::kv2::requests::ListSecretsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListSecretsResponse {
    pub keys: Vec<String>,
}

/// Response from executing
/// [ReadSecretMetadataRequest][crate::api::kv2::requests::ReadSecretMetadataRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadSecretMetadataResponse {
    pub cas_required: bool,
    pub created_time: String,
    pub current_version: u64,
    pub delete_version_after: String,
    pub max_versions: u64,
    pub oldest_version: u64,
    pub updated_time: String,
    pub versions: HashMap<String, SecretMetadata>,
}

/// Response from executing
/// [ReadSecretMetadataRequest][crate::api::kv2::requests::ReadSecretMetadataRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct SecretMetadata {
    pub created_time: String,
    pub deletion_time: String,
    pub destroyed: bool,
}
