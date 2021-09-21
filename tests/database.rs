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
    });
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
                .connection_url(db_server.address.as_str())
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
