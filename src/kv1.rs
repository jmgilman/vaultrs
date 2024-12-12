use crate::{
    api::{
        self,
        kv1::{
            requests::{
                DeleteSecretRequest, GetSecretRequest, ListSecretRequest, SetSecretRequest,
            },
            responses::{GetSecretResponse, ListSecretResponse},
        },
    },
    client::Client,
    error::ClientError,
};

use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;

/// Sets the value of the secret at the given path
///
/// A key called ttl will trigger some special behavior. See the [Vault KV secrets engine documentation][<https://developer.hashicorp.com/vault/docs/secrets/kv>] for details.
/// See [SetSecretRequest]
pub async fn set<T: Serialize>(
    client: &impl Client,
    mount: &str,
    path: &str,
    data: &HashMap<&str, T>,
) -> Result<(), ClientError> {
    let data_value_json = data
        .serialize(serde_json::value::Serializer)
        .map_err(|e| ClientError::JsonParseError { source: e })?;

    // Convert our JSON values as bytes to be sent to Vault
    let data_u8 = serde_json::to_vec(&data_value_json)
        .map_err(|e| ClientError::JsonParseError { source: e })?;

    let endpoint = SetSecretRequest::builder()
        .mount(mount)
        .path(path)
        .data(data_u8)
        .build()
        .unwrap();

    api::exec_with_empty(client, endpoint).await
}

/// Get value of the secret at given path.
/// Return the deserialized HashMap of secret directly,
/// if you need to access additional fields such as lead_duration, use [get_raw]
pub async fn get<D: DeserializeOwned>(
    client: &impl Client,
    mount: &str,
    path: &str,
) -> Result<D, ClientError> {
    // let endpoint = GetSecretRequest::builder()
    //     .mount(mount)
    //     .path(path)
    //     .build()
    //     .unwrap();

    // let res = api::exec_with_no_result(client, endpoint).await?;
    let res = get_raw(client, mount, path).await?;
    serde_json::value::from_value(res.data).map_err(|e| ClientError::JsonParseError { source: e })
}

/// Get value of the secret at given path, returning the raw response without deserialization
/// Additional fields are available on raw response, such as lease_duration
pub async fn get_raw(
    client: &impl Client,
    mount: &str,
    path: &str,
) -> Result<GetSecretResponse, ClientError> {
    let endpoint = GetSecretRequest::builder()
        .mount(mount)
        .path(path)
        .build()
        .unwrap();

    api::exec_with_no_result(client, endpoint).await
}

/// List secret keys at given location, returning raw server response
///
/// See [ListSecretRequest]
pub async fn list(
    client: &impl Client,
    mount: &str,
    path: &str,
) -> Result<ListSecretResponse, ClientError> {
    let endpoint = ListSecretRequest::builder()
        .mount(mount)
        .path(path)
        .build()
        .unwrap();

    api::exec_with_no_result(client, endpoint).await
}

/// Delete secret at given location
///
/// See [DeleteSecretRequest]
pub async fn delete(client: &impl Client, mount: &str, path: &str) -> Result<(), ClientError> {
    let endpoint = DeleteSecretRequest::builder()
        .mount(mount)
        .path(path)
        .build()
        .unwrap();

    api::exec_with_empty(client, endpoint).await
}
