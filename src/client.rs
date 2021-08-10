use std::{env, fs, str::FromStr};

use crate::{enums::RequestType, error::ClientError};
use reqwest::{
    blocking::{Request, RequestBuilder, Response},
    Method,
};
use serde::ser::Serialize;
use url::Url;

const VALID_SCHEMES: [&str; 2] = ["http", "https"];

pub struct VaultClient {
    pub http: reqwest::blocking::Client,
    pub settings: VaultClientSettings,
}

impl VaultClient {
    pub fn new(settings: VaultClientSettings) -> Result<VaultClient, ClientError> {
        let http_client = reqwest::blocking::ClientBuilder::new()
            .danger_accept_invalid_certs(!settings.verify)
            .build()
            .map_err(|e| ClientError::ClientBuildError { source: e })?;
        Ok(VaultClient {
            settings,
            http: http_client,
        })
    }

    pub fn request<S: Serialize>(
        &self,
        req_type: RequestType,
        url: Url,
        data: Option<&S>,
    ) -> RequestBuilder {
        let builder = match req_type {
            RequestType::DELETE => match data {
                Some(d) => self.http.delete(url).json(&d),
                None => self.http.delete(url),
            },
            RequestType::GET => self.http.get(url),
            RequestType::HEAD => match data {
                Some(d) => self.http.head(url).json(&d),
                None => self.http.head(url),
            },
            RequestType::LIST => match data {
                Some(d) => self
                    .http
                    .request(Method::from_str("LIST").unwrap(), url)
                    .json(&d),
                None => self.http.request(Method::from_str("LIST").unwrap(), url),
            },
            RequestType::POST => match data {
                Some(d) => self.http.post(url).json(&d),
                None => self.http.post(url),
            },
        };
        self.add_token(builder)
    }

    pub fn execute(&self, request: Request) -> Result<Response, reqwest::Error> {
        self.http.execute(request)
    }

    fn add_token(&self, builder: RequestBuilder) -> RequestBuilder {
        builder.header("X-Vault-Token", &self.settings.token)
    }
}

#[derive(Builder, Debug)]
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
        match env::var("VAULT_SKIP_VERIFY") {
            Ok(_s) => false,
            _ => true,
        }
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
