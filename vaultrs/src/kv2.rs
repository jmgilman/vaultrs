use crate::{
    api::{
        self,
        kv2::{
            requests::{
                DeleteLatestSecretVersionRequest, DeleteSecretMetadataRequest,
                DeleteSecretVersionsRequest, DestroySecretVersionsRequest, ListSecretsRequest,
                ReadSecretMetadataRequest, ReadSecretRequest, SetSecretMetadataRequest,
                SetSecretMetadataRequestBuilder, SetSecretRequest, SetSecretRequestOptions,
                UndeleteSecretVersionsRequest,
            },
            responses::{ReadSecretMetadataResponse, SecretVersionMetadata},
        },
    },
    client::Client,
    error::ClientError,
};
use serde::{de::DeserializeOwned, Serialize};

/// Soft-delete the latest version of a secret
///
/// See [DeleteLatestSecretVersionRequest]
pub async fn delete_latest(
    client: &impl Client,
    mount: &str,
    path: &str,
) -> Result<(), ClientError> {
    let endpoint = DeleteLatestSecretVersionRequest::builder()
        .mount(mount)
        .path(path)
        .build()
        .unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// Delete all metadata and versions of a secret
///
/// See [DeleteSecretMetadataRequest]
pub async fn delete_metadata(
    client: &impl Client,
    mount: &str,
    path: &str,
) -> Result<(), ClientError> {
    let endpoint = DeleteSecretMetadataRequest::builder()
        .mount(mount)
        .path(path)
        .build()
        .unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// Soft-delete specific versions of a secret
///
/// See [DeleteSecretVersionsRequest]
pub async fn delete_versions(
    client: &impl Client,
    mount: &str,
    path: &str,
    versions: Vec<u64>,
) -> Result<(), ClientError> {
    let endpoint = DeleteSecretVersionsRequest::builder()
        .mount(mount)
        .path(path)
        .versions(versions)
        .build()
        .unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// Permanently delete specific versions of a secret
///
/// See [DestroySecretVersionsRequest]
pub async fn destroy_versions(
    client: &impl Client,
    mount: &str,
    path: &str,
    versions: Vec<u64>,
) -> Result<(), ClientError> {
    let endpoint = DestroySecretVersionsRequest::builder()
        .mount(mount)
        .path(path)
        .versions(versions)
        .build()
        .unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// Lists all secret keys at the given path
///
/// See [ListSecretsRequest]
pub async fn list(
    client: &impl Client,
    mount: &str,
    path: &str,
) -> Result<Vec<String>, ClientError> {
    let endpoint = ListSecretsRequest::builder()
        .mount(mount)
        .path(path)
        .build()
        .unwrap();
    Ok(api::exec_with_result(client, endpoint).await?.keys)
}

/// Reads the value of the secret at the given path
///
/// See [ReadSecretRequest]
pub async fn read<D: DeserializeOwned>(
    client: &impl Client,
    mount: &str,
    path: &str,
) -> Result<D, ClientError> {
    let endpoint = ReadSecretRequest::builder()
        .mount(mount)
        .path(path)
        .build()
        .unwrap();
    let res = api::exec_with_result(client, endpoint).await?;
    serde_json::value::from_value(res.data).map_err(|e| ClientError::JsonParseError { source: e })
}

/// Reads the metadata of the secret at the given path
///
/// See [ReadSecretMetadataRequest]
pub async fn read_metadata(
    client: &impl Client,
    mount: &str,
    path: &str,
) -> Result<ReadSecretMetadataResponse, ClientError> {
    let endpoint = ReadSecretMetadataRequest::builder()
        .mount(mount)
        .path(path)
        .build()
        .unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Reads the value of the secret at the given version and path
///
/// See [ReadSecretRequest]
pub async fn read_version<D: DeserializeOwned>(
    client: &impl Client,
    mount: &str,
    path: &str,
    version: u64,
) -> Result<D, ClientError> {
    let endpoint = ReadSecretRequest::builder()
        .mount(mount)
        .path(path)
        .version(Some(version))
        .build()
        .unwrap();
    let res = api::exec_with_result(client, endpoint).await?;
    serde_json::value::from_value(res.data).map_err(|e| ClientError::JsonParseError { source: e })
}

/// Sets the value of the secret at the given path
///
/// See [SetSecretRequest]
pub async fn set<T: Serialize>(
    client: &impl Client,
    mount: &str,
    path: &str,
    data: &T,
) -> Result<SecretVersionMetadata, ClientError> {
    let data_value = data
        .serialize(serde_json::value::Serializer)
        .map_err(|e| ClientError::JsonParseError { source: e })?;
    let endpoint = SetSecretRequest::builder()
        .mount(mount)
        .path(path)
        .data(data_value)
        .build()
        .unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Sets the value of the secret at the given path
/// including an argument for [SetSecretRequestOptions]
///
/// See [SetSecretRequest]
pub async fn set_with_options<T: Serialize>(
    client: &impl Client,
    mount: &str,
    path: &str,
    data: &T,
    options: SetSecretRequestOptions,
) -> Result<SecretVersionMetadata, ClientError> {
    let data_value = data
        .serialize(serde_json::value::Serializer)
        .map_err(|e| ClientError::JsonParseError { source: e })?;
    let endpoint = SetSecretRequest::builder()
        .mount(mount)
        .path(path)
        .data(data_value)
        .options(options)
        .build()
        .unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Sets the value of the secret at the given path
///
/// See [SetSecretMetadataRequest]
pub async fn set_metadata(
    client: &impl Client,
    mount: &str,
    path: &str,
    opts: Option<&mut SetSecretMetadataRequestBuilder>,
) -> Result<(), ClientError> {
    let mut t = SetSecretMetadataRequest::builder();
    let endpoint = opts
        .unwrap_or(&mut t)
        .mount(mount)
        .path(path)
        .build()
        .unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// Undelete specific versions of a secret
///
/// See [UndeleteSecretVersionsRequest]
pub async fn undelete_versions(
    client: &impl Client,
    mount: &str,
    path: &str,
    versions: Vec<u64>,
) -> Result<(), ClientError> {
    let endpoint = UndeleteSecretVersionsRequest::builder()
        .mount(mount)
        .path(path)
        .versions(versions)
        .build()
        .unwrap();
    api::exec_with_empty(client, endpoint).await
}

pub mod config {
    use crate::{
        api::{
            self,
            kv2::{
                requests::{
                    ReadConfigurationRequest, SetConfigurationRequest,
                    SetConfigurationRequestBuilder,
                },
                responses::ReadConfigurationResponse,
            },
        },
        client::Client,
        error::ClientError,
    };

    /// Read the configuration of the mounted KV engine
    ///
    /// See [ReadConfigurationResponse]
    pub async fn read(
        client: &impl Client,
        mount: &str,
    ) -> Result<ReadConfigurationResponse, ClientError> {
        let endpoint = ReadConfigurationRequest::builder()
            .mount(mount)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Update the configuration of the mounted KV engine
    ///
    /// See [SetConfigurationRequest]
    pub async fn set(
        client: &impl Client,
        mount: &str,
        opts: Option<&mut SetConfigurationRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = SetConfigurationRequest::builder();
        let endpoint = opts.unwrap_or(&mut t).mount(mount).build().unwrap();
        api::exec_with_empty(client, endpoint).await
    }
}
