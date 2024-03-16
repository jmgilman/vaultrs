use super::responses::{
    ListSecretsResponse, ReadConfigurationResponse, ReadSecretMetadataResponse, ReadSecretResponse,
    SecretVersionMetadata,
};
use rustify_derive::Endpoint;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Debug;

/// ## Configure the KV Engine
/// This path configures backend level settings that are applied to every key in
/// the key-value store.
///
/// * Path: {self.mount}/config
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/kv/kv-v2#configure-the-kv-engine>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "{self.mount}/config", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct SetConfigurationRequest {
    #[endpoint(skip)]
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
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/kv/kv-v2#read-kv-engine-configuration>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/config",
    response = "ReadConfigurationResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadConfigurationRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Read Secret Version
/// This endpoint retrieves the secret at the specified location.
///
/// * Path: {self.mount}/data/{self.path}
/// * Method: GET
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/kv/kv-v2#read-secret-version>
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/data/{self.path}",
    response = "ReadSecretResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct ReadSecretRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub path: String,
    #[builder(default = "None")]
    #[endpoint(query)]
    pub version: Option<u64>,
}

/// ## Create/Update Secret
/// This endpoint creates a new version of a secret at the specified location.
///
/// * Path: {self.mount}/data/{self.path}
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/kv/kv-v2#create-update-secret>
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/data/{self.path}",
    response = "SecretVersionMetadata",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into))]
pub struct SetSecretRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub path: String,
    pub data: Value,
    #[builder(setter(strip_option), default)]
    pub options: Option<SetSecretRequestOptions>,
}

#[derive(Builder, Clone, Debug, serde::Serialize)]
#[builder(setter(into))]
pub struct SetSecretRequestOptions {
    pub cas: u32,
}

/// ## Delete Latest Version of Secret
/// This endpoint issues a soft delete of the secret's latest version at the
/// specified location.
///
/// * Path: {self.mount}/data/{self.path}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/kv/kv-v2#delete-latest-version-of-secret>
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/data/{self.path}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into))]
pub struct DeleteLatestSecretVersionRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub path: String,
}

/// ## Delete Secret Versions
/// This endpoint issues a soft delete of the specified versions of the secret.
///
/// * Path: {self.mount}/delete/{self.path}
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/kv/kv-v2#delete-secret-versions>
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/delete/{self.path}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into))]
pub struct DeleteSecretVersionsRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub path: String,
    pub versions: Vec<u64>,
}

/// ## Undelete Secret Versions
/// Undeletes the data for the provided version and path in the key-value store.
///
/// * Path: {self.mount}/undelete/{self.path}
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/kv/kv-v2#undelete-secret-versions>
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/undelete/{self.path}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into))]
pub struct UndeleteSecretVersionsRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub path: String,
    pub versions: Vec<u64>,
}

/// ## Destroy Secret Versions
/// Permanently removes the specified version data for the provided key and
/// version numbers from the key-value store.
///
/// * Path: {self.mount}/destroy/{self.path}
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/kv/kv-v2#destroy-secret-versions>
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/destroy/{self.path}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into))]
pub struct DestroySecretVersionsRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub path: String,
    pub versions: Vec<u64>,
}

/// ## List Secrets
/// This endpoint returns a list of key names at the specified location.
///
/// * Path: {self.mount}/metadata/{self.path}
/// * Method: LIST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/kv/kv-v2#list-secrets>
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/metadata/{self.path}",
    response = "ListSecretsResponse",
    method = "LIST",
    builder = "true"
)]
#[builder(setter(into))]
pub struct ListSecretsRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub path: String,
}

/// ## Read Secret Metadata
/// This endpoint retrieves the metadata and versions for the secret at the
/// specified path.
///
/// * Path: {self.mount}/metadata/{self.path}
/// * Method: GET
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/kv/kv-v2#read-secret-metadata>
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/metadata/{self.path}",
    response = "ReadSecretMetadataResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct ReadSecretMetadataRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub path: String,
}

/// ## Create/Update Metadata
/// This endpoint creates or updates the metadata of a secret at the specified
/// location.
///
/// * Path: {self.mount}/metadata/{self.path}
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/kv/kv-v2#create-update-metadata>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/metadata/{self.path}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct SetSecretMetadataRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub path: String,
    pub max_versions: Option<u64>,
    pub cas_required: Option<bool>,
    pub delete_version_after: Option<String>,
    pub custom_metadata: Option<HashMap<String, String>>,
}

/// ## Delete Metadata and All Versions
/// This endpoint permanently deletes the key metadata and all version data for
/// the specified key.
///
/// * Path: {self.mount}/metadata/{self.path}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/kv/kv-v2#delete-metadata-and-all-versions>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/metadata/{self.path}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteSecretMetadataRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub path: String,
}
