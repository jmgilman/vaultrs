use crate::api::AuthInfo;
use crate::api::{token::responses::LookupTokenResponse, EndpointMiddleware};
use crate::error::ClientError;
use async_trait::async_trait;
pub use reqwest::Identity;
use rustify::clients::reqwest::Client as HTTPClient;
use std::time::Duration;
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
    #[instrument(skip(settings), err)]
    pub fn new(settings: VaultClientSettings) -> Result<VaultClient, ClientError> {
        #[cfg(not(feature = "rustls"))]
        let mut http_client = reqwest::ClientBuilder::new();

        #[cfg(feature = "rustls")]
        let mut http_client = reqwest::ClientBuilder::new().use_rustls_tls();

        // Optionally set timeout on client
        http_client = if let Some(timeout) = settings.timeout {
            http_client.timeout(timeout)
        } else {
            http_client
        };

        // Disable TLS checks if specified
        if !settings.verify {
            event!(tracing::Level::WARN, "Disabling TLS verification");
        }
        http_client = http_client.danger_accept_invalid_certs(!settings.verify);

        // Adds CA certificates
        for path in &settings.ca_certs {
            let content = std::fs::read(path).map_err(|e| ClientError::FileReadError {
                source: e,
                path: path.clone(),
            })?;
            let cert = reqwest::Certificate::from_pem(&content).map_err(|e| {
                ClientError::ParseCertificateError {
                    source: e,
                    path: path.clone(),
                }
            })?;

            debug!("Importing CA certificate from {}", path);
            http_client = http_client.add_root_certificate(cert);
        }

        // Adds client certificates
        if let Some(identity) = &settings.identity {
            http_client = http_client.identity(identity.clone());
        }

        // Configures middleware for endpoints to append API version and token
        debug!("Using API version {}", settings.version);
        let version_str = format!("v{}", settings.version);
        let middle = EndpointMiddleware {
            token: settings.token.clone(),
            version: version_str,
            wrap: None,
            namespace: settings.namespace.clone(),
            strict_http: settings.strict_http,
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
    #[builder(setter(custom), default = "self.default_address()?")]
    pub address: Url,
    #[builder(default = "self.default_ca_certs()")]
    pub ca_certs: Vec<String>,
    #[builder(default = "self.default_identity()")]
    pub identity: Option<Identity>,
    #[builder(default)]
    pub timeout: Option<Duration>,
    #[builder(setter(into), default = "self.default_token()")]
    pub token: String,
    #[builder(default = "self.default_verify()")]
    pub verify: bool,
    #[builder(setter(into, strip_option), default = "1")]
    pub version: u8,
    #[builder(default = "false")]
    pub wrapping: bool,
    #[builder(default)]
    pub namespace: Option<String>,
    #[builder(default = "false")]
    pub strict_http: bool,
}

impl VaultClientSettingsBuilder {
    /// Set an address for vault. Note that if not set, it will default
    /// to the `VAULT_ADDR` environment variable and if that is not set either,
    /// it will default to `http://127.0.0.1:8200`.
    ///
    /// # Panics
    ///
    /// The setter will panic if the address given contains an invalid URL format.
    pub fn address<T>(&mut self, address: T) -> &mut Self
    where
        T: AsRef<str>,
    {
        let url = Url::parse(address.as_ref())
            .map_err(|_| format!("Invalid URL format: {}", address.as_ref()))
            .unwrap();
        self.address = Some(url);
        self
    }

    pub fn set_namespace(&mut self, str: String) -> &mut Self {
        self.namespace = Some(Some(str));
        self
    }

    fn default_address(&self) -> Result<Url, String> {
        let address = if let Ok(address) = env::var("VAULT_ADDR") {
            debug!("Using vault address from $VAULT_ADDR: {address}");
            address
        } else {
            debug!("Using default vault address http://127.0.0.1:8200");
            String::from("http://127.0.0.1:8200")
        };
        let url = Url::parse(&address);
        let url = url.map_err(|_| format!("Invalid URL format: {}", &address))?;
        // validation in derive_builder does not happen for defaults,
        // so we need to do it ourselves, here:
        self.validate_url(&url)?;
        Ok(url)
    }

    fn default_token(&self) -> String {
        match env::var("VAULT_TOKEN") {
            Ok(s) => {
                debug!("Using vault token from $VAULT_TOKEN");
                s
            }
            Err(_) => {
                debug!("Using default empty vault token");
                String::from("")
            }
        }
    }

    fn default_verify(&self) -> bool {
        debug!("Checking TLS verification using $VAULT_SKIP_VERIFY");
        match env::var("VAULT_SKIP_VERIFY") {
            Ok(value) => !matches!(value.to_lowercase().as_str(), "0" | "f" | "false"),
            Err(_) => true,
        }
    }

    fn default_ca_certs(&self) -> Vec<String> {
        let mut paths: Vec<String> = Vec::new();

        if let Ok(s) = env::var("VAULT_CACERT") {
            debug!("Found CA certificate in $VAULT_CACERT");
            paths.push(s);
        }

        if let Ok(s) = env::var("VAULT_CAPATH") {
            debug!("Found CA certificate path in $VAULT_CAPATH");
            if let Ok(p) = fs::read_dir(s) {
                for path in p {
                    paths.push(path.unwrap().path().to_str().unwrap().to_string())
                }
            }
        }

        paths
    }

    fn default_identity(&self) -> Option<reqwest::Identity> {
        // Default value can be set from environment
        let env_client_cert = env::var("VAULT_CLIENT_CERT").unwrap_or_default();
        let env_client_key = env::var("VAULT_CLIENT_KEY").unwrap_or_default();

        if env_client_cert.is_empty() || env_client_key.is_empty() {
            debug!("No client certificate (env VAULT_CLIENT_CERT & VAULT_CLIENT_KEY are not set)");
            return None;
        }

        #[cfg(feature = "rustls")]
        {
            let mut client_cert = match fs::read(&env_client_cert) {
                Ok(content) => content,
                Err(err) => {
                    error!("error reading client cert '{}': {}", env_client_cert, err);
                    return None;
                }
            };

            let mut client_key = match fs::read(&env_client_key) {
                Ok(content) => content,
                Err(err) => {
                    error!("error reading client key '{}': {}", env_client_key, err);
                    return None;
                }
            };

            // concat certificate and key
            client_cert.append(&mut client_key);

            match reqwest::Identity::from_pem(&client_cert) {
                Ok(pkcs8) => return Some(pkcs8),
                Err(err) => error!("error creating identity: {}", err),
            };
        }

        #[cfg(feature = "native-tls")]
        {
            error!("Client certificates not implemented for native-tls");
        }

        None
    }

    fn validate(&self) -> Result<(), String> {
        // Verify URL is valid
        if let Some(url) = &self.address {
            self.validate_url(url)
        } else {
            Ok(())
        }
    }

    fn validate_url(&self, url: &Url) -> Result<(), String> {
        // Verify scheme is valid HTTP endpoint
        if !VALID_SCHEMES.contains(&url.scheme()) {
            Err(format!("Invalid scheme for HTTP URL: {}", url.scheme()))
        } else {
            Ok(())
        }
    }
}
