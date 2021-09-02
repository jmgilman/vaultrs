use thiserror::Error;

/// The common error type returned by this crate
#[derive(Error, Debug)]
pub enum ClientError {
    #[error("The Vault server returned an error (status code {code})")]
    APIError { code: u16, errors: Vec<String> },
    #[error("Error reading CA certificate: {path}")]
    ReadCertificateError {
        source: std::io::Error,
        path: String,
    },
    #[error("Error parsing value into JSON")]
    JsonParseError { source: serde_json::error::Error },
    #[error("Error parsing CA certificate as PEM encoded certificate: {path}")]
    ParseCertificateError {
        source: reqwest::Error,
        path: String,
    },
    #[error("The request returned an empty response")]
    ResponseEmptyError,
    #[error("The result contained an empty data field")]
    ResponseDataEmptyError,
    #[error("Error parsing response wrapping result")]
    ResponseWrapError,
    #[error("Error configuring REST client")]
    RestClientBuildError { source: reqwest::Error },
    #[error("An error occurred with the request")]
    RestClientError {
        #[from]
        source: rustify::errors::ClientError,
    },
    #[error("The wrapped response doesn't exist or is not longer valid")]
    WrapInvalidError,
}
