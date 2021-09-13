//! # vaultrs-test
//!
//! <p align="center">
//!     <a href="https://crates.io/crates/vaultrs-test">
//!         <img src="https://img.shields.io/crates/v/vaultrs-test">
//!     </a>
//!     <a href="https://docs.rs/vaultrs-test">
//!         <img src="https://img.shields.io/docsrs/vaultrs-test" />
//!     </a>
//!     <a href="https://www.vaultproject.io/">
//!         <img src="https://img.shields.io/badge/Vault-1.8.2-green" />
//!     </a>
//!     <a href="https://github.com/jmgilman/vaultrs-test/actions/workflows/ci.yml">
//!         <img src="https://github.com/jmgilman/vaultrs-test/actions/workflows/ci.yml/badge.svg"/>
//!     </a>
//! </p>
//!
//! > A test suite for testing against [Hashicorp Vault][1] servers.
//!
//! ## Installation
//!
//! Add `vaultrs-test` as a developemnt depdendency to your cargo.toml:
//! ```ignore
//! [dev-dependencies]
//! vaultrs-test = "0.2.0"
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use vaultrs_test::docker::{Server, ServerConfig};
//! use vaultrs_test::{VaultServer, VaultServerConfig};
//!
//! // Configures a container to run Vault server v1.8.2
//! let config = VaultServerConfig::default(Some("1.8.2"));
//!
//! // Creates a test instance to run the container in
//! let instance = config.to_instance();
//!
//! // Runs the test instance, passing in details about the container environment
//! instance.run(|ops| async move {
//!     // The code below only runs after the container is verified running
//!
//!     // Creates an abstraction for interacting with the Vault container
//!     let server = VaultServer::new(&ops, &config);
//!
//!     // Run test code against container
//! })
//!
//! // Container is cleaned up at this point
//! ```
//!
//! ## Testing
//!
//! Run tests with cargo:
//!
//! ```ignore
//! cargo test
//! ```
//!
//! [1]: https://www.vaultproject.io/
#[macro_use]
extern crate tracing;

pub mod docker;
pub mod oidc;
pub mod vault;

pub use docker::TestInstance;
pub use vault::VaultServer;
pub use vault::VaultServerConfig;

#[cfg(test)]
mod tests {
    use crate::{
        docker::{Server, ServerConfig},
        oidc::{OIDCServer, OIDCServerConfig},
        vault::{VaultServer, VaultServerConfig},
        TestInstance,
    };

    #[test]
    #[tracing_test::traced_test]
    fn test_new_instance() {
        let oidc_config = OIDCServerConfig::default(Some("0.3.4"));
        let vault_config = VaultServerConfig::default(Some("1.8.2"));
        let instance = TestInstance::new(vec![oidc_config.to_comp(), vault_config.to_comp()]);

        instance.run(|ops| async move {
            let oidc_server = OIDCServer::new(&ops, &oidc_config);
            let res = reqwest::get(oidc_server.address).await;
            assert!(res.is_ok());

            let vault_server = VaultServer::new(&ops, &vault_config);
            let res = reqwest::get(vault_server.address).await;
            assert!(res.is_ok());
        })
    }
}
