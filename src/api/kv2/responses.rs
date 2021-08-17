use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct ReadConfigurationResponse {
    pub cas_required: bool,
    pub delete_version_after: String,
    pub max_versions: u64,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ReadSecretVersionResponse<T> {
    pub data: T,
    pub metadata: SecretVersionMetadata,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct SecretVersionMetadata {
    pub created_time: String,
    pub deletion_time: String,
    pub destroyed: bool,
    pub version: u64,
}
