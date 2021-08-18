mod common;

use common::VaultServer;
use serde::{Deserialize, Serialize};
use vaultrs::api::kv2::requests::SetSecretMetadataRequest;
use vaultrs::error::ClientError;
use vaultrs::kv2;

#[test]
fn test() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);
    let endpoint = setup(&server).unwrap();

    // Test set / read
    test_list(&server, &endpoint);
    test_read(&server, &endpoint);
    test_read_metadata(&server, &endpoint);
    test_read_version(&server, &endpoint);
    test_set(&server, &endpoint);
    test_set_metadata(&server, &endpoint);

    // Test delete
    test_delete_latest(&server, &endpoint);
    test_undelete_versions(&server, &endpoint);

    test_delete_versions(&server, &endpoint);
    create(&server, &endpoint).unwrap();

    test_destroy_versions(&server, &endpoint);
    create(&server, &endpoint).unwrap();

    test_delete_metadata(&server, &endpoint);
    create(&server, &endpoint).unwrap();

    // Test config
    crate::config::test_set(&server, &endpoint);
    crate::config::test_read(&server, &endpoint);
}

fn test_delete_latest(server: &VaultServer, endpoint: &SecretEndpoint) {
    let res = kv2::delete_latest(
        &server.client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
    );
    assert!(res.is_ok());
}

fn test_delete_metadata(server: &VaultServer, endpoint: &SecretEndpoint) {
    let res = kv2::delete_metadata(
        &server.client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
    );
    assert!(res.is_ok());
}

fn test_delete_versions(server: &VaultServer, endpoint: &SecretEndpoint) {
    let res = kv2::delete_versions(
        &server.client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
        vec![1],
    );
    assert!(res.is_ok());
}

fn test_destroy_versions(server: &VaultServer, endpoint: &SecretEndpoint) {
    let res = kv2::destroy_versions(
        &server.client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
        vec![1],
    );
    assert!(res.is_ok());
}

fn test_list(server: &VaultServer, endpoint: &SecretEndpoint) {
    let res = kv2::list(&server.client, endpoint.path.as_str(), "");
    assert!(res.is_ok());
    assert!(!res.unwrap().is_empty());
}

fn test_read(server: &VaultServer, endpoint: &SecretEndpoint) {
    let res = kv2::read::<TestSecret>(&server.client, endpoint.path.as_str(), "test");
    assert!(res.is_ok());
    assert_eq!(res.unwrap().key, endpoint.secret.key);
}

fn test_read_metadata(server: &VaultServer, endpoint: &SecretEndpoint) {
    let res = kv2::read_metadata(
        &server.client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
    );
    assert!(res.is_ok());
    assert!(!res.unwrap().versions.is_empty());
}

fn test_read_version(server: &VaultServer, endpoint: &SecretEndpoint) {
    let res = kv2::read_version::<TestSecret>(&server.client, endpoint.path.as_str(), "test", 1);
    assert!(res.is_ok());
    assert_eq!(res.unwrap().key, endpoint.secret.key);
}

fn test_set(server: &VaultServer, endpoint: &SecretEndpoint) {
    let res = kv2::set(
        &server.client,
        endpoint.path.as_str(),
        "test",
        &endpoint.secret,
    );
    assert!(res.is_ok());
}

fn test_set_metadata(server: &VaultServer, endpoint: &SecretEndpoint) {
    let res = kv2::set_metadata(
        &server.client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
        Some(SetSecretMetadataRequest::builder().delete_version_after("1h")),
    );
    assert!(res.is_ok());
}

fn test_undelete_versions(server: &VaultServer, endpoint: &SecretEndpoint) {
    let res = kv2::undelete_versions(
        &server.client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
        vec![1],
    );
    assert!(res.is_ok());
}

mod config {
    use crate::{SecretEndpoint, VaultServer};
    use vaultrs::{api::kv2::requests::SetConfigurationRequest, kv2::config};

    pub fn test_read(server: &VaultServer, endpoint: &SecretEndpoint) {
        let resp = config::read(&server.client, endpoint.path.as_str());

        assert!(resp.is_ok());
    }

    pub fn test_set(server: &VaultServer, endpoint: &SecretEndpoint) {
        let versions: u64 = 100;
        let resp = config::set(
            &server.client,
            endpoint.path.as_str(),
            Some(
                SetConfigurationRequest::builder()
                    .max_versions(versions)
                    .delete_version_after("768h"),
            ),
        );

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

fn create(server: &VaultServer, endpoint: &SecretEndpoint) -> Result<(), ClientError> {
    kv2::set(
        &server.client,
        endpoint.path.as_str(),
        endpoint.name.as_str(),
        &endpoint.secret,
    )?;
    Ok(())
}

fn setup(server: &VaultServer) -> Result<SecretEndpoint, ClientError> {
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
    server.mount(path, "kv-v2")?;

    // Create a test secret
    create(server, &endpoint)?;

    Ok(endpoint)
}
