use crate::common::Test;
use tracing::debug;
use vaultrs::auth::approle;
use vaultrs::client::Client;
use vaultrs::sys::auth;

#[tokio::test]
async fn test() {
    let test = Test::builder().await;
    let client = test.client();
    let endpoint = setup(client).await;

    // Test roles
    role::test_set(client, &endpoint).await;
    role::test_read(client, &endpoint).await;
    role::test_list(client, &endpoint).await;
    role::test_read_id(client, &endpoint).await;
    role::test_update_id(client, &endpoint).await;

    // Test secret IDs
    let (id, accessor) = role::secret::test_generate(client, &endpoint).await;
    role::secret::test_read(client, &endpoint, id.as_str()).await;
    role::secret::test_read_accessor(client, &endpoint, accessor.as_str()).await;
    role::secret::test_list(client, &endpoint).await;
    role::secret::test_delete_accessor(client, &endpoint, accessor.as_str()).await;
    role::secret::test_custom(client, &endpoint).await;
    role::secret::test_delete(client, &endpoint, "test").await;

    // Test auth
    test_login(client, &endpoint).await;

    role::test_delete(client, &endpoint).await;
}

pub async fn test_login(client: &impl Client, endpoint: &AppRoleEndpoint) {
    use vaultrs::auth::approle::role;

    let role_id = role::read_id(client, endpoint.path.as_str(), endpoint.role_name.as_str())
        .await
        .unwrap()
        .role_id;

    let secret_id = role::secret::generate(
        client,
        endpoint.path.as_str(),
        endpoint.role_name.as_str(),
        None,
    )
    .await
    .unwrap()
    .secret_id;

    approle::login(
        client,
        endpoint.path.as_str(),
        role_id.as_str(),
        secret_id.as_str(),
    )
    .await
    .unwrap();
}

mod role {
    use super::{AppRoleEndpoint, Client};
    use vaultrs::{api::auth::approle::requests::SetAppRoleRequest, auth::approle::role};

    pub async fn test_delete(client: &impl Client, endpoint: &AppRoleEndpoint) {
        role::delete(client, endpoint.path.as_str(), endpoint.role_name.as_str())
            .await
            .unwrap();
    }

    pub async fn test_list(client: &impl Client, endpoint: &AppRoleEndpoint) {
        role::list(client, endpoint.path.as_str()).await.unwrap();
    }

    pub async fn test_read(client: &impl Client, endpoint: &AppRoleEndpoint) {
        role::read(client, endpoint.path.as_str(), endpoint.role_name.as_str())
            .await
            .unwrap();
    }

    pub async fn test_set(client: &impl Client, endpoint: &AppRoleEndpoint) {
        role::set(
            client,
            endpoint.path.as_str(),
            endpoint.role_name.as_str(),
            Some(&mut SetAppRoleRequest::builder().token_ttl("10m")),
        )
        .await
        .unwrap();
    }

    pub async fn test_read_id(client: &impl Client, endpoint: &AppRoleEndpoint) {
        role::read_id(client, endpoint.path.as_str(), endpoint.role_name.as_str())
            .await
            .unwrap();
    }

    pub async fn test_update_id(client: &impl Client, endpoint: &AppRoleEndpoint) {
        role::update_id(
            client,
            endpoint.path.as_str(),
            endpoint.role_name.as_str(),
            "test",
        )
        .await
        .unwrap();
    }

    pub mod secret {
        use super::{AppRoleEndpoint, Client};

        use vaultrs::{
            api::auth::approle::requests::GenerateNewSecretIDRequest, auth::approle::role::secret,
        };

        pub async fn test_custom(client: &impl Client, endpoint: &AppRoleEndpoint) {
            secret::custom(
                client,
                endpoint.path.as_str(),
                endpoint.role_name.as_str(),
                "test",
                None,
            )
            .await
            .unwrap();
        }

        pub async fn test_delete(client: &impl Client, endpoint: &AppRoleEndpoint, id: &str) {
            secret::delete(
                client,
                endpoint.path.as_str(),
                endpoint.role_name.as_str(),
                id,
            )
            .await
            .unwrap();
        }

        pub async fn test_delete_accessor(
            client: &impl Client,
            endpoint: &AppRoleEndpoint,
            accessor: &str,
        ) {
            secret::delete_accessor(
                client,
                endpoint.path.as_str(),
                endpoint.role_name.as_str(),
                accessor,
            )
            .await
            .unwrap();
        }

        pub async fn test_generate(
            client: &impl Client,
            endpoint: &AppRoleEndpoint,
        ) -> (String, String) {
            let id = secret::generate(
                client,
                endpoint.path.as_str(),
                endpoint.role_name.as_str(),
                Some(
                    &mut GenerateNewSecretIDRequest::builder()
                        .metadata("{ \"tag1\": \"production\" }"),
                ),
            )
            .await
            .unwrap();

            (id.secret_id, id.secret_id_accessor)
        }

        pub async fn test_list(client: &impl Client, endpoint: &AppRoleEndpoint) {
            secret::list(client, endpoint.path.as_str(), endpoint.role_name.as_str())
                .await
                .unwrap();
        }

        pub async fn test_read(client: &impl Client, endpoint: &AppRoleEndpoint, id: &str) {
            secret::read(
                client,
                endpoint.path.as_str(),
                endpoint.role_name.as_str(),
                id,
            )
            .await
            .unwrap();
        }

        pub async fn test_read_accessor(
            client: &impl Client,
            endpoint: &AppRoleEndpoint,
            accessor: &str,
        ) {
            secret::read_accessor(
                client,
                endpoint.path.as_str(),
                endpoint.role_name.as_str(),
                accessor,
            )
            .await
            .unwrap();
        }
    }
}

#[derive(Debug)]
pub struct AppRoleEndpoint {
    pub path: String,
    pub role_name: String,
}

async fn setup(client: &impl Client) -> AppRoleEndpoint {
    debug!("setting up AppRole auth engine");
    let path = "approle_test";
    let role_name = "test";

    // Mount the AppRole auth engine
    auth::enable(client, path, "approle", None).await.unwrap();
    AppRoleEndpoint {
        path: path.to_string(),
        role_name: role_name.to_string(),
    }
}
