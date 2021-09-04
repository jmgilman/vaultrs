use crate::{api::AuthInfo, client::Client, error::ClientError};
use async_trait::async_trait;

/// Represents a method for logging into Vault which returns a new token.
#[async_trait]
pub trait LoginMethod: Sync + Send {
    async fn login(&self, client: &impl Client, mount: &str) -> Result<AuthInfo, ClientError>;
}

/// Represents a method for logging into Vault which returns a new token but
/// requires two separate steps to complete.
#[async_trait]
pub trait MultiLoginMethod: Sync + Send {
    type Callback: MultiLoginCallback;

    async fn login(&self, client: &impl Client, mount: &str)
        -> Result<Self::Callback, ClientError>;
}

/// Represents the second step of a multi-step login method that returns the
/// authentication info.
#[async_trait]
pub trait MultiLoginCallback: Sync + Send {
    async fn callback(self, client: &impl Client, mount: &str) -> Result<AuthInfo, ClientError>;
}
