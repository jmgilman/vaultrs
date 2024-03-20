#[macro_use]
extern crate tracing;

mod common;

use common::{VaultServer, VaultServerHelper};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use test_log::test;
use vaultrs::api::kv2::requests::{SetSecretMetadataRequest, SetSecretRequestOptions};
use vaultrs::client::Client;
use vaultrs::error::ClientError;
use vaultrs::kv2;

#[test]
fn test() {
    let test = common::new_test();
    test.run(|instance| async move {
        let server: VaultServer = instance.server();
        let client = server.client();
        let endpoint = setup(&server, &client).await.unwrap();

        // Test set / read
        test_list(&client, &endpoint).await;
        test_read(&client, &endpoint).await;
        test_read_version(&client, &endpoint).await;
        test_set(&client, &endpoint).await;
        test_set_with_compare_and_swap(&client, &endpoint).await;
        test_set_metadata(&client, &endpoint).await;
        test_read_metadata(&client, &endpoint).await;

        // Test delete
        test_delete_latest(&client, &endpoint).await;
        test_undelete_versions(&client, &endpoint).await;

        test_delete_versions(&client, &endpoint).await;
        create(&client, &endpoint).await.unwrap();

        test_destroy_versions(&client, &endpoint).await;
        create(&client, &endpoint).await.unwrap();

        test_delete_metadata(&client, &endpoint).await;
        create(&client, &endpoint).await.unwrap();

        // Test config
        crate::config::test_set(&client, &endpoint).await;
        crate::config::test_read(&client, &endpoint).await;

        // Test URL encoding works as expected
        test_kv2_url_encoding(&server).await;
    });
}

async fn test_kv2_url_encoding(server: &VaultServer) {
    let client = server.client();

    debug!("setting up kv2 auth engine");
    let path = "path/to/secret engine";
    let name = "path/to/some secret/password name with whitespace";
    let secret = TestSecret {
        key: "mykey".to_string(),
        password: "supersecret".to_string(),
    };
    let endpoint = SecretEndpoint {
        path: path.to_string(),
        name: name.to_string(),
        secret,
    };

    // Mount the KV2 engine
    server.mount_secret(&client, path, "kv-v2").await.unwrap();

    // Create a test secret
    create(&client, &endpoint).await.unwrap();

    let secrets = kv2::list(&client, path, "path/to/some secret/")
        .await
        .unwrap();
    assert_eq!(secrets.len(), 1);
    assert_eq!(secrets.first().unwrap(), "password name with whitespace");

    let res: Result<TestSecret, _> = kv2::read(&client, path, name).await;
    assert!(res.is_ok());
    assert_eq!(res.unwrap().key, endpoint.secret.key);
}

async fn test_delete_latest(client: &impl Client, endpoint: &SecretEndpoint) {
    let res = kv2::delete_latest(client, endpoint.path.as_str(), endpoint.name.as_str()).await;
    assert!(res.is_ok());
}

async fn test_delete_metadata(client: &impl Client, endpoint: &SecretEndpoint) {
    let res = kv2::delete_metadata(client, endpoint.path.as_str(), endpoint.name.as_str()).await;
    assert!(res.is_ok());
}

async fn test_delete_versions(client: &impl Client, endpoint: &SecretEndpoint) {
    let res = kv2::delete_versions(
        client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
        vec![1],
    )
    .await;
    assert!(res.is_ok());
}

async fn test_destroy_versions(client: &impl Client, endpoint: &SecretEndpoint) {
    let res = kv2::destroy_versions(
        client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
        vec![1],
    )
    .await;
    assert!(res.is_ok());
}

async fn test_list(client: &impl Client, endpoint: &SecretEndpoint) {
    let res = kv2::list(client, endpoint.path.as_str(), "").await;
    assert!(res.is_ok());
    assert!(!res.unwrap().is_empty());
}

async fn test_read(client: &impl Client, endpoint: &SecretEndpoint) {
    let res: Result<TestSecret, _> = kv2::read(client, endpoint.path.as_str(), "test").await;
    assert!(res.is_ok());
    assert_eq!(res.unwrap().key, endpoint.secret.key);
}

async fn test_read_metadata(client: &impl Client, endpoint: &SecretEndpoint) {
    let res = kv2::read_metadata(client, endpoint.path.as_str(), endpoint.name.as_str()).await;
    assert!(res.is_ok());
    let response = res.unwrap();
    assert!(!response.versions.is_empty());
    assert!(!response.custom_metadata.unwrap().is_empty());
}

async fn test_read_version(client: &impl Client, endpoint: &SecretEndpoint) {
    let res: Result<TestSecret, _> =
        kv2::read_version(client, endpoint.path.as_str(), "test", 1).await;
    assert!(res.is_ok());
    assert_eq!(res.unwrap().key, endpoint.secret.key);
}

async fn test_set(client: &impl Client, endpoint: &SecretEndpoint) {
    let res = kv2::set(client, endpoint.path.as_str(), "test", &endpoint.secret).await;
    assert!(res.is_ok());
}

async fn test_set_with_compare_and_swap(client: &impl Client, endpoint: &SecretEndpoint) {
    let res = kv2::set_with_options(
        client,
        endpoint.path.as_str(),
        "test-compare-and-swap",
        &endpoint.secret,
        SetSecretRequestOptions { cas: 0 },
    )
    .await;
    assert!(res.is_ok());
    let res = kv2::set_with_options(
        client,
        endpoint.path.as_str(),
        "test-compare-and-swap",
        &endpoint.secret,
        SetSecretRequestOptions { cas: 0 },
    )
    .await;
    assert!(res.is_err());
}

async fn test_set_metadata(client: &impl Client, endpoint: &SecretEndpoint) {
    let res = kv2::set_metadata(
        client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
        Some(
            SetSecretMetadataRequest::builder()
                .delete_version_after("1h")
                .custom_metadata(HashMap::from([
                    ("key1".to_string(), "foo".to_string()),
                    ("key2".to_string(), "bar".to_string()),
                ])),
        ),
    )
    .await;
    assert!(res.is_ok());
}

async fn test_undelete_versions(client: &impl Client, endpoint: &SecretEndpoint) {
    let res = kv2::undelete_versions(
        client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
        vec![1],
    )
    .await;
    assert!(res.is_ok());
}

mod config {
    use crate::{Client, SecretEndpoint};
    use vaultrs::{api::kv2::requests::SetConfigurationRequest, kv2::config};

    pub async fn test_read(client: &impl Client, endpoint: &SecretEndpoint) {
        let resp = config::read(client, endpoint.path.as_str()).await;

        assert!(resp.is_ok());
    }

    pub async fn test_set(client: &impl Client, endpoint: &SecretEndpoint) {
        let versions: u64 = 100;
        let resp = config::set(
            client,
            endpoint.path.as_str(),
            Some(
                SetConfigurationRequest::builder()
                    .max_versions(versions)
                    .delete_version_after("768h"),
            ),
        )
        .await;

        assert!(resp.is_ok());
    }
}

#[derive(Debug)]
pub struct SecretEndpoint {
    pub path: String,
    pub name: String,
    pub secret: TestSecret,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TestSecret {
    key: String,
    password: String,
}

async fn create(client: &impl Client, endpoint: &SecretEndpoint) -> Result<(), ClientError> {
    trace!(?endpoint, "creating kv2 secret");
    kv2::set(
        client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
        &endpoint.secret,
    )
    .await?;
    Ok(())
}

async fn setup(server: &VaultServer, client: &impl Client) -> Result<SecretEndpoint, ClientError> {
    debug!("setting up kv2 auth engine");
    let path = "secret_test";
    let name = "test";
    let secret = TestSecret {
        key: "mykey".to_string(),
        password: "supersecret".to_string(),
    };
    let endpoint = SecretEndpoint {
        path: path.to_string(),
        name: name.to_string(),
        secret,
    };

    // Mount the KV2 engine
    server.mount_secret(client, path, "kv-v2").await?;

    // Create a test secret
    create(client, &endpoint).await?;

    Ok(endpoint)
}
