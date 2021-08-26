mod common;

use common::VaultServer;
use serde::{Deserialize, Serialize};
use vaultrs::api::kv2::requests::SetSecretMetadataRequest;
use vaultrs::error::ClientError;
use vaultrs::kv2;

#[tokio::test]
async fn test() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);
    let endpoint = setup(&server).await.unwrap();

    // Test set / read
    test_list(&server, &endpoint).await;
    test_read(&server, &endpoint).await;
    test_read_metadata(&server, &endpoint).await;
    test_read_version(&server, &endpoint).await;
    test_set(&server, &endpoint).await;
    test_set_metadata(&server, &endpoint).await;

    // Test delete
    test_delete_latest(&server, &endpoint).await;
    test_undelete_versions(&server, &endpoint).await;

    test_delete_versions(&server, &endpoint).await;
    create(&server, &endpoint).await.unwrap();

    test_destroy_versions(&server, &endpoint).await;
    create(&server, &endpoint).await.unwrap();

    test_delete_metadata(&server, &endpoint).await;
    create(&server, &endpoint).await.unwrap();

    // Test config
    crate::config::test_set(&server, &endpoint).await;
    crate::config::test_read(&server, &endpoint).await;
}

async fn test_delete_latest(server: &VaultServer<'_>, endpoint: &SecretEndpoint) {
    let res = kv2::delete_latest(
        &server.client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
    )
    .await;
    assert!(res.is_ok());
}

async fn test_delete_metadata(server: &VaultServer<'_>, endpoint: &SecretEndpoint) {
    let res = kv2::delete_metadata(
        &server.client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
    )
    .await;
    assert!(res.is_ok());
}

async fn test_delete_versions(server: &VaultServer<'_>, endpoint: &SecretEndpoint) {
    let res = kv2::delete_versions(
        &server.client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
        vec![1],
    )
    .await;
    assert!(res.is_ok());
}

async fn test_destroy_versions(server: &VaultServer<'_>, endpoint: &SecretEndpoint) {
    let res = kv2::destroy_versions(
        &server.client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
        vec![1],
    )
    .await;
    assert!(res.is_ok());
}

async fn test_list(server: &VaultServer<'_>, endpoint: &SecretEndpoint) {
    let res = kv2::list(&server.client, endpoint.path.as_str(), "").await;
    assert!(res.is_ok());
    assert!(!res.unwrap().is_empty());
}

async fn test_read(server: &VaultServer<'_>, endpoint: &SecretEndpoint) {
    let res = kv2::read::<TestSecret>(&server.client, endpoint.path.as_str(), "test").await;
    assert!(res.is_ok());
    assert_eq!(res.unwrap().key, endpoint.secret.key);
}

async fn test_read_metadata(server: &VaultServer<'_>, endpoint: &SecretEndpoint) {
    let res = kv2::read_metadata(
        &server.client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
    )
    .await;
    assert!(res.is_ok());
    assert!(!res.unwrap().versions.is_empty());
}

async fn test_read_version(server: &VaultServer<'_>, endpoint: &SecretEndpoint) {
    let res =
        kv2::read_version::<TestSecret>(&server.client, endpoint.path.as_str(), "test", 1).await;
    assert!(res.is_ok());
    assert_eq!(res.unwrap().key, endpoint.secret.key);
}

async fn test_set(server: &VaultServer<'_>, endpoint: &SecretEndpoint) {
    let res = kv2::set(
        &server.client,
        endpoint.path.as_str(),
        "test",
        &endpoint.secret,
    )
    .await;
    assert!(res.is_ok());
}

async fn test_set_metadata(server: &VaultServer<'_>, endpoint: &SecretEndpoint) {
    let res = kv2::set_metadata(
        &server.client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
        Some(SetSecretMetadataRequest::builder().delete_version_after("1h")),
    )
    .await;
    assert!(res.is_ok());
}

async fn test_undelete_versions(server: &VaultServer<'_>, endpoint: &SecretEndpoint) {
    let res = kv2::undelete_versions(
        &server.client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
        vec![1],
    )
    .await;
    assert!(res.is_ok());
}

mod config {
    use crate::{SecretEndpoint, VaultServer};
    use vaultrs::{api::kv2::requests::SetConfigurationRequest, kv2::config};

    pub async fn test_read(server: &VaultServer<'_>, endpoint: &SecretEndpoint) {
        let resp = config::read(&server.client, endpoint.path.as_str()).await;

        assert!(resp.is_ok());
    }

    pub async fn test_set(server: &VaultServer<'_>, endpoint: &SecretEndpoint) {
        let versions: u64 = 100;
        let resp = config::set(
            &server.client,
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

async fn create(server: &VaultServer<'_>, endpoint: &SecretEndpoint) -> Result<(), ClientError> {
    kv2::set(
        &server.client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
        &endpoint.secret,
    )
    .await?;
    Ok(())
}

async fn setup(server: &VaultServer<'_>) -> Result<SecretEndpoint, ClientError> {
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

    // Mount the PKI engine
    server.mount(path, "kv-v2").await?;

    // Create a test secret
    create(server, &endpoint).await?;

    Ok(endpoint)
}
