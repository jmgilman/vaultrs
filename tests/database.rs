#[macro_use]
extern crate tracing;

mod common;

use common::{VaultServer, VaultServerHelper};
use dockertest_server::servers::database::postgres::PostgresServer;
use test_log::test;
use vaultrs::api::database::requests::PostgreSQLConnectionRequest;
use vaultrs::client::Client;
use vaultrs::error::ClientError;

#[test]
fn test() {
    let test = common::new_db_test();
    test.run(|instance| async move {
        let db_server: PostgresServer = instance.server();
        let vault_server: VaultServer = instance.server();
        let client = vault_server.client();
        let endpoint = setup(&db_server, &vault_server, &client).await.unwrap();

        // Test reset/rotate
        crate::connection::test_reset(&client, &endpoint).await;
        crate::connection::test_rotate(&client, &endpoint).await;

        // Test roles
        crate::role::test_set(&client, &endpoint).await;
        crate::role::test_read(&client, &endpoint).await;
        crate::role::test_creds(&client, &endpoint).await;
        crate::role::test_list(&client, &endpoint).await;
        crate::role::test_delete(&client, &endpoint).await;

        // Test static roles
        crate::static_role::test_set(&client, &endpoint).await;
        crate::static_role::test_read(&client, &endpoint).await;
        crate::static_role::test_creds(&client, &endpoint).await;
        crate::static_role::test_list(&client, &endpoint).await;
        crate::static_role::test_rotate(&client, &endpoint).await;
        crate::static_role::test_delete(&client, &endpoint).await;

        // Test connection
        crate::connection::test_read(&client, &endpoint).await;
        crate::connection::test_list(&client, &endpoint).await;
        crate::connection::test_delete(&client, &endpoint).await;
    });
}

mod connection {
    use super::{Client, DatabaseEndpoint};
    use vaultrs::database::connection;

    pub async fn test_delete(client: &impl Client, endpoint: &DatabaseEndpoint) {
        let res =
            connection::delete(client, endpoint.path.as_str(), endpoint.connection.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_list(client: &impl Client, endpoint: &DatabaseEndpoint) {
        let res = connection::list(client, endpoint.path.as_str()).await;
        assert!(res.is_ok());
        assert!(!res.unwrap().keys.is_empty());
    }

    pub async fn test_read(client: &impl Client, endpoint: &DatabaseEndpoint) {
        let res =
            connection::read(client, endpoint.path.as_str(), endpoint.connection.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_reset(client: &impl Client, endpoint: &DatabaseEndpoint) {
        let res =
            connection::reset(client, endpoint.path.as_str(), endpoint.connection.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_rotate(client: &impl Client, endpoint: &DatabaseEndpoint) {
        let res =
            connection::rotate(client, endpoint.path.as_str(), endpoint.connection.as_str()).await;
        assert!(res.is_ok());
    }
}

mod role {
    use super::{Client, DatabaseEndpoint};
    use vaultrs::{api::database::requests::SetRoleRequest, database::role};

    pub async fn test_creds(client: &impl Client, endpoint: &DatabaseEndpoint) {
        let res = role::creds(client, endpoint.path.as_str(), endpoint.role.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_delete(client: &impl Client, endpoint: &DatabaseEndpoint) {
        let res = role::delete(client, endpoint.path.as_str(), endpoint.role.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_list(client: &impl Client, endpoint: &DatabaseEndpoint) {
        let res = role::list(client, endpoint.path.as_str()).await;
        assert!(res.is_ok());
        assert!(!res.unwrap().keys.is_empty());
    }

    pub async fn test_read(client: &impl Client, endpoint: &DatabaseEndpoint) {
        let res = role::read(client, endpoint.path.as_str(), endpoint.role.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_set(client: &impl Client, endpoint: &DatabaseEndpoint) {
        let sql = r#"CREATE ROLE "{{name}}" WITH LOGIN PASSWORD '{{password}}' VALID UNTIL '{{expiration}}';"#;
        let res = role::set(
            client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
            Some(
                SetRoleRequest::builder()
                    .db_name(&endpoint.connection)
                    .creation_statements(vec![sql.into()]),
            ),
        )
        .await;
        assert!(res.is_ok());
    }
}

mod static_role {
    use super::{Client, DatabaseEndpoint};
    use vaultrs::{api::database::requests::SetStaticRoleRequest, database::static_role};

    pub async fn test_creds(client: &impl Client, endpoint: &DatabaseEndpoint) {
        let res = static_role::creds(
            client,
            endpoint.path.as_str(),
            endpoint.static_role.as_str(),
        )
        .await;
        assert!(res.is_ok());
    }

    pub async fn test_delete(client: &impl Client, endpoint: &DatabaseEndpoint) {
        let res = static_role::delete(
            client,
            endpoint.path.as_str(),
            endpoint.static_role.as_str(),
        )
        .await;
        assert!(res.is_ok());
    }

    pub async fn test_list(client: &impl Client, endpoint: &DatabaseEndpoint) {
        let res = static_role::list(client, endpoint.path.as_str()).await;
        assert!(res.is_ok());
        assert!(!res.unwrap().keys.is_empty());
    }

    pub async fn test_read(client: &impl Client, endpoint: &DatabaseEndpoint) {
        let res = static_role::read(
            client,
            endpoint.path.as_str(),
            endpoint.static_role.as_str(),
        )
        .await;
        assert!(res.is_ok());
    }

    pub async fn test_set(client: &impl Client, endpoint: &DatabaseEndpoint) {
        let res = static_role::set(
            client,
            endpoint.path.as_str(),
            endpoint.static_role.as_str(),
            Some(
                SetStaticRoleRequest::builder()
                    .db_name(&endpoint.connection)
                    .username(&endpoint.username)
                    .rotation_period("10m"),
            ),
        )
        .await;
        assert!(res.is_ok());
    }

    pub async fn test_rotate(client: &impl Client, endpoint: &DatabaseEndpoint) {
        let res = static_role::rotate(
            client,
            endpoint.path.as_str(),
            endpoint.static_role.as_str(),
        )
        .await;
        assert!(res.is_ok());
    }
}

#[derive(Debug)]
pub struct DatabaseEndpoint {
    pub connection: String,
    pub path: String,
    pub role: String,
    pub static_role: String,
    pub username: String,
}

async fn setup(
    db_server: &PostgresServer,
    vault_server: &VaultServer,
    client: &impl Client,
) -> Result<DatabaseEndpoint, ClientError> {
    debug!("setting up database secret engine");

    let path = "db_test";
    let connection = "postgres";
    let role = "test";
    let static_role = "static_test";

    // Mount the database secret engine
    vault_server.mount_secret(client, path, "database").await?;

    // Configure
    let url = format!(
        "postgresql://{{{{username}}}}:{{{{password}}}}@{}/postgres?sslmode=disable",
        db_server.internal_address()
    );
    vaultrs::database::connection::postgres(
        client,
        path,
        connection,
        Some(
            PostgreSQLConnectionRequest::builder()
                .plugin_name("postgresql-database-plugin")
                .connection_url(url)
                .username(&db_server.username)
                .password(&db_server.password)
                .verify_connection(false)
                .allowed_roles(vec!["*".into()]),
        ),
    )
    .await?;

    Ok(DatabaseEndpoint {
        connection: connection.to_string(),
        path: path.to_string(),
        role: role.to_string(),
        static_role: static_role.to_string(),
        username: db_server.username.clone(),
    })
}
