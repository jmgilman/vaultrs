mod common;

use common::VaultServer;
use vaultrs::auth::userpass;
use vaultrs::error::ClientError;

#[tokio::test]
async fn test() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);
    let endpoint = setup(&server).await.unwrap();

    // Test user
    user::test_set(&server, &endpoint).await;
    user::test_read(&server, &endpoint).await;
    user::test_list(&server, &endpoint).await;
    user::test_update_policies(&server, &endpoint).await;

    // Test login
    test_login(&server, &endpoint).await;

    // Test update password and delete
    user::test_update_password(&server, &endpoint).await;
    user::test_delete(&server, &endpoint).await;
}

pub async fn test_login(server: &VaultServer<'_>, endpoint: &UserPassEndpoint) {
    let res = userpass::login(
        &server.client,
        endpoint.path.as_str(),
        endpoint.username.as_str(),
        endpoint.password.as_str(),
    )
    .await;
    assert!(res.is_ok());
}

pub mod user {
    use crate::{common::VaultServer, UserPassEndpoint};
    use vaultrs::auth::userpass::user;

    pub async fn test_delete(server: &VaultServer<'_>, endpoint: &UserPassEndpoint) {
        let res = user::delete(
            &server.client,
            endpoint.path.as_str(),
            endpoint.username.as_str(),
        )
        .await;
        assert!(res.is_ok());
    }

    pub async fn test_list(server: &VaultServer<'_>, endpoint: &UserPassEndpoint) {
        let res = user::list(&server.client, endpoint.path.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_read(server: &VaultServer<'_>, endpoint: &UserPassEndpoint) {
        let res = user::read(
            &server.client,
            endpoint.path.as_str(),
            endpoint.username.as_str(),
        )
        .await;
        assert!(res.is_ok());
    }

    pub async fn test_set(server: &VaultServer<'_>, endpoint: &UserPassEndpoint) {
        let res = user::set(
            &server.client,
            endpoint.path.as_str(),
            endpoint.username.as_str(),
            endpoint.password.as_str(),
            None,
        )
        .await;
        assert!(res.is_ok());
    }

    pub async fn test_update_password(server: &VaultServer<'_>, endpoint: &UserPassEndpoint) {
        let res = user::update_password(
            &server.client,
            endpoint.path.as_str(),
            endpoint.username.as_str(),
            "This1sAT3st!!",
        )
        .await;
        assert!(res.is_ok());
    }

    pub async fn test_update_policies(server: &VaultServer<'_>, endpoint: &UserPassEndpoint) {
        let res = user::update_policies(
            &server.client,
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

async fn setup(server: &VaultServer<'_>) -> Result<UserPassEndpoint, ClientError> {
    let path = "userpass_test";
    let username = "test";
    let password = "This1sAT3st!";

    // Mount the UserPass auth engine
    server.mount_auth(path, "userpass").await?;

    Ok(UserPassEndpoint {
        path: path.to_string(),
        username: username.to_string(),
        password: password.to_string(),
    })
}
