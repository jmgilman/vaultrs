use tracing::debug;
use vaultrs::api::auth::cert::requests::ConfigureTlsCertificateMethodBuilder;
use vaultrs::auth::cert::{self};
use vaultrs::client::Client;
use vaultrs::sys::auth;

use crate::common::Test;

#[tokio::test]
async fn test() {
    let test = Test::new_tls().await;
    let client = test.client();
    let ca_cert = test.ca_cert().unwrap();

    let endpoint = setup(client).await;

    // Test CA cert role
    ca_cert_role::test_set(client, &endpoint, ca_cert).await;
    ca_cert_role::test_read(client, &endpoint).await;
    ca_cert_role::test_list(client, &endpoint).await;

    // Test login
    test_login(client, &endpoint).await;

    test_configure(client, &endpoint).await;

    // Test delete
    ca_cert_role::test_delete(client, &endpoint).await;
}

pub async fn test_login(client: &impl Client, endpoint: &CertEndpoint) {
    cert::login(client, endpoint.path.as_str(), endpoint.name.as_str())
        .await
        .unwrap();
}

pub async fn test_configure(client: &impl Client, endpoint: &CertEndpoint) {
    cert::configure_tls_certificate_method(
        client,
        endpoint.path.as_str(),
        Some(
            &mut ConfigureTlsCertificateMethodBuilder::default()
                .enable_identity_alias_metadata(true),
        ),
    )
    .await
    .unwrap();
    let login = cert::login(client, endpoint.path.as_str(), endpoint.name.as_str())
        .await
        .unwrap();
    let entity = vaultrs::identity::entity::read_by_id(client, &login.entity_id)
        .await
        .unwrap();
    // FIXME: When we will bump the tested vault to a newer version, we will need to update this assert.
    assert!(entity.metadata.is_none());
}

pub mod ca_cert_role {
    use std::{fs, path::Path};

    use vaultrs::{auth::cert::ca_cert_role, client::Client};

    use super::CertEndpoint;

    pub async fn test_delete(client: &impl Client, endpoint: &CertEndpoint) {
        ca_cert_role::delete(client, endpoint.path.as_str(), endpoint.name.as_str())
            .await
            .unwrap();
    }

    pub async fn test_list(client: &impl Client, endpoint: &CertEndpoint) {
        ca_cert_role::list(client, endpoint.path.as_str())
            .await
            .unwrap();
    }

    pub async fn test_read(client: &impl Client, endpoint: &CertEndpoint) {
        ca_cert_role::read(client, endpoint.path.as_str(), endpoint.name.as_str())
            .await
            .unwrap();
    }

    pub async fn test_set(client: &impl Client, endpoint: &CertEndpoint, ca_cert: &Path) {
        let client_crt = fs::read_to_string(ca_cert).unwrap();

        ca_cert_role::set(
            client,
            endpoint.path.as_str(),
            endpoint.name.as_str(),
            &client_crt,
            None,
        )
        .await
        .unwrap();
    }
}

#[derive(Debug)]
pub struct CertEndpoint {
    pub path: String,
    pub name: String,
}

async fn setup(client: &impl Client) -> CertEndpoint {
    debug!("setting up cert auth engine");

    let path = "cert_test";
    let name = "test";

    // Mount the cert auth engine
    auth::enable(client, path, "cert", None).await.unwrap();

    CertEndpoint {
        path: path.to_string(),
        name: name.to_string(),
    }
}
