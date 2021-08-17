use super::responses::ReadConfigurationResponse;
use crate::api::EndpointResult;
use rustify_derive::Endpoint;
use serde::Serialize;
use serde_with::skip_serializing_none;

/// ## Configure the KV Engine
/// This path configures backend level settings that are applied to every key in
/// the key-value store.
///
/// * Path: {self.mount}/config
/// * Method: POST
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/secret/kv/kv-v2#configure-the-kv-engine
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(path = "{self.mount}/config", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct SetConfigurationRequest {
    #[serde(skip)]
    pub mount: String,
    pub max_versions: u64,
    pub cas_required: bool,
    pub delete_version_after: String,
}

/// ## Read KV Engine configuration
/// This path retrieves the current configuration for the secrets backend at the
/// given path.
///
/// * Path: {self.mount}/config
/// * Method: GET
/// * Response: ReadConfigurationResponse
/// * Reference: https://www.vaultproject.io/api-docs/secret/kv/kv-v2#read-kv-engine-configuration
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/config",
    result = "EndpointResult<ReadConfigurationResponse>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadConfigurationRequest {
    #[serde(skip)]
    pub mount: String,
    pub cas_required: bool,
    pub delete_version_after: String,
    pub max_versions: u64,
}
