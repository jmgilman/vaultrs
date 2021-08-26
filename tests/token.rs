mod common;

use common::VaultServer;
use vaultrs::token;

#[tokio::test]
async fn test() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);

    test_create(&server).await;
    test_create_orphan(&server).await;
}

pub async fn test_create(server: &VaultServer<'_>) {
    let resp = token::new(&server.client, None).await;
    assert!(resp.is_ok());
}

pub async fn test_create_orphan(server: &VaultServer<'_>) {
    let resp = token::new_orphan(&server.client, None).await;
    assert!(resp.is_ok());
}

// TODO: Add test for create token with role
