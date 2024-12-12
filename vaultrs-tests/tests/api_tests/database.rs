use tracing::debug;
use vaultrs::api::database::requests::PostgreSQLConnectionRequest;
use vaultrs::client::Client;
use vaultrs::sys::mount;

use crate::common::{Test, POSTGRES_PASSWORD, POSTGRES_USER};

#[tokio::test]
async fn test() {
    let test = Test::builder().with_postgres().await;
    let client = test.client();
    let db_url = test.postgres_url().unwrap();
    let endpoint = setup(db_url, client).await;

    // Test reset/rotate
    connection::test_reset(client, &endpoint).await;
    connection::test_rotate(client, &endpoint).await;

    // Test roles
    role::test_set(client, &endpoint).await;
    role::test_read(client, &endpoint).await;
    role::test_creds(client, &endpoint).await;
    role::test_list(client, &endpoint).await;
    role::test_delete(client, &endpoint).await;

    // Test static roles
    static_role::test_set(client, &endpoint).await;
    static_role::test_read(client, &endpoint).await;
    static_role::test_creds(client, &endpoint).await;
    static_role::test_list(client, &endpoint).await;
    static_role::test_rotate(client, &endpoint).await;
    static_role::test_delete(client, &endpoint).await;

    // Test connection
    connection::test_read(client, &endpoint).await;
    connection::test_list(client, &endpoint).await;
    connection::test_delete(client, &endpoint).await;
}

mod connection {
    use super::{Client, DatabaseEndpoint};
    use vaultrs::database::connection;

    pub async fn test_delete(client: &impl Client, endpoint: &DatabaseEndpoint) {
        connection::delete(client, endpoint.path.as_str(), endpoint.connection.as_str())
            .await
            .unwrap();
    }

    pub async fn test_list(client: &impl Client, endpoint: &DatabaseEndpoint) {
        assert!(!connection::list(client, endpoint.path.as_str())
            .await
            .unwrap()
            .keys
            .is_empty());
    }

    pub async fn test_read(client: &impl Client, endpoint: &DatabaseEndpoint) {
        connection::read(client, endpoint.path.as_str(), endpoint.connection.as_str())
            .await
            .unwrap();
    }

    pub async fn test_reset(client: &impl Client, endpoint: &DatabaseEndpoint) {
        dbg!(&endpoint);
        connection::reset(client, endpoint.path.as_str(), endpoint.connection.as_str())
            .await
            .unwrap();
    }

    pub async fn test_rotate(client: &impl Client, endpoint: &DatabaseEndpoint) {
        connection::rotate(client, endpoint.path.as_str(), endpoint.connection.as_str())
            .await
            .unwrap();
    }
}

mod role {
    use super::{Client, DatabaseEndpoint};
    use vaultrs::{api::database::requests::SetRoleRequest, database::role};

    pub async fn test_creds(client: &impl Client, endpoint: &DatabaseEndpoint) {
        role::creds(client, endpoint.path.as_str(), endpoint.role.as_str())
            .await
            .unwrap();
    }

    pub async fn test_delete(client: &impl Client, endpoint: &DatabaseEndpoint) {
        role::delete(client, endpoint.path.as_str(), endpoint.role.as_str())
            .await
            .unwrap();
    }

    pub async fn test_list(client: &impl Client, endpoint: &DatabaseEndpoint) {
        assert!(!role::list(client, endpoint.path.as_str())
            .await
            .unwrap()
            .keys
            .is_empty());
    }

    pub async fn test_read(client: &impl Client, endpoint: &DatabaseEndpoint) {
        role::read(client, endpoint.path.as_str(), endpoint.role.as_str())
            .await
            .unwrap();
    }

    pub async fn test_set(client: &impl Client, endpoint: &DatabaseEndpoint) {
        let sql = r#"CREATE ROLE "{{name}}" WITH LOGIN PASSWORD '{{password}}' VALID UNTIL '{{expiration}}';"#;
        role::set(
            client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
            Some(
                SetRoleRequest::builder()
                    .db_name(&endpoint.connection)
                    .creation_statements(vec![sql.into()]),
            ),
        )
        .await
        .unwrap();
    }
}

mod static_role {
    use super::{Client, DatabaseEndpoint};
    use vaultrs::{api::database::requests::SetStaticRoleRequest, database::static_role};

    pub async fn test_creds(client: &impl Client, endpoint: &DatabaseEndpoint) {
        static_role::creds(
            client,
            endpoint.path.as_str(),
            endpoint.static_role.as_str(),
        )
        .await
        .unwrap();
    }

    pub async fn test_delete(client: &impl Client, endpoint: &DatabaseEndpoint) {
        static_role::delete(
            client,
            endpoint.path.as_str(),
            endpoint.static_role.as_str(),
        )
        .await
        .unwrap();
    }

    pub async fn test_list(client: &impl Client, endpoint: &DatabaseEndpoint) {
        assert!(!static_role::list(client, endpoint.path.as_str())
            .await
            .unwrap()
            .keys
            .is_empty());
    }

    pub async fn test_read(client: &impl Client, endpoint: &DatabaseEndpoint) {
        static_role::read(
            client,
            endpoint.path.as_str(),
            endpoint.static_role.as_str(),
        )
        .await
        .unwrap();
    }

    pub async fn test_set(client: &impl Client, endpoint: &DatabaseEndpoint) {
        static_role::set(
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
        .await
        .unwrap();
    }

    pub async fn test_rotate(client: &impl Client, endpoint: &DatabaseEndpoint) {
        static_role::rotate(
            client,
            endpoint.path.as_str(),
            endpoint.static_role.as_str(),
        )
        .await
        .unwrap();
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

async fn setup(db_url: &str, client: &impl Client) -> DatabaseEndpoint {
    debug!("setting up database secret engine");

    let path = "db_test";
    let connection = "postgres";
    let role = "test";
    let static_role = "static_test";
    // Mount the database secret engine
    mount::enable(client, path, "database", None).await.unwrap();

    // Configure
    let url = format!(
        "postgresql://{{{{username}}}}:{{{{password}}}}@{}/postgres?sslmode=disable",
        db_url
    );
    vaultrs::database::connection::postgres(
        client,
        path,
        connection,
        Some(
            PostgreSQLConnectionRequest::builder()
                .plugin_name("postgresql-database-plugin")
                .connection_url(url)
                .username(POSTGRES_USER)
                .password(POSTGRES_PASSWORD)
                .verify_connection(false)
                .allowed_roles(vec!["*".into()]),
        ),
    )
    .await
    .unwrap();

    DatabaseEndpoint {
        connection: connection.to_string(),
        path: path.to_string(),
        role: role.to_string(),
        static_role: static_role.to_string(),
        username: "postgres".to_string(),
    }
}
