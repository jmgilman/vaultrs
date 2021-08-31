mod common;

use std::collections::HashMap;

use common::VaultServer;
use vaultrs::api::auth::approle::requests::SetAppRoleRequest;
use vaultrs::api::auth::userpass::requests::CreateUserRequest;
use vaultrs::auth::{approle, userpass};
use vaultrs::login::{AppRoleLogin, UserpassLogin};

#[tokio::test]
async fn test_list() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);

    // Mount engines
    let res = server.mount_auth("approle_test", "approle").await;
    assert!(res.is_ok());

    let res = server.mount_auth("userpass_test", "userpass").await;
    assert!(res.is_ok());

    let mut expected = HashMap::<String, vaultrs::login::Method>::new();
    expected.insert("approle_test/".to_string(), vaultrs::login::Method::APPROLE);
    expected.insert("token/".to_string(), vaultrs::login::Method::TOKEN);
    expected.insert(
        "userpass_test/".to_string(),
        vaultrs::login::Method::USERPASS,
    );

    let res = vaultrs::login::list(&server.client).await;
    assert!(res.is_ok());

    let res = res.unwrap();
    assert_eq!(res["approle_test/"], expected["approle_test/"]);
    assert_eq!(res["token/"], expected["token/"]);
    assert_eq!(res["userpass_test/"], expected["userpass_test/"]);
}

#[tokio::test]
async fn test_list_supported() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);

    // Mount engines
    let res = server.mount_auth("oci_test", "oci").await;
    assert!(res.is_ok());

    let res = vaultrs::login::list_supported(&server.client).await;
    assert!(res.is_ok());

    let res = res.unwrap();
    assert_eq!(res.keys().len(), 1);
}

#[tokio::test]
async fn test_approle() {
    let docker = testcontainers::clients::Cli::default();
    let mut server = VaultServer::new(&docker);

    // Mount engine
    let res = server.mount_auth("approle_test", "approle").await;
    assert!(res.is_ok());

    // Create role
    let res = approle::role::set(
        &server.client,
        "approle_test",
        "test",
        Some(&mut SetAppRoleRequest::builder().token_ttl("10m")),
    )
    .await;
    assert!(res.is_ok());

    // Fetch details
    let res = approle::role::read_id(&server.client, "approle_test", "test").await;
    assert!(res.is_ok());
    let role_id = res.unwrap().role_id;

    let res = approle::role::secret::generate(&server.client, "approle_test", "test", None).await;
    assert!(res.is_ok());
    let secret_id = res.unwrap().secret_id;

    // Test login
    let res = &server
        .client
        .login("approle_test", &AppRoleLogin { role_id, secret_id })
        .await;
    assert!(res.is_ok());
    assert!(server.client.lookup().await.is_ok());
}

#[tokio::test]
async fn test_userpass() {
    let docker = testcontainers::clients::Cli::default();
    let mut server = VaultServer::new(&docker);

    // Mount engine
    let res = server.mount_auth("userpass_test", "userpass").await;
    assert!(res.is_ok());

    // Create a user
    let res = userpass::user::set(
        &server.client,
        "userpass_test",
        "test",
        "test",
        Some(CreateUserRequest::builder().token_policies(vec!["default".to_string()])),
    )
    .await;
    assert!(res.is_ok());

    // Test login
    let res = &server
        .client
        .login(
            "userpass_test",
            &UserpassLogin {
                username: "test".to_string(),
                password: "test".to_string(),
            },
        )
        .await;
    assert!(res.is_ok());
    assert!(server.client.lookup().await.is_ok());
}
