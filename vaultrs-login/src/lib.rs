use async_trait::async_trait;
use vaultrs::{api::AuthInfo, client::Client, error::ClientError};

pub mod engines;
pub mod method;

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

/// Adds login behavior to [Client]s.
#[async_trait]
pub trait LoginClient {
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
}

#[async_trait]
impl<C: Client> LoginClient for C {
    async fn login<M: 'static + LoginMethod>(
        &mut self,
        mount: &str,
        method: &M,
    ) -> Result<(), ClientError> {
        let info = method.login(self, mount).await?;
        self.set_token(info.client_token.as_str());
        Ok(())
    }

    async fn login_multi<M: 'static + MultiLoginMethod>(
        &self,
        mount: &str,
        method: M,
    ) -> Result<M::Callback, ClientError> {
        method.login(self, mount).await
    }

    async fn login_multi_callback<M: 'static + MultiLoginCallback>(
        &mut self,
        mount: &str,
        callback: M,
    ) -> Result<(), ClientError> {
        let info = callback.callback(self, mount).await?;
        self.set_token(info.client_token.as_str());
        Ok(())
    }
}
