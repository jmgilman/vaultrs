#[macro_use]
extern crate tracing;

mod common;

use common::VaultServerHelper;
use test_env_log::test;
use vaultrs::client::Client;
use vaultrs::{api::token::requests::CreateTokenRequest, error::ClientError, token};
use vaultrs_test::docker::{Server, ServerConfig};
use vaultrs_test::{VaultServer, VaultServerConfig};

#[test]
fn test() {
    let config = VaultServerConfig::default(Some(common::VERSION));
    let instance = config.to_instance();
    instance.run(|ops| async move {
        let server = VaultServer::new(&ops, &config);
        let client = server.client();
        let mut token = setup(&client).await.unwrap();

        // Test token roles
        crate::role::test_set(&client, "test").await;
        crate::role::test_list(&client).await;
        crate::role::test_read(&client, "test").await;
        crate::role::test_delete(&client, "test").await;

        // Test tidy
        test_tidy(&client).await;

        // Test creating tokens
        test_new(&client).await;
        test_new_orphan(&client).await;

        // Test looking up tokens
        test_lookup(&client, token.token.as_str()).await;
        test_lookup_self(&client).await;
        test_lookup_accessor(&client, token.accessor.as_str()).await;

        // Test renewing tokens
        test_renew(&client, token.token.as_str()).await;
        test_renew_self(&client).await;
        test_renew_accessor(&client, token.accessor.as_str()).await;

        // Test revoking tokens
        test_revoke(&client, token.token.as_str()).await;
        token = setup(&client).await.unwrap();
        test_revoke_accessor(&client, token.accessor.as_str()).await;
        token = setup(&client).await.unwrap();
        test_revoke_orphan(&client, token.token.as_str()).await;

        test_revoke_self(&client).await;
    });
}

pub async fn test_lookup(client: &impl Client, token: &str) {
    let resp = token::lookup(client, token).await;
    assert!(resp.is_ok());
}

pub async fn test_lookup_accessor(client: &impl Client, accessor: &str) {
    let resp = token::lookup_accessor(client, accessor).await;
    assert!(resp.is_ok());
}

pub async fn test_lookup_self(client: &impl Client) {
    let resp = token::lookup_self(client).await;
    assert!(resp.is_ok());
}

pub async fn test_new(client: &impl Client) {
    let resp = token::new(client, None).await;
    assert!(resp.is_ok());
}

pub async fn test_new_orphan(client: &impl Client) {
    let resp = token::new_orphan(client, None).await;
    assert!(resp.is_ok());
}

pub async fn test_renew(client: &impl Client, token: &str) {
    let resp = token::renew(client, token, Some("20m")).await;
    assert!(resp.is_ok());
}

pub async fn test_renew_accessor(client: &impl Client, accessor: &str) {
    let resp = token::renew_accessor(client, accessor, Some("20m")).await;
    assert!(resp.is_ok());
}

pub async fn test_renew_self(client: &impl Client) {
    let resp = token::renew_self(client, Some("20m")).await;
    assert!(resp.is_err()); // Cannot renew the root token
    if let ClientError::APIError { code: _, errors } = resp.unwrap_err() {
        assert_eq!(errors[0], "lease is not renewable");
    }
}

pub async fn test_revoke(client: &impl Client, token: &str) {
    let resp = token::revoke(client, token).await;
    assert!(resp.is_ok());
}

pub async fn test_revoke_accessor(client: &impl Client, accessor: &str) {
    let resp = token::revoke_accessor(client, accessor).await;
    assert!(resp.is_ok());
}

pub async fn test_revoke_orphan(client: &impl Client, token: &str) {
    let resp = token::revoke_orphan(client, token).await;
    assert!(resp.is_ok());
}

pub async fn test_revoke_self(client: &impl Client) {
    let resp = token::revoke_self(client).await;
    assert!(resp.is_ok());
}

pub async fn test_tidy(client: &impl Client) {
    let resp = token::tidy(client).await;
    assert!(resp.is_ok());
}

mod role {
    use vaultrs::api::token::requests::SetTokenRoleRequest;

    use super::Client;
    use crate::token::role;

    pub async fn test_delete(client: &impl Client, role_name: &str) {
        let resp = role::delete(client, role_name).await;
        assert!(resp.is_ok());
    }

    pub async fn test_list(client: &impl Client) {
        let resp = role::list(client).await;
        assert!(resp.is_ok());
    }

    pub async fn test_read(client: &impl Client, role_name: &str) {
        let resp = role::read(client, role_name).await;
        assert!(resp.is_ok());
    }

    pub async fn test_set(client: &impl Client, role_name: &str) {
        let resp = role::set(
            client,
            role_name,
            Some(
                SetTokenRoleRequest::builder()
                    .renewable(true)
                    .token_explicit_max_ttl("1h"),
            ),
        )
        .await;
        assert!(resp.is_ok());
    }
}

// TODO: Add test for create token with role

struct Token {
    pub accessor: String,
    pub token: String,
}

async fn setup(client: &impl Client) -> Result<Token, ClientError> {
    debug!("creating new token");

    // Create a new token
    let resp = token::new(
        client,
        Some(
            CreateTokenRequest::builder()
                .ttl("10m")
                .renewable(true)
                .explicit_max_ttl("1h"),
        ),
    )
    .await?;
    Ok(Token {
        accessor: resp.accessor,
        token: resp.client_token,
    })
}
