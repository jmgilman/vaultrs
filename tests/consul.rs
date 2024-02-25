#[macro_use]
extern crate tracing;

mod common;

use common::{ConsulServer, VaultServer, VaultServerHelper};
use test_log::test;
use vaultrs::api::consul::requests::SetAccessConfigRequest;
use vaultrs::client::Client;
use vaultrs::error::ClientError;

#[test]
fn test() {
    let test = common::new_consul_test();
    test.run(|instance| async move {
        let consul_server: ConsulServer = instance.server();
        let vault_server: VaultServer = instance.server();
        let client = vault_server.client();
        let endpoint = setup(&consul_server, &vault_server, &client).await.unwrap();

        // Test roles
        crate::role::test_set(&client, &endpoint).await;
        crate::role::test_read(&client, &endpoint).await;
        crate::role::test_list(&client, &endpoint).await;
        crate::role::test_delete(&client, &endpoint).await;
    });
}

mod role {
    use super::{Client, ConsulEndpoint};
    use vaultrs::{api::consul::requests::SetRoleRequest, consul::role};

    pub async fn test_delete(client: &impl Client, endpoint: &ConsulEndpoint) {
        let res = role::delete(client, endpoint.path.as_str(), endpoint.role.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_list(client: &impl Client, endpoint: &ConsulEndpoint) {
        let res = role::list(client, endpoint.path.as_str()).await;
        assert!(res.is_ok());
        assert!(!res.unwrap().keys.is_empty());
    }

    pub async fn test_read(client: &impl Client, endpoint: &ConsulEndpoint) {
        let res = role::read(client, endpoint.path.as_str(), endpoint.role.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_set(client: &impl Client, endpoint: &ConsulEndpoint) {
        let policies = vec!["global-management".to_string()];
        let res = role::set(
            client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
            Some(SetRoleRequest::builder().policies(policies)),
        )
        .await;
        assert!(res.is_ok());
    }
}

#[derive(Debug)]
pub struct ConsulEndpoint {
    pub connection: String,
    pub path: String,
    pub role: String,
}

async fn setup(
    consul_server: &ConsulServer,
    vault_server: &VaultServer,
    client: &impl Client,
) -> Result<ConsulEndpoint, ClientError> {
    debug!("setting up consul secret engine");

    let connection = "consul";
    let path = "consul_test";
    let role = "test";

    // Mount the database secret engine
    vault_server.mount_secret(client, path, "consul").await?;
    vaultrs::consul::config::set(
        client,
        path,
        Some(
            SetAccessConfigRequest::builder()
                .address(consul_server.internal_address())
                .token("test".to_string()),
        ),
    )
    .await?;

    Ok(ConsulEndpoint {
        connection: connection.to_string(),
        path: path.to_string(),
        role: role.to_string(),
    })
}
