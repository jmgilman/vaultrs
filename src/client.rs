use crate::api;
use crate::api::{token::responses::TokenLookupResponse, EndpointMiddleware};
use crate::error::ClientError;
use rustify::clients::reqwest::Client;
use std::{env, fs};
use url::Url;

/// Valid URL schemes that can be used for a Vault server address
const VALID_SCHEMES: [&str; 2] = ["http", "https"];

/// A client which can be used to execute calls against a Vault server.
///
/// A vault client is configured using [VaultClientSettings] and will
/// automatically configure a backing instance of a [ReqwestClient] which is
/// used for executing [Endpoints][rustify::endpoint::Endpoint]. All requests
/// made will automatically be configured according to how this client is setup
/// (i.e adding the Vault token to requests). All calls using this client are
/// blocking.
pub struct VaultClient {
    pub http: Client,
    pub middle: EndpointMiddleware,
    pub settings: VaultClientSettings,
}

impl VaultClient {
    /// Creates a new [VaultClient] using the given [VaultClientSettings].
    pub fn new(settings: VaultClientSettings) -> Result<VaultClient, ClientError> {
        let http_client = reqwest::ClientBuilder::new()
            .danger_accept_invalid_certs(!settings.verify)
            .build()
            .map_err(|e| ClientError::RestClientBuildError { source: e })?;

        // Configures middleware for endpoints to append API version and token
        let version_str = format!("v{}", settings.version);
        let middle = EndpointMiddleware {
            token: settings.token.clone(),
            version: version_str,
            wrap: None,
        };
        let http = Client::new(settings.address.as_str(), http_client);
        Ok(VaultClient {
            settings,
            middle,
            http,
        })
    }

    /// Looks up the current token being used by this client
    pub async fn lookup(&self) -> Result<TokenLookupResponse, ClientError> {
        crate::token::lookup_self(self).await
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
