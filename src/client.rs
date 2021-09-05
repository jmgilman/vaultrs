use crate::api::AuthInfo;
use crate::api::{token::responses::LookupTokenResponse, EndpointMiddleware};
use crate::error::ClientError;
use crate::login::core::{LoginMethod, MultiLoginCallback, MultiLoginMethod};
use async_trait::async_trait;
use rustify::clients::reqwest::Client as HTTPClient;
use std::{env, fs};
use url::Url;

/// Valid URL schemes that can be used for a Vault server address
const VALID_SCHEMES: [&str; 2] = ["http", "https"];

#[async_trait]
pub trait Client: Send + Sync {
    fn http(&self) -> &HTTPClient;

    /// Performs a login using the given method and sets the resulting token to
    /// this client.
    async fn login<M: 'static + LoginMethod>(
        &mut self,
        mount: &str,
        method: &M,
    ) -> Result<(), ClientError>;

    /// Performs the first step of a multi-step login, returning the resulting
    /// callback which must be passed back to the client to finish the login
    /// flow.
    async fn login_multi<M: 'static + MultiLoginMethod>(
        &self,
        mount: &str,
        method: M,
    ) -> Result<M::Callback, ClientError>;

    /// Performs the second step of a multi-step login and sets the resulting
    /// token to this client.
    async fn login_multi_callback<C: 'static + MultiLoginCallback>(
        &mut self,
        mount: &str,
        callback: C,
    ) -> Result<(), ClientError>;

    fn middle(&self) -> &EndpointMiddleware;

    fn settings(&self) -> &VaultClientSettings;
}

/// A client which can be used to execute calls against a Vault server.
///
/// A vault client is configured using [VaultClientSettings] and will
/// automatically configure a backing instance of a [ReqwestClient] which is
/// used for executing [Endpoints][rustify::endpoint::Endpoint]. All requests
/// made will automatically be configured according to how this client is setup
/// (i.e adding the Vault token to requests). All calls using this client are
/// blocking.
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

    /// Performs a login using the given method and sets the resulting token to
    /// this client.
    async fn login<M: 'static + LoginMethod>(
        &mut self,
        mount: &str,
        method: &M,
    ) -> Result<(), ClientError> {
        let info = method.login(self, mount).await?;
        self.settings.token = info.client_token.clone();
        self.middle.token = info.client_token;
        Ok(())
    }

    /// Performs the first step of a multi-step login, returning the resulting
    /// callback which must be passed back to the client to finish the login
    /// flow.
    async fn login_multi<M: 'static + MultiLoginMethod>(
        &self,
        mount: &str,
        method: M,
    ) -> Result<M::Callback, ClientError> {
        method.login(self, mount).await
    }

    /// Performs the second step of a multi-step login and sets the resulting
    /// token to this client.
    async fn login_multi_callback<C: 'static + MultiLoginCallback>(
        &mut self,
        mount: &str,
        callback: C,
    ) -> Result<(), ClientError> {
        let info = callback.callback(self, mount).await?;
        self.settings.token = info.client_token.clone();
        self.middle.token = info.client_token;
        Ok(())
    }

    fn middle(&self) -> &EndpointMiddleware {
        &self.middle
    }

    fn settings(&self) -> &VaultClientSettings {
        &self.settings
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

    /// Performs the first step of a multi-step login, returning the resulting
    /// callback which must be passed back to the client to finish the login
    /// flow.
    pub async fn login_multi<M: MultiLoginMethod>(
        &self,
        mount: &str,
        method: M,
    ) -> Result<M::Callback, ClientError> {
        method.login(self, mount).await
    }

    /// Performs the second step of a multi-step login and sets the resulting
    /// token to this client.
    pub async fn login_multi_callback(
        &mut self,
        mount: &str,
        callback: impl MultiLoginCallback,
    ) -> Result<(), ClientError> {
        let info = callback.callback(self, mount).await?;
        self.settings.token = info.client_token.clone();
        self.middle.token = info.client_token;
        Ok(())
    }

    /// Writes the token configured for this client to the default location.
    pub fn token_to_file(&mut self) -> Result<(), ClientError> {
        let home_dir = dirs::home_dir();
        let token_file = match home_dir {
            Some(d) => d.join(".vault-token"),
            None => {
                return Err(ClientError::FileNotFoundError {
                    path: "$HOME".to_string(),
                })
            }
        };

        let token_file_string = token_file.to_string_lossy().to_string();
        std::fs::write(token_file, self.settings.token.clone()).map_err(|e| {
            ClientError::FileWriteError {
                source: e,
                path: token_file_string,
            }
        })
    }

    /// Looks up the current token being used by this client
    pub async fn lookup(&self) -> Result<LookupTokenResponse, ClientError> {
        crate::token::lookup_self(self).await
    }

    /// Renews the current token being used by this client
    pub async fn renew(&self, increment: Option<&str>) -> Result<AuthInfo, ClientError> {
        crate::token::renew_self(self, increment).await
    }

    /// Revokes the current token being used by this client
    pub async fn revoke(&self) -> Result<(), ClientError> {
        crate::token::revoke_self(self).await
    }

    /// Returns the status of the configured Vault server
    pub async fn status(&self) -> crate::sys::ServerStatus {
        crate::sys::status(self).await
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

    /// Reads a token from the default location and returns it
    pub fn token_from_file() -> Result<String, ClientError> {
        let home_dir = dirs::home_dir();
        let token_file = match home_dir {
            Some(d) => d.join(".vault-token"),
            None => {
                return Err(ClientError::FileNotFoundError {
                    path: "$HOME".to_string(),
                })
            }
        };

        let token_file_string = token_file.to_string_lossy().to_string();
        if !token_file.exists() {
            return Err(ClientError::FileNotFoundError {
                path: token_file_string,
            });
        }

        fs::read_to_string(token_file).map_err(|e| ClientError::FileReadError {
            source: e,
            path: token_file_string.clone(),
        })
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
