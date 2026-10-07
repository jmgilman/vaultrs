use testcontainers::runners::AsyncRunner;
use vaultrs::{
    api::token::requests::CreateTokenRequest,
    client::{Client, VaultClient, VaultClientSettingsBuilder},
    token,
};

use crate::common::{Agent, TestBuilder};

#[tokio::test]
async fn lookup_self_with_auto_auth() {
    TestBuilder::new()
        .check(|test| async move {
            let root = test.client();

            // Create a renewable token that the Agent will use as its auto-auth token.
            let credentials = token::new(
                root,
                Some(
                    CreateTokenRequest::builder()
                        .policies(vec!["default".to_owned()])
                        .ttl("1h")
                        .renewable(true),
                ),
            )
            .await
            .unwrap();

            // Direct lookup against the server keeps both identifiers.
            let server_address = root.settings().address.as_str().to_owned();
            let direct = client(&server_address, &credentials.client_token);
            let response = token::lookup_self(&direct).await.unwrap();
            assert_eq!(
                response.id.as_deref(),
                Some(credentials.client_token.as_str())
            );
            assert_eq!(
                response.accessor.as_deref(),
                Some(credentials.accessor.as_str())
            );

            // Start a real Agent with a token-file auto-auth method to exercise
            // response redaction independently of the server's auth backends.
            let agent = Agent::new(
                test.image_name(),
                test.image_tag(),
                format!("http://{}:8200", test.vault_bridge_ip().await),
                credentials.client_token.clone(),
            )
            .start()
            .await
            .unwrap();
            let agent_address = format!(
                "http://localhost:{}",
                agent.get_host_port_ipv4(8200).await.unwrap()
            );
            let proxied = client(&agent_address, "agent-placeholder");
            let response = token::lookup_self(&proxied).await.unwrap();
            assert!(response.id.is_none());
            assert!(response.accessor.is_none());
            assert!(response.ttl > 0);
            assert_eq!(response.renewable, Some(true));
            assert_eq!(response.policies, vec!["default"]);

            // The shared response type must also retain the server's empty-string
            // ID for an accessor lookup, rather than treating it as missing.
            let response = token::lookup_accessor(root, &credentials.accessor)
                .await
                .unwrap();
            assert_eq!(response.id.as_deref(), Some(""));
            assert_eq!(
                response.accessor.as_deref(),
                Some(credentials.accessor.as_str())
            );
        })
        .await;
}

fn client(address: &str, token: &str) -> VaultClient {
    VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(address)
            .token(token)
            .build()
            .unwrap(),
    )
    .unwrap()
}
