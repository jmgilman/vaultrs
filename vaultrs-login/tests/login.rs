#[macro_use]
extern crate tracing;
#[macro_use]
extern crate tracing_test;

mod common;

use std::collections::HashMap;

use common::{LocalStackServer, OIDCServer, VaultServer, VaultServerHelper};
use vaultrs::api::auth::approle::requests::SetAppRoleRequest;
use vaultrs::api::auth::userpass::requests::CreateUserRequest;
use vaultrs::auth::{approle, aws, userpass};
use vaultrs::client::VaultClient;
use vaultrs_login::engines::{approle::AppRoleLogin, userpass::UserpassLogin};
use vaultrs_login::method::{self, Method};
use vaultrs_login::LoginClient;

#[traced_test]
#[test]
fn test() {
    let test = common::new_test();
    test.run(|instance| async move {
        let _oidc_server: OIDCServer = instance.server();
        let vault_server: VaultServer = instance.server();
        let _localstack_server: LocalStackServer = instance.server();
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
        vault_server
            .mount_auth(&client, "aws_test", "aws")
            .await
            .unwrap();

        // Test login methods
        test_list(&client).await;
        test_list_supported(&client).await;

        // Test login endpoints
        test_approle(&mut vault_server.client()).await;
        test_userpass(&mut vault_server.client()).await;

        #[cfg(feature = "oidc")]
        test_oidc(&_oidc_server, &vault_server, &mut vault_server.client()).await;

        #[cfg(feature = "aws")]
        test_aws(&_localstack_server, &mut vault_server.client()).await;
    });
}

#[instrument(skip(client))]
async fn test_list(client: &VaultClient) {
    debug!("running test...");

    // Mount engines
    let mut expected = HashMap::<String, Method>::new();
    expected.insert("approle_test/".to_string(), Method::APPROLE);
    expected.insert("token/".to_string(), Method::TOKEN);
    expected.insert("userpass_test/".to_string(), Method::USERPASS);
    expected.insert("aws_test/".to_string(), Method::AWS);

    let res = method::list(client).await;
    assert!(res.is_ok());

    let res = res.unwrap();
    assert_eq!(res["approle_test/"], expected["approle_test/"]);
    assert_eq!(res["token/"], expected["token/"]);
    assert_eq!(res["userpass_test/"], expected["userpass_test/"]);
    assert_eq!(res["aws_test/"], expected["aws_test/"]);
}

#[instrument(skip(client))]
async fn test_list_supported(client: &VaultClient) {
    debug!("running test...");

    let res = method::list_supported(client).await;
    assert!(res.is_ok());

    let res = res.unwrap();
    assert_eq!(res.keys().len(), 3);
}

#[instrument(skip(client))]
async fn test_approle(client: &mut VaultClient) {
    debug!("running test...");

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
#[instrument(skip(client, oidc_server, vault_server))]
async fn test_oidc(oidc_server: &OIDCServer, vault_server: &VaultServer, client: &mut VaultClient) {
    debug!("running test...");

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
    let auth_url = format!("{}/default", oidc_server.internal_url());
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
    let login = vaultrs_login::engines::oidc::OIDCLogin {
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
        oidc_server.internal_url().as_str(),
        oidc_server.external_url().as_str(),
    );
    let rclient = reqwest::Client::default();
    let params = [("username", "default"), ("acr", "default")];
    rclient.post(url).form(&params).send().await.unwrap();

    // The callback should be successful now
    client.login_multi_callback(mount, callback).await.unwrap();
    //assert!(vault_server.client.lookup().await.is_ok());
}

#[instrument(skip(client))]
async fn test_userpass(client: &mut VaultClient) {
    debug!("running test...");

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

#[cfg(feature = "aws")]
#[instrument(skip(localstack, client))]
async fn test_aws(localstack: &LocalStackServer, client: &mut VaultClient) {
    debug!("running test...");

    use vaultrs::api::auth::aws::requests::{ConfigureClientRequest, CreateRoleRequest};

    let mount = "aws_test";

    aws::config::client::set(
        client,
        mount,
        Some(
            &mut ConfigureClientRequest::builder()
                .access_key("test")
                .secret_key("test")
                .endpoint(localstack.internal_url())
                .iam_endpoint(localstack.internal_url())
                .sts_endpoint(localstack.internal_url())
                .sts_region("local"),
        ),
    )
    .await
    .unwrap();

    // create role
    use aws_credential_types::Credentials;
    use aws_types::{region::Region, sdk_config::SharedCredentialsProvider, SdkConfig};

    let credentials = Credentials::new("test", "test", None, None, "static");

    let aws_config = SdkConfig::builder()
        .region(Region::new("local"))
        .credentials_provider(SharedCredentialsProvider::new(credentials))
        .build();

    let iam_config = aws_sdk_iam::config::Builder::from(&aws_config)
        .endpoint_url(localstack.internal_url())
        .behavior_version_latest()
        .build();

    let iam_client = aws_sdk_iam::Client::from_conf(iam_config);

    let assume_role_policy_document = r#"{
        "Version": "2012-10-17",
        "Statement": [{
            "Effect": "Allow",
            "Principal": {
                "AWS": "arn:aws:iam::000000000000:root"
            },
            "Action": "sts:AssumeRole"
        }]
    }"#;

    let aws_role = iam_client
        .create_role()
        .role_name("TestLogin")
        .assume_role_policy_document(assume_role_policy_document)
        .send()
        .await
        .unwrap();

    let aws_role_arn = aws_role.role().unwrap().arn();

    aws::role::create(
        client,
        mount,
        "test_role",
        Some(
            &mut CreateRoleRequest::builder()
                .auth_type("iam")
                .bound_iam_principal_arn([aws_role_arn.to_string()])
                .resolve_aws_unique_ids(false),
        ),
    )
    .await
    .unwrap();

    let sts_config = aws_sdk_sts::config::Builder::from(&aws_config)
        .endpoint_url(localstack.internal_url())
        .behavior_version_latest()
        .build();
    let sts_client = aws_sdk_sts::Client::from_conf(sts_config);

    let assumed_role_credentials = sts_client
        .assume_role()
        .role_arn(aws_role_arn)
        .role_session_name("TestSession")
        .send()
        .await
        .unwrap()
        .credentials
        .unwrap();

    // Test login
    let login = vaultrs_login::engines::aws::AwsIamLogin {
        access_key: assumed_role_credentials.access_key_id,
        secret_key: assumed_role_credentials.secret_access_key,
        region: "local".to_string(),
        session_token: Some(assumed_role_credentials.session_token),
        role: Some("test_role".to_string()),
        header_value: None,
    };

    let res = client.login(mount, &login).await;
    assert!(res.is_ok());
    //assert!(vault_server.client.lookup().await.is_ok());
}
