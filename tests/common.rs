use async_trait::async_trait;
pub use dockertest_server::servers::hashi::{VaultServer, VaultServerConfig};
use dockertest_server::Test;
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
