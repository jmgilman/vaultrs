use reqwest::StatusCode;
use testcontainers::{
    core::{wait::HttpWaitStrategy, ContainerPort, WaitFor},
    runners::AsyncRunner,
    GenericImage, ImageExt,
};
use vaultrs::{
    api::token::requests::CreateTokenRequest,
    client::{VaultClient, VaultClientSettingsBuilder},
    token,
};

use crate::common::{Vault, TESTED_VERSION};

#[tokio::test]
async fn lookup_self_with_auto_auth() {
    for (image, tag) in TESTED_VERSION {
        check_lookup_self(image, tag).await;
    }
}

async fn check_lookup_self(image: &str, tag: &str) {
    let server = Vault::default()
        .with_name(image)
        .with_tag(tag)
        .start()
        .await
        .unwrap();
    let server_address = format!(
        "http://localhost:{}",
        server.get_host_port_ipv4(8200).await.unwrap()
    );
    let root = client(&server_address, "root");
    let credentials = token::new(
        &root,
        Some(
            CreateTokenRequest::builder()
                .policies(vec!["default".to_owned()])
                .ttl("1h")
                .renewable(true),
        ),
    )
    .await
    .unwrap();

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

    // Use a real Agent with a token-file auto-auth method to exercise response
    // redaction independently of the server's authentication backends.
    let server_ip = server.get_bridge_ip_address().await.unwrap();
    let config = format!(
        r#"
vault {{ address = "http://{server_ip}:8200" }}
listener "tcp" {{
  address = "0.0.0.0:8200"
  tls_disable = true
}}
api_proxy {{ use_auto_auth_token = "force" }}
auto_auth {{
  method "token_file" {{
    config = {{ token_file_path = "/tmp/agent-token" }}
  }}
}}
"#
    );
    let agent = GenericImage::new(image, tag)
        .with_entrypoint(if image.contains("openbao") {
            "bao"
        } else {
            "vault"
        })
        .with_exposed_port(ContainerPort::Tcp(8200))
        .with_wait_for(WaitFor::http(
            HttpWaitStrategy::new("/v1/auth/token/lookup-self")
                .with_expected_status_code(StatusCode::OK),
        ))
        .with_copy_to("/tmp/agent.hcl", config.into_bytes())
        .with_copy_to(
            "/tmp/agent-token",
            credentials.client_token.clone().into_bytes(),
        )
        .with_cmd(["agent", "-config=/tmp/agent.hcl"])
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

    // The shared response type must also retain the server's empty-string ID
    // for an accessor lookup, rather than treating it as a missing identifier.
    let response = token::lookup_accessor(&root, &credentials.accessor)
        .await
        .unwrap();
    assert_eq!(response.id.as_deref(), Some(""));
    assert_eq!(
        response.accessor.as_deref(),
        Some(credentials.accessor.as_str())
    );
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
