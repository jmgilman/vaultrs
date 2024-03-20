// This file is copied from https://github.com/jmgilman/dockertest-server/blob/master/src/servers/hashi/vault.rs
// because of the LOG_MSG condition is only applicable to vault dev server.

use derive_builder::Builder;
use dockertest::{waitfor, Source};
use dockertest_server::{Config, ContainerConfig, Server};
use std::collections::HashMap;

const IMAGE: &str = "vault";
const PORT: u32 = 8300;
const LOG_MSG: &str = "Vault server started!";
const SOURCE: Source = Source::DockerHub;

/// Configuration for creating a Hashicorp Vault server.
///
/// A token with root permissions will automatically be generated using the
/// `token` field. If it's omitted the token will automatically be generated.
///
/// By default the Vault server listens on port 8200 for HTTP requests. This
/// is exposed on the container by default, but the exposed port can be
/// controlled by setting the `port` field.
///
/// See the [Dockerhub](https://hub.docker.com/_/vault) page for more
/// information on the arguments and environment variables that can be used to
/// configure the server.
#[derive(Clone, Default, Builder)]
#[builder(default)]
pub struct VaultServerConfig {
    #[builder(default = "vec![String::from(\"server\")]")]
    pub args: Vec<String>,
    #[builder(default = "HashMap::new()")]
    pub env: HashMap<String, String>,
    #[builder(default = "dockertest_server::server::new_handle(IMAGE)")]
    pub handle: String,
    #[builder(default = "8200")]
    pub port: u32,
    #[builder(default = "15")]
    pub timeout: u16,
    #[builder(default = "String::from(\"latest\")")]
    pub version: String,
}

impl VaultServerConfig {
    pub fn builder() -> VaultServerConfigBuilder {
        VaultServerConfigBuilder::default()
    }
}

impl Config for VaultServerConfig {
    fn into_composition(self) -> dockertest::Composition {
        let ports = vec![(PORT, self.port)];

        let timeout = self.timeout;
        let wait = Box::new(waitfor::MessageWait {
            message: LOG_MSG.into(),
            source: waitfor::MessageSource::Stdout,
            timeout,
        });

        ContainerConfig {
            args: self.args,
            env: self.env,
            handle: self.handle,
            name: IMAGE.into(),
            source: SOURCE,
            version: self.version,
            ports: Some(ports),
            wait: Some(wait),
            bind_mounts: HashMap::new(),
        }
        .into()
    }

    fn handle(&self) -> &str {
        self.handle.as_str()
    }
}

/// A running instance of a Vault server.
///
/// The server URL which is accessible from the local host can be found in `local_address`.
/// Other running containers which need access to this server should use the
/// `address` field instead.
pub struct VaultServer {
    pub external_port: u32,
    pub internal_port: u32,
    pub ip: String,
}

impl VaultServer {
    fn format_address(&self, host: &str, port: u32) -> String {
        format!("{}:{}", host, port)
    }

    fn format_url(&self, host: &str, port: u32) -> String {
        format!("http://{}", self.format_address(host, port))
    }

    /// The external HTTP address
    pub fn external_url(&self) -> String {
        self.format_url("localhost", self.external_port)
    }
}

impl Server for VaultServer {
    type Config = VaultServerConfig;

    fn new(config: &Self::Config, container: &dockertest::RunningContainer) -> Self {
        VaultServer {
            external_port: config.port,
            internal_port: PORT,
            ip: container.ip().to_string(),
        }
    }
}
