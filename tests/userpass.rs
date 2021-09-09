pub const VERSION: &str = "1.8.2";

use vaultrs::auth::userpass;
use vaultrs::error::ClientError;
use vaultrs_test::docker::{Server, ServerConfig};
use vaultrs_test::{VaultServer, VaultServerConfig};

#[test]
fn test() {
    let config = VaultServerConfig::default(Some(VERSION));
    let instance = config.to_instance();
    instance.run(|ops| async move {
        let server = VaultServer::new(&ops, &config);
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
    });
}

pub async fn test_login(server: &VaultServer, endpoint: &UserPassEndpoint) {
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
    use super::{UserPassEndpoint, VaultServer};
    use vaultrs::auth::userpass::user;

    pub async fn test_delete(server: &VaultServer, endpoint: &UserPassEndpoint) {
        let res = user::delete(
            &server.client,
            endpoint.path.as_str(),
            endpoint.username.as_str(),
        )
        .await;
        assert!(res.is_ok());
    }

    pub async fn test_list(server: &VaultServer, endpoint: &UserPassEndpoint) {
        let res = user::list(&server.client, endpoint.path.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_read(server: &VaultServer, endpoint: &UserPassEndpoint) {
        let res = user::read(
            &server.client,
            endpoint.path.as_str(),
            endpoint.username.as_str(),
        )
        .await;
        assert!(res.is_ok());
    }

    pub async fn test_set(server: &VaultServer, endpoint: &UserPassEndpoint) {
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

    pub async fn test_update_password(server: &VaultServer, endpoint: &UserPassEndpoint) {
        let res = user::update_password(
            &server.client,
            endpoint.path.as_str(),
            endpoint.username.as_str(),
            "This1sAT3st!!",
        )
        .await;
        assert!(res.is_ok());
    }

    pub async fn test_update_policies(server: &VaultServer, endpoint: &UserPassEndpoint) {
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

async fn setup(server: &VaultServer) -> Result<UserPassEndpoint, ClientError> {
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
