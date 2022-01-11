use async_trait::async_trait;
pub use dockertest_server::servers::cloud::localstack::{LocalStackServer, LocalStackServerConfig};
pub use dockertest_server::servers::database::postgres::{PostgresServer, PostgresServerConfig};
pub use dockertest_server::servers::hashi::{VaultServer, VaultServerConfig};
use dockertest_server::Test;
use std::collections::HashMap;
use tracing::trace;
use vaultrs::{
    api::sys::requests::{
        EnableAuthDataConfig, EnableAuthRequest, EnableEngineDataConfig, EnableEngineRequest,
    },
    client::{Client, VaultClient, VaultClientSettingsBuilder},
    error::ClientError,
    sys::{auth, mount},
};
//use vaultrs_test::VaultServer;

pub const PORT: u32 = 8300;
pub const VERSION: &str = "1.8.2";

#[async_trait]
pub trait VaultServerHelper {
    /// Mounts a new instance of the requested secret engine at the given path.
    async fn mount_secret(
        &self,
        client: &impl Client,
        path: &str,
        engine: &str,
    ) -> Result<(), ClientError>;

    /// Mounts a new instance of the requested secret engine at the given path
    /// using a configuration.
    async fn mount_secret_with_config(
        &self,
        client: &impl Client,
        path: &str,
        engine: &str,
        config: EnableEngineDataConfig,
    ) -> Result<(), ClientError>;

    /// Mounts a new instance of the requested auth engine at the given path.
    async fn mount_auth(
        &self,
        client: &impl Client,
        path: &str,
        engine: &str,
    ) -> Result<(), ClientError>;

    /// Mounts a new instance of the requested auth engine at the given path
    /// using a configuration.
    async fn mount_auth_with_config(
        &self,
        client: &impl Client,
        path: &str,
        engine: &str,
        config: EnableAuthDataConfig,
    ) -> Result<(), ClientError>;

    fn client(&self) -> VaultClient;
}

#[async_trait]
impl VaultServerHelper for VaultServer {
    /// Mounts a new instance of the requested secret engine at the given path.
    async fn mount_secret(
        &self,
        client: &impl Client,
        path: &str,
        engine: &str,
    ) -> Result<(), ClientError> {
        trace!(?path, ?engine, "mounting secret engine");
        mount::enable(client, path, engine, None).await
    }

    /// Mounts a new instance of the requested secret engine at the given path
    /// using a configuration.
    async fn mount_secret_with_config(
        &self,
        client: &impl Client,
        path: &str,
        engine: &str,
        config: EnableEngineDataConfig,
    ) -> Result<(), ClientError> {
        trace!(?path, ?engine, ?config, "mounting secret engine");
        mount::enable(
            client,
            path,
            engine,
            Some(EnableEngineRequest::builder().config(config)),
        )
        .await
    }

    /// Mounts a new instance of the requested auth engine at the given path.
    async fn mount_auth(
        &self,
        client: &impl Client,
        path: &str,
        engine: &str,
    ) -> Result<(), ClientError> {
        trace!(?path, ?engine, "mounting auth engine");
        auth::enable(client, path, engine, None).await
    }

    /// Mounts a new instance of the requested auth engine at the given path
    /// using a configuration.
    async fn mount_auth_with_config(
        &self,
        client: &impl Client,
        path: &str,
        engine: &str,
        config: EnableAuthDataConfig,
    ) -> Result<(), ClientError> {
        trace!(?path, ?engine, ?config, "mounting auth engine");
        auth::enable(
            client,
            path,
            engine,
            Some(EnableAuthRequest::builder().config(config)),
        )
        .await
    }

    fn client(&self) -> VaultClient {
        VaultClient::new(
            VaultClientSettingsBuilder::default()
                .address(self.external_url())
                .token(self.token.clone())
                .build()
                .unwrap(),
        )
        .unwrap()
    }
}

// Sets up a new test.
#[allow(dead_code)]
pub fn new_test() -> Test {
    let mut test = Test::default();
    let config = VaultServerConfig::builder()
        .port(PORT)
        .version(VERSION.into())
        .build()
        .unwrap();
    test.register(config);
    test
}

// Sets up a new database test.
#[allow(dead_code)]
pub fn new_db_test() -> Test {
    let mut test = new_test();
    let db_config = PostgresServerConfig::builder().port(6432).build().unwrap();
    test.register(db_config);
    test
}

// Sets up a new AWS test.
#[allow(dead_code)]
pub fn new_aws_test() -> Test {
    let mut test = new_test();
    let localstack_config = LocalStackServerConfig::builder()
        .env(
            vec![(String::from("SERVICES"), String::from("iam,sts"))]
                .into_iter()
                .collect::<HashMap<_, _>>(),
        )
        .version("0.13.1".to_string())
        .build()
        .unwrap();
    test.register(localstack_config);
    test
}
