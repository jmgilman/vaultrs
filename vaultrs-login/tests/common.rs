use async_trait::async_trait;
pub use dockertest_server::servers::auth::{OIDCServer, OIDCServerConfig};
pub use dockertest_server::servers::hashi::{VaultServer, VaultServerConfig};
use dockertest_server::Test;
use vaultrs::{
    api::sys::requests::{
        EnableAuthDataConfig, EnableAuthRequest, EnableEngineDataConfig, EnableEngineRequest,
    },
    client::{Client, VaultClient, VaultClientSettingsBuilder},
    error::ClientError,
    sys::{auth, mount},
};
//use vaultrs_test::VaultServer;

pub const OIDC_PORT: u32 = 9080;
pub const OIDC_VERSION: &str = "0.3.5";
pub const VAULT_PORT: u32 = 8300;
pub const VAULT_VERSION: &str = "1.8.2";

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
                .address(self.local_address.clone())
                .token(self.token.clone())
                .build()
                .unwrap(),
        )
        .unwrap()
    }
}

// Sets up a new Vault test with OIDC support.
pub fn new_test() -> Test {
    let mut test = Test::default();
    let vault_config = VaultServerConfig::builder()
        .port(VAULT_PORT)
        .version(VAULT_VERSION.into())
        .build()
        .unwrap();
    let oidc_config = OIDCServerConfig::builder()
        .port(OIDC_PORT)
        .version(OIDC_VERSION.into())
        .build()
        .unwrap();
    test.register(vault_config);
    test.register(oidc_config);
    test
}
