use std::error::Error;

use thiserror::Error;

use crate::api::endpoint::ErrorEndpointResult;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Error building REST client")]
    ClientBuildError { source: reqwest::Error },
    #[error("Error parsing response data")]
    ParseError {
        source: serde_json::Error,
        content: String,
    },
    #[error("Error sending HTTP request")]
    RequestError { source: reqwest::Error },
    #[error("Error building HTTP request")]
    RequestBuildError { source: reqwest::Error },
    #[error("Error building data for HTTP request")]
    RequestDataBuildError { source: Box<dyn Error> },
    #[error("The request returned an empty response")]
    ResponseEmptyError,
    #[error("Error getting response data")]
    ResponseError { source: reqwest::Error },
    #[error("Server returned {code:?} response")]
    ServerResponseError {
        url: String,
        code: u16,
        error: Option<ErrorEndpointResult>,
    },
    #[error("Error building URL for endpoint address")]
    UrlBuildError { source: url::ParseError },
}
