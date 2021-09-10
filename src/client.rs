use crate::api::AuthInfo;
use crate::api::{token::responses::LookupTokenResponse, EndpointMiddleware};
use crate::error::ClientError;
use async_trait::async_trait;
use rustify::clients::reqwest::Client as HTTPClient;
use std::{env, fs};
use url::Url;

/// Valid URL schemes that can be used for a Vault server address
const VALID_SCHEMES: [&str; 2] = ["http", "https"];

/// The client interface capabale of interacting with API functions
#[async_trait]
pub trait Client: Send + Sync + Sized {
    /// Returns the underlying HTTP client being used for API calls
    fn http(&self) -> &HTTPClient;

    /// Returns the middleware to be used when executing API calls
    fn middle(&self) -> &EndpointMiddleware;

    /// Returns the settings used to configure this client
    fn settings(&self) -> &VaultClientSettings;

    /// Sets the underlying token for this client
    fn set_token(&mut self, token: &str);

    /// Looks up the current token being used by this client
    async fn lookup(&self) -> Result<LookupTokenResponse, ClientError> {
        crate::token::lookup_self(self).await
    }

    /// Renews the current token being used by this client
    async fn renew(&self, increment: Option<&str>) -> Result<AuthInfo, ClientError> {
        crate::token::renew_self(self, increment).await
    }

    /// Revokes the current token being used by this client
    async fn revoke(&self) -> Result<(), ClientError> {
        crate::token::revoke_self(self).await
    }

    /// Returns the status of the configured Vault server
    async fn status(&self) -> Result<crate::sys::ServerStatus, ClientError> {
        crate::sys::status(self).await
    }
}

/// A client which can be used to execute calls against a Vault server.
///
/// A vault client is configured using [VaultClientSettings] and will
/// automatically configure a backing instance of a [HTTPClient] which is
/// used for executing [Endpoints][rustify::endpoint::Endpoint].
pub struct VaultClient {
    pub http: HTTPClient,
    pub middle: EndpointMiddleware,
    pub settings: VaultClientSettings,
}

#[async_trait]
impl Client for VaultClient {
    fn http(&self) -> &HTTPClient {
        &self.http
    }

    fn middle(&self) -> &EndpointMiddleware {
        &self.middle
    }

    fn settings(&self) -> &VaultClientSettings {
        &self.settings
    }

    fn set_token(&mut self, token: &str) {
        self.settings.token = token.to_string();
        self.middle.token = token.to_string();
    }
}

impl VaultClient {
    /// Creates a new [VaultClient] using the given [VaultClientSettings].
    pub fn new(settings: VaultClientSettings) -> Result<VaultClient, ClientError> {
        let mut http_client = reqwest::ClientBuilder::new();

        // Disable TLS checks if specified
        http_client = http_client.danger_accept_invalid_certs(!settings.verify);

        // Adds CA certificates
        for path in &settings.ca_certs {
            let content = std::fs::read(&path).map_err(|e| ClientError::FileReadError {
                source: e,
                path: path.clone(),
            })?;
            let cert = reqwest::Certificate::from_pem(&content).map_err(|e| {
                ClientError::ParseCertificateError {
                    source: e,
                    path: path.clone(),
                }
            })?;

            http_client = http_client.add_root_certificate(cert);
        }

        // Configures middleware for endpoints to append API version and token
        let version_str = format!("v{}", settings.version);
        let middle = EndpointMiddleware {
            token: settings.token.clone(),
            version: version_str,
            wrap: None,
        };

        let http_client = http_client
            .build()
            .map_err(|e| ClientError::RestClientBuildError { source: e })?;
        let http = HTTPClient::new(settings.address.as_str(), http_client);
        Ok(VaultClient {
            settings,
            middle,
            http,
        })
    }
}

/// Contains settings for configuring a [VaultClient].
///
/// Most settings that are not directly configured will have their default value
/// pulled from their respective environment variables. Specifically:
///
/// * `address`: VAULT_ADDR
/// * `ca_certs: VAULT_CACERT / VAULT_CAPATH
/// * `token`: VAULT_TOKEN
/// * verify`: VAULT_SKIP_VERIFY
///
/// The `address` is validated when the settings are built and will throw an
/// error if the format is invalid.
#[derive(Builder, Clone, Debug)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct VaultClientSettings {
    #[builder(setter(into), default = "self.default_address()")]
    pub address: String,
    #[builder(default = "self.default_ca_certs()")]
    pub ca_certs: Vec<String>,
    #[builder(setter(into), default = "self.default_token()")]
    pub token: String,
    #[builder(default = "self.default_verify()")]
    pub verify: bool,
    #[builder(setter(into, strip_option), default = "1")]
    pub version: u8,
    #[builder(default = "false")]
    pub wrapping: bool,
}

impl VaultClientSettingsBuilder {
    fn default_address(&self) -> String {
        env::var("VAULT_ADDR").unwrap_or_else(|_e| String::from("http://127.0.0.1:8200"))
    }

    fn default_token(&self) -> String {
        env::var("VAULT_TOKEN").unwrap_or_else(|_e| String::from(""))
    }

    fn default_verify(&self) -> bool {
        env::var("VAULT_SKIP_VERIFY").is_err()
    }

    fn default_ca_certs(&self) -> Vec<String> {
        let mut paths: Vec<String> = Vec::new();

        if let Ok(s) = env::var("VAULT_CACERT") {
            paths.push(s);
        }

        if let Ok(s) = env::var("VAULT_CAPATH") {
            if let Ok(p) = fs::read_dir(s) {
                for path in p {
                    paths.push(path.unwrap().path().to_str().unwrap().to_string())
                }
            }
        }

        paths
    }

    fn validate(&self) -> Result<(), String> {
        // Verify URL is valid
        let address = self.address.as_ref().unwrap().as_str();
        let url = Url::parse(address).map_err(|_| format!("Invalid URL format: {}", address))?;

        // Verify scheme is valid HTTP endpoint
        if !VALID_SCHEMES.contains(&url.scheme()) {
            Err(format!("Invalid scheme for HTTP URL: {}", url.scheme()))
        } else {
            Ok(())
        }
    }
}
