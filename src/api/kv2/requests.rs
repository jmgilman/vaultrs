use super::responses::{ReadConfigurationResponse, ReadSecretResponse};
use crate::api::EndpointResult;
use rustify_derive::Endpoint;
use serde::Serialize;
use serde_json::Value;
use serde_with::skip_serializing_none;
use std::fmt::Debug;

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
    pub delete_version_after: Option<String>,
    pub cas_required: Option<bool>,
    pub max_versions: Option<u64>,
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
}

/// ## Read Secret Version
/// This endpoint retrieves the secret at the specified location.
///
/// * Path: {self.mount}/data/{self.path}?version={self.version}
/// * Method: GET
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/secret/kv/kv-v2#read-secret-version
#[derive(Builder, Debug, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/data/{self.path}",
    result = "EndpointResult<ReadSecretResponse>",
    builder = "true"
)]
#[builder(setter(into))]
pub struct ReadSecretRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub path: String,
    #[serde(skip)]
    #[query]
    #[builder(default = "0")]
    pub version: u64,
}

/// ## Create/Update Secret
/// This endpoint creates a new version of a secret at the specified location.
///
/// * Path: {self.mount}/data/{self.path}
/// * Method: POST
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/secret/kv/kv-v2#create-update-secret
#[derive(Builder, Debug, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/data/{self.path}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into))]
pub struct SetSecretRequest {
    #[serde(skip)]
    pub mount: String,
    #[serde(skip)]
    pub path: String,
    pub data: Value,
}
