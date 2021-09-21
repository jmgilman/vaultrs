#[macro_use]
extern crate tracing;

mod common;

use common::{PostgresServer, VaultServer, VaultServerHelper};
use test_env_log::test;
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

        // Test roles
        crate::role::test_set(&client, &endpoint).await;
        crate::role::test_read(&client, &endpoint).await;
        crate::role::test_list(&client, &endpoint).await;
        crate::role::test_delete(&client, &endpoint).await;
    });
}

mod role {
    use super::{Client, DatabaseEndpoint};
    use vaultrs::{api::database::requests::SetRoleRequest, database::role};

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
        let res = role::set(
            client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
            Some(SetRoleRequest::builder().db_name(&endpoint.connection)),
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

    // Configure connection
    vaultrs::database::connection::postgres(
        client,
        path,
        connection,
        Some(
            PostgreSQLConnectionRequest::builder()
                .plugin_name("postgresql-database-plugin")
                .connection_url(db_server.internal_url().as_str())
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
    })
}
