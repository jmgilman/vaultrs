mod common;

use std::collections::HashMap;

use common::VaultServerHelper;
use vaultrs::api::auth::approle::requests::SetAppRoleRequest;
use vaultrs::api::auth::userpass::requests::CreateUserRequest;
use vaultrs::auth::{approle, userpass};
use vaultrs::client::Client;
use vaultrs::login::{AppRoleLogin, UserpassLogin};
use vaultrs_test::docker::{Server, ServerConfig};
use vaultrs_test::oidc::{OIDCServer, OIDCServerConfig};
use vaultrs_test::{TestInstance, VaultServer, VaultServerConfig};

#[test]
fn test() {
    let oidc_config = OIDCServerConfig::default(Some("0.3.4"));
    let vault_config = VaultServerConfig::default(Some(common::VERSION));
    let instance = TestInstance::new(vec![oidc_config.to_comp(), vault_config.to_comp()]);

    instance.run(|ops| async move {
        let oidc_server = OIDCServer::new(&ops, &oidc_config);
        let vault_server = VaultServer::new(&ops, &vault_config);
        let client = vault_server.client();

        // Mounts
        vault_server
            .mount_auth(&client, "approle_test", "approle")
            .await
            .unwrap();
        vault_server
            .mount_auth(&client, "oci_test", "oci")
            .await
            .unwrap();
        vault_server
            .mount_auth(&client, "userpass_test", "userpass")
            .await
            .unwrap();

        // Test login methods
        test_list(&client).await;
        test_list_supported(&client).await;

        #[cfg(feature = "oidc")]
        test_oidc(&oidc_server, &vault_server, &mut vault_server.client()).await;

        // Test login endpoints
        test_approle(&mut vault_server.client()).await;
        test_userpass(&mut vault_server.client()).await;
    });
}

async fn test_list(client: &impl Client) {
    // Mount engines
    let mut expected = HashMap::<String, vaultrs::login::Method>::new();
    expected.insert("approle_test/".to_string(), vaultrs::login::Method::APPROLE);
    expected.insert("token/".to_string(), vaultrs::login::Method::TOKEN);
    expected.insert(
        "userpass_test/".to_string(),
        vaultrs::login::Method::USERPASS,
    );

    let res = vaultrs::login::method::list(client).await;
    assert!(res.is_ok());

    let res = res.unwrap();
    assert_eq!(res["approle_test/"], expected["approle_test/"]);
    assert_eq!(res["token/"], expected["token/"]);
    assert_eq!(res["userpass_test/"], expected["userpass_test/"]);
}

async fn test_list_supported(client: &impl Client) {
    let res = vaultrs::login::method::list_supported(client).await;
    assert!(res.is_ok());

    let res = res.unwrap();
    assert_eq!(res.keys().len(), 2);
}

async fn test_approle(client: &mut impl Client) {
    // Create role
    let res = approle::role::set(
        client,
        "approle_test",
        "test",
        Some(&mut SetAppRoleRequest::builder().token_ttl("10m")),
    )
    .await;
    assert!(res.is_ok());

    // Fetch details
    let res = approle::role::read_id(client, "approle_test", "test").await;
    assert!(res.is_ok());
    let role_id = res.unwrap().role_id;

    let res = approle::role::secret::generate(client, "approle_test", "test", None).await;
    assert!(res.is_ok());
    let secret_id = res.unwrap().secret_id;

    // Test login
    let res = client
        .login("approle_test", &AppRoleLogin { role_id, secret_id })
        .await;
    assert!(res.is_ok());
    //assert!(client.lookup().await.is_ok());
}

#[cfg(feature = "oidc")]
async fn test_oidc(oidc_server: &OIDCServer, vault_server: &VaultServer, client: &mut impl Client) {
    use vaultrs::api::auth::oidc::requests::{SetConfigurationRequest, SetRoleRequest};

    let mount = "oidc_test";
    let role = "test";
    let port = 8350;

    // Mount OIDC engine
    vault_server
        .mount_auth(client, mount, "oidc")
        .await
        .unwrap();

    // Configure OIDC engine
    let auth_url = format!("{}/default", oidc_server.address_internal);
    vaultrs::auth::oidc::config::set(
        client,
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
    let redirect = format!("http://localhost:{}/oidc/callback", port);
    vaultrs::auth::oidc::role::set(
        client,
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
        port: Some(port),
        role: Some(role.to_string()),
    };
    let callback = client.login_multi(mount, login).await.unwrap();

    // Perform a mock login
    // Vault is configured to use the DNS name and port of the test OAuth server
    // so it can communicate with it on the Docker network. Our local test
    // client won't be able to resolve the DNS name or reach the port since it's
    // forwarded to a random OS port. So we must replace it with the version
    // that our test client can resolve.
    let url = callback.url.replace(
        oidc_server.address_internal.as_str(),
        oidc_server.address.as_str(),
    );
    let rclient = reqwest::Client::default();
    let params = [("username", "default"), ("acr", "default")];
    rclient.post(url).form(&params).send().await.unwrap();

    // The callback should be successful now
    client.login_multi_callback(mount, callback).await.unwrap();
    //assert!(vault_server.client.lookup().await.is_ok());
}

async fn test_userpass(client: &mut impl Client) {
    // Create a user
    let res = userpass::user::set(
        client,
        "userpass_test",
        "test",
        "test",
        Some(CreateUserRequest::builder().token_policies(vec!["default".to_string()])),
    )
    .await;
    assert!(res.is_ok());

    // Test login
    let res = client
        .login(
            "userpass_test",
            &UserpassLogin {
                username: "test".to_string(),
                password: "test".to_string(),
            },
        )
        .await;
    assert!(res.is_ok());
    //assert!(server.client.lookup().await.is_ok());
}
