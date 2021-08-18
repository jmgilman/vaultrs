use crate::{
    api::{
        self,
        kv2::requests::{ReadSecretRequest, SetSecretRequest},
    },
    client::VaultClient,
    error::ClientError,
};
use serde::{de::DeserializeOwned, Serialize};

/// Sets the value of the secret at the given path
///
/// See [SetSecretRequest]
pub fn set<T: Serialize>(
    client: &VaultClient,
    mount: &str,
    path: &str,
    data: &T,
) -> Result<(), ClientError> {
    let data_value =
        data.serialize(serde_json::value::Serializer)
            .map_err(|e| ClientError::JsonParseError {
                source: Box::new(e),
            })?;
    let endpoint = SetSecretRequest::builder()
        .mount(mount)
        .path(path)
        .data(data_value)
        .build()
        .unwrap();
    api::exec_with_empty(client, endpoint)
}

/// Reads the value of the secret at the given path
///
/// See [ReadSecretRequest]
pub fn read<T: DeserializeOwned>(
    client: &VaultClient,
    mount: &str,
    path: &str,
) -> Result<T, ClientError> {
    let endpoint = ReadSecretRequest::builder()
        .mount(mount)
        .path(path)
        .build()
        .unwrap();
    let res = api::exec_with_result(client, endpoint)?;
    serde_json::value::from_value(res.data).map_err(|e| ClientError::JsonParseError {
        source: Box::new(e),
    })
}

/// Reads the value of the secret at the given version and path
///
/// See [ReadSecretRequest]
pub fn read_version<T: DeserializeOwned>(
    client: &VaultClient,
    mount: &str,
    path: &str,
    version: u64,
) -> Result<T, ClientError> {
    let endpoint = ReadSecretRequest::builder()
        .mount(mount)
        .path(path)
        .version(version)
        .build()
        .unwrap();
    let res = api::exec_with_result(client, endpoint)?;
    serde_json::value::from_value(res.data).map_err(|e| ClientError::JsonParseError {
        source: Box::new(e),
    })
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
        client::VaultClient,
        error::ClientError,
    };

    /// Read the configuration of the mounted KV engine
    ///
    /// See [ReadConfigurationResponse]
    pub fn read(
        client: &VaultClient,
        mount: &str,
    ) -> Result<ReadConfigurationResponse, ClientError> {
        let endpoint = ReadConfigurationRequest::builder()
            .mount(mount)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint)
    }

    /// Update the configuration of the mounted KV engine
    ///
    /// See [SetConfigurationRequest]
    pub fn set(
        client: &VaultClient,
        mount: &str,
        opts: Option<&mut SetConfigurationRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = SetConfigurationRequest::builder();
        let endpoint = opts.unwrap_or(&mut t).mount(mount).build().unwrap();
        api::exec_with_empty(client, endpoint)
    }
}
