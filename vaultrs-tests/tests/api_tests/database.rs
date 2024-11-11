use tracing::debug;
use vaultrs::api::database::requests::PostgreSQLConnectionRequest;
use vaultrs::client::Client;
use vaultrs::error::ClientError;
use vaultrs::sys::mount;

use crate::common::{Test, POSTGRES_PASSWORD, POSTGRES_USER};

#[tokio::test]
async fn test() {
    let test = Test::builder().with_postgres().await;
    let client = test.client();
    let db_url = test.postgres_url().unwrap();
    let endpoint = setup(db_url, client).await.unwrap();

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
        dbg!(&endpoint);
        connection::reset(client, endpoint.path.as_str(), endpoint.connection.as_str())
            .await
            .unwrap();
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

async fn setup(db_url: &str, client: &impl Client) -> Result<DatabaseEndpoint, ClientError> {
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
    dbg!(&url);
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
    .await?;

    Ok(DatabaseEndpoint {
        connection: connection.to_string(),
        path: path.to_string(),
        role: role.to_string(),
        static_role: static_role.to_string(),
        username: "postgres".to_string(),
    })
}
