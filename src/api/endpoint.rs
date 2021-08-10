use reqwest::{blocking::Response, Url};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde::Deserialize;
use std::fmt::Debug;

use crate::{client::VaultClient, enums::RequestType, error::ClientError};

const HTTP_SUCCESS_CODES: [u16; 2] = [200, 204];

pub trait Endpoint: Debug {
    type RequestData: Serialize;
    type Response: DeserializeOwned;

    fn action(&self) -> String;
    fn method(&self) -> RequestType;
    fn data(&self) -> Option<&Self::RequestData>;

    fn build(
        &self,
        client: &VaultClient,
        mount: &str,
    ) -> Result<reqwest::blocking::Request, ClientError> {
        client
            .request(
                self.method(),
                self.build_url(client, mount, self.action().as_str())?,
                self.data(),
            )
            .build()
            .map_err(|e| ClientError::RequestBuildError { source: e })
    }

    fn build_url(
        &self,
        client: &VaultClient,
        mount: &str,
        action: &str,
    ) -> Result<reqwest::Url, ClientError> {
        let mut url = Url::parse(client.settings.address.as_str())
            .map_err(|e| ClientError::UrlBuildError { source: e })?;
        url.path_segments_mut()
            .unwrap()
            .push(format!("v{}", client.settings.version).as_str())
            .push(mount)
            .extend(action.split("/"));
        Ok(url)
    }

    fn send(&self, client: &VaultClient, mount: &str) -> Result<Response, ClientError> {
        let request = self.build(client, mount)?;
        client
            .execute(request)
            .map_err(|e| ClientError::RequestError { source: e })
    }

    fn execute(
        &self,
        client: &VaultClient,
        mount: &str,
    ) -> Result<Option<Self::Response>, ClientError> {
        // Send request
        let response = self.send(client, mount)?;
        let status_code = response.status().as_u16();
        let url = response.url().to_string();

        // Don't return an empty response
        if status_code == 204 {
            return Ok(None);
        }

        // Unwrap response content
        let content = &response
            .text()
            .map_err(|e| ClientError::ResponseError { source: e })?;

        // Check response
        if !HTTP_SUCCESS_CODES.contains(&status_code) {
            return Err(ClientError::ServerResponseError {
                url: url,
                code: status_code,
                error: serde_json::from_str(&content).ok(),
            });
        }

        // Check for response content
        if content.is_empty() {
            return Ok(None);
        }

        // Parse response content
        serde_json::from_str(content).map_err(|e| ClientError::ParseError {
            source: e,
            content: content.to_string(),
        })
    }
}

#[derive(Deserialize, Debug)]
pub struct EndpointResult<T> {
    pub request_id: String,
    pub lease_id: String,
    pub renewable: bool,
    pub lease_duration: u32,
    pub data: T,
}

#[derive(Deserialize, Debug)]
pub struct ErrorEndpointResult {
    errors: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct EmptyEndpointResult {}

#[derive(serde::Serialize, Debug)]
pub struct EmptyEndpointData {}
