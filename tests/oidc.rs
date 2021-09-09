pub const VERSION: &str = "1.8.2";

use vaultrs::error::ClientError;
use vaultrs_test::docker::{Server, ServerConfig};
use vaultrs_test::{VaultServer, VaultServerConfig};

#[test]
fn test() {
    let config = VaultServerConfig::default(Some(VERSION));
    let instance = config.to_instance();

    instance.run(|ops| async move {
        let server = VaultServer::new(&ops, &config);
        let endpoint = setup(&server).await.unwrap();

        // Test config
        crate::config::test_set(&server, &endpoint).await;
        crate::config::test_read(&server, &endpoint).await;

        // Test roles
        crate::role::test_set(&server, &endpoint).await;
        crate::role::test_read(&server, &endpoint).await;
        crate::role::test_list(&server, &endpoint).await;

        crate::role::test_delete(&server, &endpoint).await;
    });
}

mod config {
    use crate::{OIDCEndpoint, VaultServer};
    use vaultrs::{api::auth::oidc::requests::SetConfigurationRequest, auth::oidc::config};

    pub async fn test_read(server: &VaultServer, endpoint: &OIDCEndpoint) {
        let resp = config::read(&server.client, endpoint.path.as_str()).await;

        assert!(resp.is_ok());
    }

    pub async fn test_set(server: &VaultServer, endpoint: &OIDCEndpoint) {
        // TODO: This might not always work
        let resp = config::set(
            &server.client,
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
    use super::{OIDCEndpoint, VaultServer};
    use vaultrs::{api::auth::oidc::requests::SetRoleRequest, auth::oidc::role};

    pub async fn test_delete(server: &VaultServer, endpoint: &OIDCEndpoint) {
        let res = role::delete(
            &server.client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
        )
        .await;
        assert!(res.is_ok());
    }

    pub async fn test_list(server: &VaultServer, endpoint: &OIDCEndpoint) {
        let res = role::list(&server.client, endpoint.path.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_read(server: &VaultServer, endpoint: &OIDCEndpoint) {
        let res = role::read(
            &server.client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
        )
        .await;
        assert!(res.is_ok());
    }

    pub async fn test_set(server: &VaultServer, endpoint: &OIDCEndpoint) {
        let res = role::set(
            &server.client,
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

async fn setup(server: &VaultServer) -> Result<OIDCEndpoint, ClientError> {
    let path = "oidc_test";
    let role = "test";

    // Mount the OIDC auth engine
    server.mount_auth(path, "oidc").await?;

    Ok(OIDCEndpoint {
        path: path.to_string(),
        role: role.to_string(),
    })
}
