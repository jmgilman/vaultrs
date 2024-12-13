use crate::common::Test;
use tracing::debug;
use vaultrs::auth::userpass;
use vaultrs::client::Client;
use vaultrs::error::ClientError;
use vaultrs::sys::auth;

#[tokio::test]
async fn test() {
    let test = Test::builder().await;
    let client = test.client();
    let endpoint = setup(client).await.unwrap();

    // Test user
    user::test_set(client, &endpoint).await;
    user::test_read(client, &endpoint).await;
    user::test_list(client, &endpoint).await;
    user::test_update_policies(client, &endpoint).await;

    // Test login
    test_login(client, &endpoint).await;

    // Test update password and delete
    user::test_update_password(client, &endpoint).await;
    user::test_delete(client, &endpoint).await;
}

pub async fn test_login(client: &impl Client, endpoint: &UserPassEndpoint) {
    userpass::login(
        client,
        endpoint.path.as_str(),
        endpoint.username.as_str(),
        endpoint.password.as_str(),
    )
    .await
    .unwrap();
}

pub mod user {
    use super::{Client, UserPassEndpoint};
    use vaultrs::auth::userpass::user;

    pub async fn test_delete(client: &impl Client, endpoint: &UserPassEndpoint) {
        user::delete(client, endpoint.path.as_str(), endpoint.username.as_str())
            .await
            .unwrap();
    }

    pub async fn test_list(client: &impl Client, endpoint: &UserPassEndpoint) {
        user::list(client, endpoint.path.as_str()).await.unwrap();
    }

    pub async fn test_read(client: &impl Client, endpoint: &UserPassEndpoint) {
        user::read(client, endpoint.path.as_str(), endpoint.username.as_str())
            .await
            .unwrap();
    }

    pub async fn test_set(client: &impl Client, endpoint: &UserPassEndpoint) {
        user::set(
            client,
            endpoint.path.as_str(),
            endpoint.username.as_str(),
            endpoint.password.as_str(),
            None,
        )
        .await
        .unwrap();
    }

    pub async fn test_update_password(client: &impl Client, endpoint: &UserPassEndpoint) {
        user::update_password(
            client,
            endpoint.path.as_str(),
            endpoint.username.as_str(),
            "This1sAT3st!!",
        )
        .await
        .unwrap();
    }

    pub async fn test_update_policies(client: &impl Client, endpoint: &UserPassEndpoint) {
        user::update_policies(
            client,
            endpoint.path.as_str(),
            endpoint.username.as_str(),
            "default",
        )
        .await
        .unwrap();
    }
}

#[derive(Debug)]
pub struct UserPassEndpoint {
    pub path: String,
    pub username: String,
    pub password: String,
}

async fn setup(client: &impl Client) -> Result<UserPassEndpoint, ClientError> {
    debug!("setting up UserPass auth engine");

    let path = "userpass_test";
    let username = "test";
    let password = "This1sAT3st!";

    // Mount the UserPass auth engine
    auth::enable(client, path, "userpass", None).await.unwrap();

    Ok(UserPassEndpoint {
        path: path.to_string(),
        username: username.to_string(),
        password: password.to_string(),
    })
}
