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
    crate::role::test_read_id(&server, &endpoint).await;
    crate::role::test_update_id(&server, &endpoint).await;

    // Test secret IDs
    let (id, accessor) = crate::role::secret::test_generate(&server, &endpoint).await;
    crate::role::secret::test_read(&server, &endpoint, id.as_str()).await;
    crate::role::secret::test_read_accessor(&server, &endpoint, accessor.as_str()).await;
    crate::role::secret::test_list(&server, &endpoint).await;
    crate::role::secret::test_delete_accessor(&server, &endpoint, accessor.as_str()).await;
    crate::role::secret::test_custom(&server, &endpoint).await;
    crate::role::secret::test_delete(&server, &endpoint, "test").await;

    // Test auth
    test_login(&server, &endpoint).await;

    crate::role::test_delete(&server, &endpoint).await;
}

pub async fn test_login(server: &VaultServer<'_>, endpoint: &AppRoleEndpoint) {
    use vaultrs::auth::approle::role;
    let role_id_resp = role::read_id(
        &server.client,
        endpoint.path.as_str(),
        endpoint.role_name.as_str(),
    )
    .await;
    assert!(role_id_resp.is_ok());
    let role_id = role_id_resp.unwrap().role_id;

    let secret_id_resp = role::secret::generate(
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
    use vaultrs::{api::auth::approle::requests::SetAppRoleRequest, auth::approle::role};

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

    pub async fn test_read_id(server: &VaultServer<'_>, endpoint: &AppRoleEndpoint) {
        let res = role::read_id(
            &server.client,
            endpoint.path.as_str(),
            endpoint.role_name.as_str(),
        )
        .await;
        assert!(res.is_ok());
    }

    pub async fn test_update_id(server: &VaultServer<'_>, endpoint: &AppRoleEndpoint) {
        let res = role::update_id(
            &server.client,
            endpoint.path.as_str(),
            endpoint.role_name.as_str(),
            "test",
        )
        .await;
        assert!(res.is_ok());
    }

    pub mod secret {
        use crate::{common::VaultServer, AppRoleEndpoint};
        use vaultrs::{
            api::auth::approle::requests::GenerateNewSecretIDRequest, auth::approle::role::secret,
        };

        pub async fn test_custom(server: &VaultServer<'_>, endpoint: &AppRoleEndpoint) {
            let res = secret::custom(
                &server.client,
                endpoint.path.as_str(),
                endpoint.role_name.as_str(),
                "test",
                None,
            )
            .await;
            assert!(res.is_ok());
        }

        pub async fn test_delete(server: &VaultServer<'_>, endpoint: &AppRoleEndpoint, id: &str) {
            let res = secret::delete(
                &server.client,
                endpoint.path.as_str(),
                endpoint.role_name.as_str(),
                id,
            )
            .await;
            assert!(res.is_ok());
        }

        pub async fn test_delete_accessor(
            server: &VaultServer<'_>,
            endpoint: &AppRoleEndpoint,
            accessor: &str,
        ) {
            let res = secret::delete_accessor(
                &server.client,
                endpoint.path.as_str(),
                endpoint.role_name.as_str(),
                accessor,
            )
            .await;
            assert!(res.is_ok());
        }

        pub async fn test_generate(
            server: &VaultServer<'_>,
            endpoint: &AppRoleEndpoint,
        ) -> (String, String) {
            let res = secret::generate(
                &server.client,
                endpoint.path.as_str(),
                endpoint.role_name.as_str(),
                Some(
                    &mut GenerateNewSecretIDRequest::builder()
                        .metadata("{ \"tag1\": \"production\" }"),
                ),
            )
            .await;
            assert!(res.is_ok());

            let id = res.unwrap();
            (id.secret_id, id.secret_id_accessor)
        }

        pub async fn test_list(server: &VaultServer<'_>, endpoint: &AppRoleEndpoint) {
            let res = secret::list(
                &server.client,
                endpoint.path.as_str(),
                endpoint.role_name.as_str(),
            )
            .await;
            assert!(res.is_ok());
        }

        pub async fn test_read(server: &VaultServer<'_>, endpoint: &AppRoleEndpoint, id: &str) {
            let res = secret::read(
                &server.client,
                endpoint.path.as_str(),
                endpoint.role_name.as_str(),
                id,
            )
            .await;
            assert!(res.is_ok());
        }

        pub async fn test_read_accessor(
            server: &VaultServer<'_>,
            endpoint: &AppRoleEndpoint,
            accessor: &str,
        ) {
            let res = secret::read_accessor(
                &server.client,
                endpoint.path.as_str(),
                endpoint.role_name.as_str(),
                accessor,
            )
            .await;
            assert!(res.is_ok());
        }
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
