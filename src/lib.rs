//! # vaultrs
//!
//! > An asynchronous Rust client library for the [Hashicorp Vault][1] API
//!
//! The following backends are currently supported:
//!
//! * Auth
//!   * [AppleRole](https://www.vaultproject.io/docs/auth/approle)
//!   * [AWS](https://www.vaultproject.io/docs/auth/aws)
//!   * [JWT/OIDC](https://www.vaultproject.io/api-docs/auth/jwt)
//!   * [Token](https://www.vaultproject.io/docs/auth/token)
//!   * [Userpass](https://www.vaultproject.io/docs/auth/userpass)
//! * Secrets
//!   * [Databases](https://www.vaultproject.io/api-docs/secret/databases)
//!   * [KV v2](https://www.vaultproject.io/docs/secrets/kv/kv-v2)
//!   * [PKI](https://www.vaultproject.io/docs/secrets/pki)
//!   * [SSH](https://www.vaultproject.io/docs/secrets/ssh)
//!   * [Transit](https://www.vaultproject.io/api-docs/secret/transit)
//! * Sys
//!   * [Health](https://www.vaultproject.io/api-docs/system/health)
//!   * [Policies](https://www.vaultproject.io/api-docs/system/policy)
//!   * [Sealing](https://www.vaultproject.io/api-docs/system/seal)
//!   * [Wrapping](https://www.vaultproject.io/docs/concepts/response-wrapping)
//!
//! See something missing?
//! [Open an issue](https://github.com/jmgilman/vaultrs/issues/new).
//!
//! ## Installation
//!
//! Add vaultrs as a dependency to your cargo.toml:
//! ```toml
//! [dependencies]
//! vaultrs = "0.7.0"
//! ```
//!
//! ## Usage
//!
//! ### Basic
//!
//! The client is used to configure the connection to Vault and is required to
//! be passed to all API calls for execution. Behind the scenes it uses an
//! asynchronous client from [Reqwest](https://docs.rs/reqwest/) for
//! communicating to Vault.
//!
//! ```rust
//! use vaultrs::client::{Client, VaultClient, VaultClientSettingsBuilder};
//!
//! // Create a client
//! let mut client = VaultClient::new(
//!     VaultClientSettingsBuilder::default()
//!         .address("https://127.0.0.1:8200")
//!         .token("TOKEN")
//!         .build()
//!         .unwrap()
//! ).unwrap();
//!
//! ```
//!
//! ### Secrets
//!
//! The library currently supports all operations available for version 2 of the
//! key/value store.
//!
//! ```should_panic
//! use serde::{Deserialize, Serialize};
//! use vaultrs::kv2;
//! # use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
//!
//! # let client = VaultClient::new(
//! #     VaultClientSettingsBuilder::default()
//! #         .address("https://127.0.0.1:8200")
//! #         .token("TOKEN")
//! #         .build()
//! #         .unwrap()
//! # ).unwrap();
//! #
//! // Create and read secrets
//! #[derive(Debug, Deserialize, Serialize)]
//! struct MySecret {
//!     key: String,
//!     password: String,
//! }
//!
//! let secret = MySecret {
//!     key: "super".to_string(),
//!     password: "secret".to_string(),
//! };
//! # tokio_test::block_on(async {
//! kv2::set(
//!     &client,
//!     "secret",
//!     "mysecret",
//!     &secret,
//! ).await;
//!
//! let secret: MySecret = kv2::read(&client, "secret", "mysecret").await.unwrap();
//! println!("{}", secret.password) // "secret"
//! # })
//! ```
//!
//! ### PKI
//!
//! The library currently supports all operations available for the PKI secrets
//! engine.
//!
//! ```should_panic
//! use vaultrs::api::pki::requests::GenerateCertificateRequest;
//! # use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
//! use vaultrs::pki::cert;
//!
//! # let client = VaultClient::new(
//! #     VaultClientSettingsBuilder::default()
//! #         .address("https://127.0.0.1:8200")
//! #         .token("TOKEN")
//! #         .build()
//! #         .unwrap()
//! # ).unwrap();
//! #
//! # tokio_test::block_on(async {
//! // Generate a certificate using the PKI backend
//! let cert = cert::generate(
//!     &client,
//!     "pki",
//!     "my_role",
//!     Some(GenerateCertificateRequest::builder().common_name("test.com")),
//! ).await.unwrap();
//! println!("{}", cert.certificate) // "{PEM encoded certificate}"
//! # })
//! ```
//!
//! ### Wrapping
//!
//! All requests implement the ability to be
//! [wrapped](https://www.vaultproject.io/docs/concepts/response-wrapping). These
//! can be passed in your application internally before being unwrapped.
//!
//! ```should_panic
//! use vaultrs::api::ResponseWrapper;
//! use vaultrs::api::sys::requests::ListMountsRequest;
//! # use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
//!
//! # let client = VaultClient::new(
//! #     VaultClientSettingsBuilder::default()
//! #         .address("https://127.0.0.1:8200")
//! #         .token("TOKEN")
//! #         .build()
//! #         .unwrap()
//! # ).unwrap();
//! #
//! # tokio_test::block_on(async {
//! let endpoint = ListMountsRequest::builder().build().unwrap();
//! let wrap_resp = endpoint.wrap(&client).await; // Wrapped response
//! assert!(wrap_resp.is_ok());
//!
//! let wrap_resp = wrap_resp.unwrap(); // Unwrap Result<>
//! let info = wrap_resp.lookup(&client).await; // Check status of this wrapped response
//! assert!(info.is_ok());
//!
//! let unwrap_resp = wrap_resp.unwrap(&client).await; // Unwrap the response
//! assert!(unwrap_resp.is_ok());
//!
//! let info = wrap_resp.lookup(&client).await; // Error: response already unwrapped
//! assert!(info.is_err());
//! # })
//! ```
//!
//! ## Error Handling
//!
//! All errors generated by this crate are wrapped in the `ClientError` enum
//! provided by the crate. API warnings are automatically captured via `log` and
//! API errors are captured and returned as their own variant. Connection
//! related errors from `rusify` are wrapped and returned as a single variant.
//!
//! ## Testing
//!
//! See the the [tests](tests) directory for tests. Run tests with `cargo test`.
//!
//! **Note**: All tests rely on bringing up a local Vault development server
//! using Docker. In order to run tests Docker must be running locally (Docker
//! Desktop works).
//!
//! ## Contributing
//!
//! 1. Fork it (https://github.com/jmgilman/vaultrs/fork)
//! 2. Create your feature branch (git checkout -b feature/fooBar)
//! 3. Commit your changes (git commit -am 'Add some fooBar')
//! 4. Push to the branch (git push origin feature/fooBar)
//! 5. Create a new Pull Request
//!
//! See [CONTRIBUTING](CONTRIBUTING.md) for extensive documentation on the
//! architecture of this library and how to add additional functionality to it.
//!
//! [1]: https://www.vaultproject.io/
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate tracing;

pub mod api;
pub mod auth;
pub mod client;
pub mod database;
pub mod error;
pub mod kv1;
pub mod kv2;
pub mod pki;
pub mod ssh;
pub mod sys;
pub mod token;
pub mod transit;
