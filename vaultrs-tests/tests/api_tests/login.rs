use std::collections::HashMap;
use tracing::{debug, instrument};
use vaultrs::api::auth::approle::requests::SetAppRoleRequest;
use vaultrs::api::auth::userpass::requests::CreateUserRequest;
use vaultrs::auth::{approle, userpass};
use vaultrs::client::{Client, VaultClient};
use vaultrs::sys::auth;
use vaultrs_login::engines::{approle::AppRoleLogin, userpass::UserpassLogin};
use vaultrs_login::method::{self, Method};
use vaultrs_login::LoginClient;

use crate::common::Test;

#[tokio::test]
#[ignore]
async fn test() {
    let mut test = Test::builder()
        .with_localstack(["iam", "sts"])
        .with_oidc()
        .await;
    let client = test.client();

    // Mounts
    auth::enable(client, "approle_test", "approle", None)
        .await
        .unwrap();
    auth::enable(client, "oidc_test", "oidc", None)
        .await
        .unwrap();
    auth::enable(client, "userpass_test", "userpass", None)
        .await
        .unwrap();
    auth::enable(client, "aws_test", "aws", None).await.unwrap();

    // Test login methods
    test_list(client).await;
    test_list_supported(client).await;

    // Test login endpoints
    test_approle(test.client_mut()).await;
    test.client_mut().set_token("root");

    test_userpass(test.client_mut()).await;
    test.client_mut().set_token("root");

    let oidc_url = test.oidc_url().unwrap().to_string();
    test_oidc(&oidc_url, test.client_mut()).await;
    test.client_mut().set_token("root");

    let aws_url = test.localstack_url().unwrap().to_string();
    test_aws(&aws_url, test.client_mut()).await;
}

#[instrument(skip(client))]
async fn test_list(client: &VaultClient) {
    debug!("running test...");

    let auths = method::list(client).await.unwrap();
    let expected_auths = HashMap::from([
        ("approle_test/".into(), Method::APPROLE),
        ("oidc_test/".to_string(), Method::OIDC),
        ("token/".to_string(), Method::TOKEN),
        ("userpass_test/".to_string(), Method::USERPASS),
        ("aws_test/".to_string(), Method::AWS),
    ]);

    assert_eq!(auths, expected_auths);
}

#[instrument(skip(client))]
async fn test_list_supported(client: &VaultClient) {
    debug!("running test...");

    assert_eq!(
        method::list_supported(client).await.unwrap().keys().len(),
        4
    );
}

#[instrument(skip(client))]
async fn test_approle(client: &mut VaultClient) {
    debug!("running test...");

    // Create role
    approle::role::set(
        client,
        "approle_test",
        "test",
        Some(&mut SetAppRoleRequest::builder().token_ttl("10m")),
    )
    .await
    .unwrap();

    // Fetch details
    let role_id = approle::role::read_id(client, "approle_test", "test")
        .await
        .unwrap()
        .role_id;

    let secret_id = approle::role::secret::generate(client, "approle_test", "test", None)
        .await
        .unwrap()
        .secret_id;

    // Test login
    client
        .login("approle_test", &AppRoleLogin { role_id, secret_id })
        .await
        .unwrap();
    client.lookup().await.unwrap();
}

#[instrument(skip(client, oidc_url))]
async fn test_oidc(oidc_url: &str, client: &mut VaultClient) {
    debug!("running test...");

    use vaultrs::api::auth::oidc::requests::{SetConfigurationRequest, SetRoleRequest};

    let mount = "oidc_test";
    let role = "test";
    const PORT: u16 = 8350;

    // Configure OIDC engine
    let auth_url = format!("{}/default", oidc_url);
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

    // This url is the call back of the server that vault login will spawn.
    let redirect = format!("http://localhost:{PORT}/oidc/callback");
    // Create OIDC test role
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
        port: Some(PORT),
        role: Some(role.to_string()),
    };
    // The callback contains the aurorize url and the parameters
    let callback = client.login_multi(mount, login).await.unwrap();

    let url = callback.url.as_str();
    let rclient = reqwest::Client::default();
    let params = [("username", "default"), ("acr", "default")];

    rclient.post(url).form(&params).send().await.unwrap();

    // The callback should be successful now
    client.login_multi_callback(mount, callback).await.unwrap();
    client.lookup().await.unwrap();
}

#[instrument(skip(client))]
async fn test_userpass(client: &mut VaultClient) {
    debug!("running test...");

    // Create a user
    userpass::user::set(
        client,
        "userpass_test",
        "test",
        "test",
        Some(CreateUserRequest::builder().token_policies(vec!["default".to_string()])),
    )
    .await
    .unwrap();

    // Test login
    client
        .login(
            "userpass_test",
            &UserpassLogin {
                username: "test".to_string(),
                password: "test".to_string(),
            },
        )
        .await
        .unwrap();
    client.lookup().await.unwrap();
}

#[instrument(skip(localstack_url, client))]
async fn test_aws(localstack_url: &str, client: &mut VaultClient) {
    debug!("running test...");

    use vaultrs::api::auth::aws::requests::{ConfigureClientRequest, CreateRoleRequest};
    use vaultrs::auth::aws;

    let mount = "aws_test";

    aws::config::client::set(
        client,
        mount,
        Some(
            &mut ConfigureClientRequest::builder()
                .access_key("test")
                .secret_key("test")
                .endpoint(localstack_url)
                .iam_endpoint(localstack_url)
                .sts_endpoint(localstack_url)
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
        .endpoint_url(localstack_url)
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
        .endpoint_url(localstack_url)
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

    client.login(mount, &login).await.unwrap();
    client.lookup().await.unwrap();
}
