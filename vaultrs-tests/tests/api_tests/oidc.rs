use tracing::debug;
use vaultrs::client::Client;
use vaultrs::error::ClientError;
use vaultrs::sys::auth;

use crate::common::Test;

#[tokio::test]
async fn test() {
    let test = Test::builder().await;
    let client = test.client();
    let endpoint = setup(client).await.unwrap();

    // Test config
    config::test_set(client, &endpoint).await;
    config::test_read(client, &endpoint).await;

    // Test roles
    role::test_set(client, &endpoint).await;
    role::test_read(client, &endpoint).await;
    role::test_list(client, &endpoint).await;

    role::test_delete(client, &endpoint).await;
}

mod config {
    use vaultrs::client::Client;

    use vaultrs::{api::auth::oidc::requests::SetConfigurationRequest, auth::oidc::config};

    use super::OIDCEndpoint;

    pub async fn test_read(client: &impl Client, endpoint: &OIDCEndpoint) {
        config::read(client, endpoint.path.as_str()).await.unwrap();
    }

    pub async fn test_set(client: &impl Client, endpoint: &OIDCEndpoint) {
        // TODO: This might not always work
        config::set(
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
        .await
        .unwrap();
    }
}

mod role {
    use super::{Client, OIDCEndpoint};
    use vaultrs::{api::auth::oidc::requests::SetRoleRequest, auth::oidc::role};

    pub async fn test_delete(client: &impl Client, endpoint: &OIDCEndpoint) {
        role::delete(client, endpoint.path.as_str(), endpoint.role.as_str())
            .await
            .unwrap();
    }

    pub async fn test_list(client: &impl Client, endpoint: &OIDCEndpoint) {
        role::list(client, endpoint.path.as_str()).await.unwrap();
    }

    pub async fn test_read(client: &impl Client, endpoint: &OIDCEndpoint) {
        role::read(client, endpoint.path.as_str(), endpoint.role.as_str())
            .await
            .unwrap();
    }

    pub async fn test_set(client: &impl Client, endpoint: &OIDCEndpoint) {
        role::set(
            client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
            "claim",
            vec!["https://samples.auth0.com/authorize".to_string()],
            Some(&mut SetRoleRequest::builder()),
        )
        .await
        .unwrap();
    }
}

#[derive(Debug)]
pub struct OIDCEndpoint {
    pub path: String,
    pub role: String,
}

async fn setup(client: &impl Client) -> Result<OIDCEndpoint, ClientError> {
    debug!("setting up OIDC auth engine");

    let path = "oidc_test";
    let role = "test";

    // Mount the OIDC auth engine
    auth::enable(client, path, "oidc", None).await.unwrap();
    Ok(OIDCEndpoint {
        path: path.to_string(),
        role: role.to_string(),
    })
}
