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

#[cfg(feature = "oidc")]
#[tokio::test]
async fn test_oidc() {
    use vaultrs::api::auth::oidc::requests::{SetConfigurationRequest, SetRoleRequest};

    // Create Vault container
    let vault_docker = testcontainers::clients::Cli::default();
    let mut vault_server = VaultServer::new(&vault_docker);

    // Create mock OAuth server
    // We must construct an "internal" url for the OAuth server using its DNS
    // name and internal port. When we pass Vault the OIDC discovery URL it will
    // configure itself by sending a request to the URL. Thus, it must be
    // reachable by Vault and we utilize Docker's internal DNS to accomplish
    // this.
    let oauth_docker = testcontainers::clients::Cli::default();
    let oauth_server = common::OAuthServer::new(&oauth_docker);
    let oauth_internal_url = format!("http://{}:{}", oauth_server.name, oauth_server.port);

    let mount = "oidc_test";
    let role = "test";
    let port = 8350;

    // Mount OIDC engine
    vault_server.mount_auth(mount, "oidc").await.unwrap();

    // Configure OIDC engine
    let auth_url = format!("{}/default", oauth_internal_url);
    vaultrs::auth::oidc::config::set(
        &vault_server.client,
        mount,
        Some(
            SetConfigurationRequest::builder()
                .oidc_discovery_url(auth_url)
                .oidc_client_id("test")
                .oidc_client_secret("test")
                .default_role(role),
        ),
    )
    .await
    .unwrap();

    // Create OIDC test role
    let redirect = format!("http://127.0.0.1:{}/oidc/callback", port);
    vaultrs::auth::oidc::role::set(
        &vault_server.client,
        mount,
        role,
        "sub",
        vec![redirect.clone()],
        Some(SetRoleRequest::builder().token_policies(vec!["default".to_string()])),
    )
    .await
    .unwrap();

    // Create OIDC login request
    let login = vaultrs::login::OIDCLogin {
        port: port,
        role: Some(role.to_string()),
    };
    let callback = vault_server.client.login_multi(mount, login).await.unwrap();

    // Perform a mock login
    // Vault is configured to use the DNS name and port of the test OAuth server
    // so it can communicate with it on the Docker network. Our local test
    // client won't be able to resolve the DNS name or reach the port since it's
    // forwarded to a random OS port. So we must replace it with the version
    // that our test client can resolve.
    let url = callback
        .url
        .replace(oauth_internal_url.as_str(), oauth_server.address.as_str());
    let client = reqwest::Client::default();
    let params = [("username", "default"), ("acr", "default")];
    client.post(url).form(&params).send().await.unwrap();

    // The callback should be successful now
    vault_server
        .client
        .login_multi_callback(mount, callback)
        .await
        .unwrap();
    assert!(vault_server.client.lookup().await.is_ok());
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
