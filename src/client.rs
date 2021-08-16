use crate::error::ClientError;
use rustify::{
    clients::reqwest::{MiddleWare, ReqwestClient},
    endpoint::Endpoint,
};
use std::{env, fs};
use url::Url;

const VALID_SCHEMES: [&str; 2] = ["http", "https"];

struct VaultMiddleWare {
    token: String,
    version: String,
}
impl MiddleWare for VaultMiddleWare {
    fn handle(&self, mut r: reqwest::blocking::Request) -> reqwest::blocking::Request {
        let url_c = r.url().clone();
        let mut segs: Vec<&str> = url_c.path_segments().unwrap().collect();
        segs.insert(0, self.version.as_str());
        r.url_mut()
            .path_segments_mut()
            .unwrap()
            .clear()
            .extend(segs);

        // Adds vault token to all requests
        r.headers_mut().append(
            "X-Vault-Token",
            reqwest::header::HeaderValue::from_str(self.token.as_str()).unwrap(),
        );
        r
    }
}

pub struct VaultClient {
    pub http: ReqwestClient,
    pub settings: VaultClientSettings,
}

impl VaultClient {
    pub fn new(settings: VaultClientSettings) -> Result<VaultClient, ClientError> {
        let http_client = reqwest::blocking::ClientBuilder::new()
            .danger_accept_invalid_certs(!settings.verify)
            .build()
            .map_err(|e| ClientError::RestClientBuildError { source: e })?;

        // Configures middleware for REST client to append API version and token
        let settings_c = settings.clone();
        let version_str = format!("v{}", settings_c.version);
        let rest_client = ReqwestClient::new(
            settings.address.as_str(),
            http_client,
            Box::new(VaultMiddleWare {
                token: settings_c.token,
                version: version_str,
            }),
        );
        Ok(VaultClient {
            settings,
            http: rest_client,
        })
    }

    pub fn execute<E: Endpoint<Response = T>, T>(
        &self,
        endpoint: E,
    ) -> Result<Option<T>, ClientError> {
        endpoint.execute(&self.http).map_err(ClientError::from)
    }
}

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
