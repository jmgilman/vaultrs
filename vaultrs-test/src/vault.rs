use crate::docker::{Server, ServerConfig, TestInstance};
use dockertest::{waitfor, Composition, DockerOperations, Image, PullPolicy, Source};
use vaultrs::{
    api::sys::requests::{
        EnableAuthDataConfig, EnableAuthRequest, EnableEngineDataConfig, EnableEngineRequest,
    },
    client::{VaultClient, VaultClientSettingsBuilder},
    error::ClientError,
    sys::{auth, mount},
};

/// Configuration for bringing up a container running a dev instance of Vault.
#[derive(Clone)]
pub struct VaultServerConfig {
    pub handle: String,
    pub timeout: u16,
    pub token: String,
    pub port: u32,
    pub version: String,
}

impl ServerConfig for VaultServerConfig {
    fn to_comp(&self) -> Composition {
        const IMAGE_NAME: &str = "vault";
        const IMAGE_PORT: u32 = 8200;
        const WAIT_MESSAGE: &str =
            "Development mode should NOT be used in production installations!";

        const PULL_POLICY: PullPolicy = PullPolicy::IfNotPresent;
        const SOURCE: Source = Source::DockerHub(PULL_POLICY);

        // Build image
        let image = Image::with_repository(IMAGE_NAME)
            .source(SOURCE)
            .tag(&self.version);

        // Build composition
        let wait = Box::new(waitfor::MessageWait {
            message: String::from(WAIT_MESSAGE),
            source: waitfor::MessageSource::Stdout,
            timeout: self.timeout,
        });

        let mut comp = Composition::with_image(image);
        comp.env("VAULT_DEV_ROOT_TOKEN_ID", &self.token);
        comp.port_map(IMAGE_PORT, self.port);
        comp.with_wait_for(wait).with_container_name(&self.handle)
    }

    fn to_instance(&self) -> TestInstance {
        TestInstance::new(vec![self.to_comp()])
    }
}

impl VaultServerConfig {
    pub fn new(
        handle: &str,
        port: u32,
        version: &str,
        token: &str,
        timeout: u16,
    ) -> VaultServerConfig {
        VaultServerConfig {
            handle: handle.to_string(),
            port,
            timeout,
            token: token.to_string(),
            version: version.to_string(),
        }
    }

    /// If version is [None], defaults to `latest`.
    pub fn default(version: Option<&str>) -> VaultServerConfig {
        VaultServerConfig {
            handle: String::from("vaultrs-vault"),
            port: 8300,
            timeout: 15,
            token: String::from("test"),
            version: version
                .map(|v| v.to_string())
                .unwrap_or_else(|| String::from("latest")),
        }
    }
}

/// Represents a running instance of a Vault server.
///
/// This should be built after the [TestInstance] has started and will use the
/// details from a [VaultServerConfig] to instantiate a new [VaultClient] which
/// can be used for interacting with the Vault server running in the container.
pub struct VaultServer {
    pub address: String,
    pub address_internal: String,
    pub client: VaultClient,
    pub config: VaultServerConfig,
}

impl Server for VaultServer {
    type Config = VaultServerConfig;

    fn new(opts: &DockerOperations, config: &Self::Config) -> Self {
        let cont = opts.handle(config.handle.as_str());
        let address = format!("http://localhost:{}", config.port);
        let address_internal = format!("http://{}:{}", cont.ip(), config.port);
        let client = VaultClient::new(
            VaultClientSettingsBuilder::default()
                .address(address.clone())
                .token(config.token.clone())
                .build()
                .unwrap(),
        )
        .unwrap();

        VaultServer {
            address,
            address_internal,
            client,
            config: config.clone(),
        }
    }
}

impl VaultServer {
    /// Mounts a new instance of the requested secret engine at the given path.
    pub async fn mount_secret(&self, path: &str, engine: &str) -> Result<(), ClientError> {
        mount::enable(&self.client, path, engine, None).await
    }

    /// Mounts a new instance of the requested secret engine at the given path
    /// using a configuration.
    pub async fn mount_secret_with_config(
        &self,
        path: &str,
        engine: &str,
        config: EnableEngineDataConfig,
    ) -> Result<(), ClientError> {
        mount::enable(
            &self.client,
            path,
            engine,
            Some(EnableEngineRequest::builder().config(config)),
        )
        .await
    }

    /// Mounts a new instance of the requested auth engine at the given path.
    pub async fn mount_auth(&self, path: &str, engine: &str) -> Result<(), ClientError> {
        auth::enable(&self.client, path, engine, None).await
    }

    /// Mounts a new instance of the requested auth engine at the given path
    /// using a configuration.
    pub async fn mount_auth_with_config(
        &self,
        path: &str,
        engine: &str,
        config: EnableAuthDataConfig,
    ) -> Result<(), ClientError> {
        auth::enable(
            &self.client,
            path,
            engine,
            Some(EnableAuthRequest::builder().config(config)),
        )
        .await
    }

    pub fn new_client(&self) -> VaultClient {
        VaultClient::new(
            VaultClientSettingsBuilder::default()
                .address(self.address.clone())
                .token(self.config.token.clone())
                .build()
                .unwrap(),
        )
        .unwrap()
    }
}
