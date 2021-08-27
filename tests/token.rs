mod common;

use common::VaultServer;
use vaultrs::{api::token::requests::CreateTokenRequest, error::ClientError, token};

#[tokio::test]
async fn test() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);
    let mut token = setup(&server).await.unwrap();

    // Test token roles
    crate::role::test_set(&server, "test").await;
    crate::role::test_list(&server).await;
    crate::role::test_read(&server, "test").await;
    crate::role::test_delete(&server, "test").await;

    // Test tidy
    test_tidy(&server).await;

    // Test creating tokens
    test_new(&server).await;
    test_new_orphan(&server).await;

    // Test looking up tokens
    test_lookup(&server, token.token.as_str()).await;
    test_lookup_self(&server).await;
    test_lookup_accessor(&server, token.accessor.as_str()).await;

    // Test renewing tokens
    test_renew(&server, token.token.as_str()).await;
    test_renew_self(&server).await;
    test_renew_accessor(&server, token.accessor.as_str()).await;

    // Test revoking tokens
    test_revoke(&server, token.token.as_str()).await;
    token = setup(&server).await.unwrap();
    test_revoke_accessor(&server, token.accessor.as_str()).await;
    token = setup(&server).await.unwrap();
    test_revoke_orphan(&server, token.token.as_str()).await;

    test_revoke_self(&server).await;
}

pub async fn test_lookup(server: &VaultServer<'_>, token: &str) {
    let resp = token::lookup(&server.client, token).await;
    assert!(resp.is_ok());
}

pub async fn test_lookup_accessor(server: &VaultServer<'_>, accessor: &str) {
    let resp = token::lookup_accessor(&server.client, accessor).await;
    assert!(resp.is_ok());
}

pub async fn test_lookup_self(server: &VaultServer<'_>) {
    let resp = token::lookup_self(&server.client).await;
    assert!(resp.is_ok());
}

pub async fn test_new(server: &VaultServer<'_>) {
    let resp = token::new(&server.client, None).await;
    assert!(resp.is_ok());
}

pub async fn test_new_orphan(server: &VaultServer<'_>) {
    let resp = token::new_orphan(&server.client, None).await;
    assert!(resp.is_ok());
}

pub async fn test_renew(server: &VaultServer<'_>, token: &str) {
    let resp = token::renew(&server.client, token, Some("20m")).await;
    assert!(resp.is_ok());
}

pub async fn test_renew_accessor(server: &VaultServer<'_>, accessor: &str) {
    let resp = token::renew_accessor(&server.client, accessor, Some("20m")).await;
    assert!(resp.is_ok());
}

pub async fn test_renew_self(server: &VaultServer<'_>) {
    let resp = token::renew_self(&server.client, Some("20m")).await;
    assert!(resp.is_err()); // Cannot renew the root token
    if let ClientError::APIError { code: _, errors } = resp.unwrap_err() {
        assert_eq!(errors[0], "lease is not renewable");
    }
}

pub async fn test_revoke(server: &VaultServer<'_>, token: &str) {
    let resp = token::revoke(&server.client, token).await;
    assert!(resp.is_ok());
}

pub async fn test_revoke_accessor(server: &VaultServer<'_>, accessor: &str) {
    let resp = token::revoke_accessor(&server.client, accessor).await;
    assert!(resp.is_ok());
}

pub async fn test_revoke_orphan(server: &VaultServer<'_>, token: &str) {
    let resp = token::revoke_orphan(&server.client, token).await;
    assert!(resp.is_ok());
}

pub async fn test_revoke_self(server: &VaultServer<'_>) {
    let resp = token::revoke_self(&server.client).await;
    assert!(resp.is_ok());
}

pub async fn test_tidy(server: &VaultServer<'_>) {
    let resp = token::tidy(&server.client).await;
    assert!(resp.is_ok());
}

mod role {
    use vaultrs::api::token::requests::SetTokenRoleRequest;

    use super::VaultServer;
    use crate::token::role;

    pub async fn test_delete(server: &VaultServer<'_>, role_name: &str) {
        let resp = role::delete(&server.client, role_name).await;
        assert!(resp.is_ok());
    }

    pub async fn test_list(server: &VaultServer<'_>) {
        let resp = role::list(&server.client).await;
        assert!(resp.is_ok());
    }

    pub async fn test_read(server: &VaultServer<'_>, role_name: &str) {
        let resp = role::read(&server.client, role_name).await;
        assert!(resp.is_ok());
    }

    pub async fn test_set(server: &VaultServer<'_>, role_name: &str) {
        let resp = role::set(
            &server.client,
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

async fn setup(server: &VaultServer<'_>) -> Result<Token, ClientError> {
    // Create a new token
    let resp = token::new(
        &server.client,
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
