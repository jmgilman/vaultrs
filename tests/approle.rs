#[macro_use]
extern crate tracing;

mod common;

use common::{VaultServer, VaultServerHelper};
use test_env_log::test;
use vaultrs::auth::approle;
use vaultrs::client::Client;
use vaultrs::error::ClientError;

#[test]
fn test() {
    let test = common::new_test();
    test.run(|instance| async move {
        let server: VaultServer = instance.server();
        let client = server.client();
        let endpoint = setup(&server, &client).await.unwrap();

        // Test roles
        crate::role::test_set(&client, &endpoint).await;
        crate::role::test_read(&client, &endpoint).await;
        crate::role::test_list(&client, &endpoint).await;
        crate::role::test_read_id(&client, &endpoint).await;
        crate::role::test_update_id(&client, &endpoint).await;

        // Test secret IDs
        let (id, accessor) = crate::role::secret::test_generate(&client, &endpoint).await;
        crate::role::secret::test_read(&client, &endpoint, id.as_str()).await;
        crate::role::secret::test_read_accessor(&client, &endpoint, accessor.as_str()).await;
        crate::role::secret::test_list(&client, &endpoint).await;
        crate::role::secret::test_delete_accessor(&client, &endpoint, accessor.as_str()).await;
        crate::role::secret::test_custom(&client, &endpoint).await;
        crate::role::secret::test_delete(&client, &endpoint, "test").await;

        // Test auth
        test_login(&client, &endpoint).await;

        crate::role::test_delete(&client, &endpoint).await;
    })
}

pub async fn test_login(client: &impl Client, endpoint: &AppRoleEndpoint) {
    use vaultrs::auth::approle::role;

    let role_id_resp =
        role::read_id(client, endpoint.path.as_str(), endpoint.role_name.as_str()).await;
    assert!(role_id_resp.is_ok());
    let role_id = role_id_resp.unwrap().role_id;

    let secret_id_resp = role::secret::generate(
        client,
        endpoint.path.as_str(),
        endpoint.role_name.as_str(),
        None,
    )
    .await;
    assert!(secret_id_resp.is_ok());
    let secret_id = secret_id_resp.unwrap().secret_id;

    let resp = approle::login(
        client,
        endpoint.path.as_str(),
        role_id.as_str(),
        secret_id.as_str(),
    )
    .await;
    assert!(resp.is_ok());
}

mod role {
    use super::{AppRoleEndpoint, Client};
    use vaultrs::{api::auth::approle::requests::SetAppRoleRequest, auth::approle::role};

    pub async fn test_delete(client: &impl Client, endpoint: &AppRoleEndpoint) {
        let res = role::delete(client, endpoint.path.as_str(), endpoint.role_name.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_list(client: &impl Client, endpoint: &AppRoleEndpoint) {
        let res = role::list(client, endpoint.path.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_read(client: &impl Client, endpoint: &AppRoleEndpoint) {
        let res = role::read(client, endpoint.path.as_str(), endpoint.role_name.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_set(client: &impl Client, endpoint: &AppRoleEndpoint) {
        let res = role::set(
            client,
            endpoint.path.as_str(),
            endpoint.role_name.as_str(),
            Some(&mut SetAppRoleRequest::builder().token_ttl("10m")),
        )
        .await;
        assert!(res.is_ok());
    }

    pub async fn test_read_id(client: &impl Client, endpoint: &AppRoleEndpoint) {
        let res = role::read_id(client, endpoint.path.as_str(), endpoint.role_name.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_update_id(client: &impl Client, endpoint: &AppRoleEndpoint) {
        let res = role::update_id(
            client,
            endpoint.path.as_str(),
            endpoint.role_name.as_str(),
            "test",
        )
        .await;
        assert!(res.is_ok());
    }

    pub mod secret {
        use super::{AppRoleEndpoint, Client};

        use vaultrs::{
            api::auth::approle::requests::GenerateNewSecretIDRequest, auth::approle::role::secret,
        };

        pub async fn test_custom(client: &impl Client, endpoint: &AppRoleEndpoint) {
            let res = secret::custom(
                client,
                endpoint.path.as_str(),
                endpoint.role_name.as_str(),
                "test",
                None,
            )
            .await;
            assert!(res.is_ok());
        }

        pub async fn test_delete(client: &impl Client, endpoint: &AppRoleEndpoint, id: &str) {
            let res = secret::delete(
                client,
                endpoint.path.as_str(),
                endpoint.role_name.as_str(),
                id,
            )
            .await;
            assert!(res.is_ok());
        }

        pub async fn test_delete_accessor(
            client: &impl Client,
            endpoint: &AppRoleEndpoint,
            accessor: &str,
        ) {
            let res = secret::delete_accessor(
                client,
                endpoint.path.as_str(),
                endpoint.role_name.as_str(),
                accessor,
            )
            .await;
            assert!(res.is_ok());
        }

        pub async fn test_generate(
            client: &impl Client,
            endpoint: &AppRoleEndpoint,
        ) -> (String, String) {
            let res = secret::generate(
                client,
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

        pub async fn test_list(client: &impl Client, endpoint: &AppRoleEndpoint) {
            let res =
                secret::list(client, endpoint.path.as_str(), endpoint.role_name.as_str()).await;
            assert!(res.is_ok());
        }

        pub async fn test_read(client: &impl Client, endpoint: &AppRoleEndpoint, id: &str) {
            let res = secret::read(
                client,
                endpoint.path.as_str(),
                endpoint.role_name.as_str(),
                id,
            )
            .await;
            assert!(res.is_ok());
        }

        pub async fn test_read_accessor(
            client: &impl Client,
            endpoint: &AppRoleEndpoint,
            accessor: &str,
        ) {
            let res = secret::read_accessor(
                client,
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

async fn setup(server: &VaultServer, client: &impl Client) -> Result<AppRoleEndpoint, ClientError> {
    debug!("setting up AppRole auth engine");
    let path = "approle_test";
    let role_name = "test";

    // Mount the AppRole auth engine
    server.mount_auth(client, path, "approle").await?;

    Ok(AppRoleEndpoint {
        path: path.to_string(),
        role_name: role_name.to_string(),
    })
}
