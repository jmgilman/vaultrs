#[macro_use]
extern crate tracing;

mod common;

use common::{VaultServer, VaultServerHelper};
use test_log::test;
use vaultrs::client::Client;
use vaultrs::error::ClientError;

#[test]
fn test() {
    let test = common::new_test();
    test.run(|instance| async move {
        let server: VaultServer = instance.server();
        let client = server.client();
        let endpoint = setup(&server, &client).await.unwrap();

        // Test config
        crate::config::test_set(&client, &endpoint).await;
        crate::config::test_read(&client, &endpoint).await;

        // Test roles
        crate::role::test_set(&client, &endpoint).await;
        crate::role::test_read(&client, &endpoint).await;
        crate::role::test_list(&client, &endpoint).await;

        crate::role::test_delete(&client, &endpoint).await;
    });
}

mod config {
    use crate::{Client, OIDCEndpoint};
    use vaultrs::{api::auth::oidc::requests::SetConfigurationRequest, auth::oidc::config};

    pub async fn test_read(client: &impl Client, endpoint: &OIDCEndpoint) {
        let resp = config::read(client, endpoint.path.as_str()).await;

        assert!(resp.is_ok());
    }

    pub async fn test_set(client: &impl Client, endpoint: &OIDCEndpoint) {
        // TODO: This might not always work
        let resp = config::set(
            client,
            endpoint.path.as_str(),
            Some(
                SetConfigurationRequest::builder()
                    .oidc_discovery_url("https://samples.auth0.com/")
                    .oidc_client_id("kbyuFDidLLm280LIwVFiazOqjO3ty8KH")
                    .oidc_client_secret(
                        "60Op4HFM0I8ajz0WdiStAbziZ-VFQttXuxixHHs2R7r7-CW8GR79l-mmLqMhc-Sa",
                    ),
            ),
        )
        .await;
        assert!(resp.is_ok());
    }
}

mod role {
    use super::{Client, OIDCEndpoint};
    use vaultrs::{api::auth::oidc::requests::SetRoleRequest, auth::oidc::role};

    pub async fn test_delete(client: &impl Client, endpoint: &OIDCEndpoint) {
        let res = role::delete(client, endpoint.path.as_str(), endpoint.role.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_list(client: &impl Client, endpoint: &OIDCEndpoint) {
        let res = role::list(client, endpoint.path.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_read(client: &impl Client, endpoint: &OIDCEndpoint) {
        let res = role::read(client, endpoint.path.as_str(), endpoint.role.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_set(client: &impl Client, endpoint: &OIDCEndpoint) {
        let res = role::set(
            client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
            "claim",
            vec!["https://samples.auth0.com/authorize".to_string()],
            Some(&mut SetRoleRequest::builder()),
        )
        .await;
        assert!(res.is_ok());
    }
}

#[derive(Debug)]
pub struct OIDCEndpoint {
    pub path: String,
    pub role: String,
}

async fn setup(server: &VaultServer, client: &impl Client) -> Result<OIDCEndpoint, ClientError> {
    debug!("setting up OIDC auth engine");

    let path = "oidc_test";
    let role = "test";

    // Mount the OIDC auth engine
    server.mount_auth(client, path, "oidc").await?;

    Ok(OIDCEndpoint {
        path: path.to_string(),
        role: role.to_string(),
    })
}
