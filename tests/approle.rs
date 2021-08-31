mod common;

use common::VaultServer;
use vaultrs::auth::approle;
use vaultrs::error::ClientError;

#[tokio::test]
async fn test() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);
    let endpoint = setup(&server).await.unwrap();

    // Test roles
    crate::role::test_set(&server, &endpoint).await;
    crate::role::test_read(&server, &endpoint).await;
    crate::role::test_list(&server, &endpoint).await;
    crate::role::test_read_role_id(&server, &endpoint).await;
    crate::role::test_generate_secret_id(&server, &endpoint).await;

    // Test auth
    test_login(&server, &endpoint).await;

    crate::role::test_delete(&server, &endpoint).await;
}

pub async fn test_login(server: &VaultServer<'_>, endpoint: &AppRoleEndpoint) {
    use vaultrs::auth::approle::role::{generate_secret_id, read_role_id};
    let role_id_resp = read_role_id(
        &server.client,
        endpoint.path.as_str(),
        endpoint.role_name.as_str(),
    )
    .await;
    assert!(role_id_resp.is_ok());
    let role_id = role_id_resp.unwrap().role_id;

    let secret_id_resp = generate_secret_id(
        &server.client,
        endpoint.path.as_str(),
        endpoint.role_name.as_str(),
        None,
    )
    .await;
    assert!(secret_id_resp.is_ok());
    let secret_id = secret_id_resp.unwrap().secret_id;

    let resp = approle::login(
        &server.client,
        endpoint.path.as_str(),
        role_id.as_str(),
        secret_id.as_str(),
    )
    .await;
    assert!(resp.is_ok());
}

mod role {
    use crate::{common::VaultServer, AppRoleEndpoint};
    use vaultrs::{
        api::auth::approle::requests::{GenerateNewSecretIDRequest, SetAppRoleRequest},
        auth::approle::role,
    };

    pub async fn test_delete(server: &VaultServer<'_>, endpoint: &AppRoleEndpoint) {
        let res = role::delete(
            &server.client,
            endpoint.path.as_str(),
            endpoint.role_name.as_str(),
        )
        .await;
        assert!(res.is_ok());
    }

    pub async fn test_list(server: &VaultServer<'_>, endpoint: &AppRoleEndpoint) {
        let res = role::list(&server.client, endpoint.path.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_read(server: &VaultServer<'_>, endpoint: &AppRoleEndpoint) {
        let res = role::read(
            &server.client,
            endpoint.path.as_str(),
            endpoint.role_name.as_str(),
        )
        .await;
        assert!(res.is_ok());
    }

    pub async fn test_set(server: &VaultServer<'_>, endpoint: &AppRoleEndpoint) {
        let res = role::set(
            &server.client,
            endpoint.path.as_str(),
            endpoint.role_name.as_str(),
            Some(&mut SetAppRoleRequest::builder().token_ttl("10m")),
        )
        .await;
        assert!(res.is_ok());
    }

    pub async fn test_read_role_id(server: &VaultServer<'_>, endpoint: &AppRoleEndpoint) {
        let res = role::read_role_id(
            &server.client,
            endpoint.path.as_str(),
            endpoint.role_name.as_str(),
        )
        .await;
        assert!(res.is_ok());
    }

    pub async fn test_generate_secret_id(server: &VaultServer<'_>, endpoint: &AppRoleEndpoint) {
        let res = role::generate_secret_id(
            &server.client,
            endpoint.path.as_str(),
            endpoint.role_name.as_str(),
            Some(
                &mut GenerateNewSecretIDRequest::builder().metadata("{ \"tag1\": \"production\" }"),
            ),
        )
        .await;
        assert!(res.is_ok());
    }
}

#[derive(Debug)]
pub struct AppRoleEndpoint {
    pub path: String,
    pub role_name: String,
}

async fn setup(server: &VaultServer<'_>) -> Result<AppRoleEndpoint, ClientError> {
    let path = "approle_test";
    let role_name = "test";

    // Mount the AppRole auth engine
    server.mount_auth(path, "approle").await?;

    Ok(AppRoleEndpoint {
        path: path.to_string(),
        role_name: role_name.to_string(),
    })
}
