use crate::docker::{Server, ServerConfig, TestInstance};
use dockertest::{waitfor, Composition, DockerOperations, Image, PullPolicy, Source};

#[derive(Clone)]
pub struct OIDCServerConfig {
    pub handle: String,
    pub timeout: u16,
    pub port: u32,
    pub version: String,
}

impl ServerConfig for OIDCServerConfig {
    fn to_comp(&self) -> Composition {
        const IMAGE_NAME: &str = "ghcr.io/navikt/mock-oauth2-server";
        const IMAGE_PORT: u32 = 8080;
        const WAIT_MESSAGE: &str = "started server on address";

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
        comp.port_map(IMAGE_PORT, self.port);
        comp.with_wait_for(wait).with_container_name(&self.handle)
    }

    fn to_instance(&self) -> TestInstance {
        TestInstance::new(vec![self.to_comp()])
    }
}

impl OIDCServerConfig {
    pub fn new(handle: &str, port: u32, version: &str, timeout: u16) -> OIDCServerConfig {
        OIDCServerConfig {
            handle: handle.to_string(),
            port,
            timeout,
            version: version.to_string(),
        }
    }

    /// If version is [None], defaults to `latest`.
    pub fn default(version: Option<&str>) -> OIDCServerConfig {
        OIDCServerConfig {
            handle: String::from("vaultrs-oidc"),
            port: 8080,
            timeout: 15,
            version: version
                .map(|v| v.to_string())
                .unwrap_or_else(|| String::from("latest")),
        }
    }
}

pub struct OIDCServer {
    pub address: String,
    pub address_internal: String,
    pub config: OIDCServerConfig,
}

impl Server for OIDCServer {
    type Config = OIDCServerConfig;

    fn new(opts: &DockerOperations, config: &Self::Config) -> Self {
        let cont = opts.handle(config.handle.as_str());
        let address = format!("http://localhost:{}", config.port);
        let address_internal = format!("http://{}:{}", cont.ip(), config.port);
        OIDCServer {
            address,
            address_internal,
            config: config.clone(),
        }
    }
}
