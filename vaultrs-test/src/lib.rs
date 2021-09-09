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
//! vaultrs = "0.1.0"
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
//! // The code below only runs after the container is verified running
//! instance.run(|ops| async move {
//!     // Creates an abstraction for interacting with the Vault container
//!     let server = VaultServer::new(&ops, &config);
//!
//!     // Verify server is ready for requests
//!     let status = server.client.status().await;
//!     assert!(matches! { status, vaultrs::sys::ServerStatus::OK });
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
//! ## Contributing
//!
//! Check out the [issues][2] for items neeeding attention or submit your own and
//! then:
//!
//! 1. Fork the repo (https://github.com/jmgilman/vaultrs-test/fork)
//! 2. Create your feature branch (git checkout -b feature/fooBar)
//! 3. Commit your changes (git commit -am 'Add some fooBar')
//! 4. Push to the branch (git push origin feature/fooBar)
//! 5. Create a new Pull Request
//!
//! [1]: https://www.vaultproject.io/
//! [2]: https://github.com/jmgilman/vaultrs-test/issues

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
    fn test() {
        let oidc_config = OIDCServerConfig::default(Some("0.3.4"));
        let vault_config = VaultServerConfig::default(Some("1.8.2"));
        let instance = TestInstance::new(vec![oidc_config.to_comp(), vault_config.to_comp()]);

        instance.run(|ops| async move {
            let oidc_server = OIDCServer::new(&ops, &oidc_config);
            let res = reqwest::get(oidc_server.address).await;
            assert!(res.is_ok());

            let vault_server = VaultServer::new(&ops, &vault_config);
            let status = vault_server.client.status().await;
            assert!(matches! { status, vaultrs::sys::ServerStatus::OK });
        })
    }
}
