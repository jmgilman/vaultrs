#[macro_use]
extern crate tracing;

mod common;

use common::VaultServerHelper;
use vaultrs::auth::userpass;
use vaultrs::client::Client;
use vaultrs::error::ClientError;
use vaultrs_test::docker::{Server, ServerConfig};
use vaultrs_test::{VaultServer, VaultServerConfig};

#[tracing_test::traced_test]
#[test]
fn test() {
    let config = VaultServerConfig::default(Some(common::VERSION));
    let instance = config.to_instance();
    instance.run(|ops| async move {
        let server = VaultServer::new(&ops, &config);
        let client = server.client();
        let endpoint = setup(&server, &client).await.unwrap();

        // Test user
        user::test_set(&client, &endpoint).await;
        user::test_read(&client, &endpoint).await;
        user::test_list(&client, &endpoint).await;
        user::test_update_policies(&client, &endpoint).await;

        // Test login
        test_login(&client, &endpoint).await;

        // Test update password and delete
        user::test_update_password(&client, &endpoint).await;
        user::test_delete(&client, &endpoint).await;
    });
}

#[instrument(skip(client))]
pub async fn test_login(client: &impl Client, endpoint: &UserPassEndpoint) {
    let res = userpass::login(
        client,
        endpoint.path.as_str(),
        endpoint.username.as_str(),
        endpoint.password.as_str(),
    )
    .await;
    assert!(res.is_ok());
}

pub mod user {
    use super::{Client, UserPassEndpoint};
    use vaultrs::auth::userpass::user;

    #[instrument(skip(client))]
    pub async fn test_delete(client: &impl Client, endpoint: &UserPassEndpoint) {
        let res = user::delete(client, endpoint.path.as_str(), endpoint.username.as_str()).await;
        assert!(res.is_ok());
    }

    #[instrument(skip(client))]
    pub async fn test_list(client: &impl Client, endpoint: &UserPassEndpoint) {
        let res = user::list(client, endpoint.path.as_str()).await;
        assert!(res.is_ok());
    }

    #[instrument(skip(client))]
    pub async fn test_read(client: &impl Client, endpoint: &UserPassEndpoint) {
        let res = user::read(client, endpoint.path.as_str(), endpoint.username.as_str()).await;
        assert!(res.is_ok());
    }

    #[instrument(skip(client))]
    pub async fn test_set(client: &impl Client, endpoint: &UserPassEndpoint) {
        let res = user::set(
            client,
            endpoint.path.as_str(),
            endpoint.username.as_str(),
            endpoint.password.as_str(),
            None,
        )
        .await;
        assert!(res.is_ok());
    }

    #[instrument(skip(client))]
    pub async fn test_update_password(client: &impl Client, endpoint: &UserPassEndpoint) {
        let res = user::update_password(
            client,
            endpoint.path.as_str(),
            endpoint.username.as_str(),
            "This1sAT3st!!",
        )
        .await;
        assert!(res.is_ok());
    }

    #[instrument(skip(client))]
    pub async fn test_update_policies(client: &impl Client, endpoint: &UserPassEndpoint) {
        let res = user::update_policies(
            client,
            endpoint.path.as_str(),
            endpoint.username.as_str(),
            "default",
        )
        .await;
        assert!(res.is_ok());
    }
}

#[derive(Debug)]
pub struct UserPassEndpoint {
    pub path: String,
    pub username: String,
    pub password: String,
}

async fn setup(
    server: &VaultServer,
    client: &impl Client,
) -> Result<UserPassEndpoint, ClientError> {
    debug!("setting up UserPass auth engine");

    let path = "userpass_test";
    let username = "test";
    let password = "This1sAT3st!";

    // Mount the UserPass auth engine
    server.mount_auth(client, path, "userpass").await?;

    Ok(UserPassEndpoint {
        path: path.to_string(),
        username: username.to_string(),
        password: password.to_string(),
    })
}
