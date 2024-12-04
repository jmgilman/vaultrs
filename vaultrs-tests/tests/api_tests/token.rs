use crate::common::Test;
use tracing::debug;
use vaultrs::client::Client;
use vaultrs::{api::token::requests::CreateTokenRequest, error::ClientError, token};

#[tokio::test]
async fn test() {
    let test = Test::builder().await;
    let client = test.client();
    let mut token = setup(client).await.unwrap();

    // Test token roles
    role::test_set(client, "test").await;
    role::test_list(client).await;
    role::test_read(client, "test").await;
    role::test_delete(client, "test").await;

    // Test tidy
    test_tidy(client).await;

    // Test creating tokens
    test_new(client).await;
    test_new_orphan(client).await;

    // Test looking up tokens
    test_lookup(client, token.token.as_str()).await;
    test_lookup_self(client).await;
    test_lookup_accessor(client, token.accessor.as_str()).await;

    // Test renewing tokens
    test_renew(client, token.token.as_str()).await;
    test_renew_self(client).await;
    test_renew_accessor(client, token.accessor.as_str()).await;

    // Test revoking tokens
    test_revoke(client, token.token.as_str()).await;
    token = setup(client).await.unwrap();
    test_revoke_accessor(client, token.accessor.as_str()).await;
    token = setup(client).await.unwrap();
    test_revoke_orphan(client, token.token.as_str()).await;

    test_revoke_self(client).await;
}

pub async fn test_lookup(client: &impl Client, token: &str) {
    token::lookup(client, token).await.unwrap();
}

pub async fn test_lookup_accessor(client: &impl Client, accessor: &str) {
    token::lookup_accessor(client, accessor).await.unwrap();
}

pub async fn test_lookup_self(client: &impl Client) {
    token::lookup_self(client).await.unwrap();
}

pub async fn test_new(client: &impl Client) {
    token::new(client, None).await.unwrap();
}

pub async fn test_new_orphan(client: &impl Client) {
    token::new_orphan(client, None).await.unwrap();
}

pub async fn test_renew(client: &impl Client, token: &str) {
    token::renew(client, token, Some("20m")).await.unwrap();
}

pub async fn test_renew_accessor(client: &impl Client, accessor: &str) {
    token::renew_accessor(client, accessor, Some("20m"))
        .await
        .unwrap();
}

pub async fn test_renew_self(client: &impl Client) {
    let resp = token::renew_self(client, Some("20m")).await;
    // Cannot renew the root token
    if let ClientError::APIError { code: _, errors } = resp.unwrap_err() {
        assert_eq!(errors[0], "lease is not renewable");
    }
}

pub async fn test_revoke(client: &impl Client, token: &str) {
    token::revoke(client, token).await.unwrap();
}

pub async fn test_revoke_accessor(client: &impl Client, accessor: &str) {
    token::revoke_accessor(client, accessor).await.unwrap();
}

pub async fn test_revoke_orphan(client: &impl Client, token: &str) {
    token::revoke_orphan(client, token).await.unwrap();
}

pub async fn test_revoke_self(client: &impl Client) {
    token::revoke_self(client).await.unwrap();
}

pub async fn test_tidy(client: &impl Client) {
    token::tidy(client).await.unwrap();
}

mod role {
    use vaultrs::api::token::requests::SetTokenRoleRequest;
    use vaultrs::token::role;

    use super::Client;

    pub async fn test_delete(client: &impl Client, role_name: &str) {
        role::delete(client, role_name).await.unwrap();
    }

    pub async fn test_list(client: &impl Client) {
        role::list(client).await.unwrap();
    }

    pub async fn test_read(client: &impl Client, role_name: &str) {
        role::read(client, role_name).await.unwrap();
    }

    pub async fn test_set(client: &impl Client, role_name: &str) {
        role::set(
            client,
            role_name,
            Some(
                SetTokenRoleRequest::builder()
                    .renewable(true)
                    .token_explicit_max_ttl("1h"),
            ),
        )
        .await
        .unwrap();
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
