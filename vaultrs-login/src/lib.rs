//! # vaultrs-login
//!
//! > Adds login support for Vault clients from [vaultrs].
//!
//! ## Installation
//!
//! Add `vaultrs-login` as a dependency to your cargo.toml:
//!
//! ```toml
//! [dependencies]
//! vaultrs-login = "0.2.0"
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
//! use vaultrs_login::LoginClient;
//! use vaultrs_login::engines::approle::AppRoleLogin;
//!
//! // Create a client
//! let mut client = VaultClient::new(
//!     VaultClientSettingsBuilder::default()
//!         .address("https://127.0.0.1:8200")
//!         .build()
//!         .unwrap()
//! ).unwrap();
//!
//! // Use one of the login flows to obtain a token for the client
//! let role_id = String::from("my-role-id");
//! let secret_id = String::from("secret");
//! let login = AppRoleLogin { role_id, secret_id };
//!
//! # tokio_test::block_on(async {
//! client.login("approle", &login).await; // Token is automatically set to client
//! # })
//! ```
//!
//! [vaultrs]: https://docs.rs/vaultrs/latest/vaultrs/

#[macro_use]
extern crate tracing;

use async_trait::async_trait;
use vaultrs::{
    api::AuthInfo,
    client::{Client, VaultClient},
    error::ClientError,
};

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
pub trait LoginClient: Client + Sized {
    /// Performs a login using the given method and sets the resulting token to
    /// this client.
    #[instrument(skip(self, method), err)]
    /// Workaround until <https://github.com/tokio-rs/tracing/issues/2876> is fixed
    #[allow(clippy::blocks_in_conditions)]
    async fn login<M: 'static + LoginMethod>(
        &mut self,
        mount: &str,
        method: &M,
    ) -> Result<(), ClientError> {
        let info = method.login(self, mount).await?;
        self.set_token(info.client_token.as_str());
        Ok(())
    }

    /// Performs the first step of a multi-step login, returning the resulting
    /// callback which must be passed back to the client to finish the login
    /// flow.
    #[instrument(skip(self, method), err)]
    /// Workaround until <https://github.com/tokio-rs/tracing/issues/2876> is fixed
    #[allow(clippy::blocks_in_conditions)]
    async fn login_multi<M: 'static + MultiLoginMethod>(
        &self,
        mount: &str,
        method: M,
    ) -> Result<M::Callback, ClientError> {
        method.login(self, mount).await
    }

    /// Performs the second step of a multi-step login and sets the resulting
    /// token to this client.
    #[instrument(skip(self, callback), err)]
    /// Workaround until <https://github.com/tokio-rs/tracing/issues/2876> is fixed
    #[allow(clippy::blocks_in_conditions)]
    async fn login_multi_callback<C: 'static + MultiLoginCallback>(
        &mut self,
        mount: &str,
        callback: C,
    ) -> Result<(), ClientError> {
        let info = callback.callback(self, mount).await?;
        self.set_token(info.client_token.as_str());
        Ok(())
    }
}

impl LoginClient for VaultClient {}
