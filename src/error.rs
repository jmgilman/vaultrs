use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("The Vault server returned an error")]
    APIError {
        url: String,
        code: u16,
        errors: Vec<String>,
    },
    #[error("The request returned an empty response")]
    ResponseEmptyError,
    #[error("Error configuring REST client")]
    RestClientBuildError { source: reqwest::Error },
    #[error("An error occurred with the request")]
    RestClientError {
        #[from]
        source: rustify::errors::ClientError,
    },
}
