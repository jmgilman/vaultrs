//! # vaultrs
//!
//! > An asynchronous Rust client library for the [Hashicorp Vault][1] API
//!
//! The following backends are currently supported:
//!
//! * Auth
//!   * [AppleRole Auth Engine](https://www.vaultproject.io/docs/auth/approle)
//!   * [JWT/OIDC Auth Engine](https://www.vaultproject.io/api-docs/auth/jwt)
//!   * [Token Auth Engine](https://www.vaultproject.io/docs/auth/token)
//!   * [Userpass Auth Engine](https://www.vaultproject.io/docs/auth/userpass)
//! * Secrets
//!   * [KV Secrets Engine V2](https://www.vaultproject.io/docs/secrets/kv/kv-v2)
//!   * [PKI Secrets Engine](https://www.vaultproject.io/docs/secrets/pki)
//!   * [SSH Secrets Engine](https://www.vaultproject.io/docs/secrets/ssh)
//! * Sys
//!   * [Response Wrapping](https://www.vaultproject.io/docs/concepts/response-wrapping)
//!   * [Sealing](https://www.vaultproject.io/api-docs/system/seal)
//!   * [Server Health](https://www.vaultproject.io/api-docs/system/health)
//!
//! **Note**: An additional `oidc` feature can be enabled which provides
//! support for assisting in the OIDC login process by standing up a temporary
//! HTTP server that can respond to OAuth redirects and fetch tokens using the
//! authorization code.
//!
//! ## Installation
//!
//! Add vaultrs as a depdendency to your cargo.toml:
//! ```ignore
//! [dependencies]
//! vaultrs = "0.3.0"
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
//! use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
//! use vaultrs::login::AppRoleLogin;
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
//! // A token can be passed at creation or a new one may be acquired through one
//! // of the login flows.
//! let role_id = String::from("my-role-id");
//! let secret_id = String::from("secret");
//! let login = AppRoleLogin { role_id, secret_id };
//!
//! # tokio_test::block_on(async {
//! client.login("approle", &login).await; // Token is automatically set to client
//! # })
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
//! let secret = kv2::read::<MySecret>(&client, "secret", "mysecret").await.unwrap();
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
//! provided by the crate. API warninings are automatically captured via `log` and
//! API errors are captured and returned as their own variant. Connection related
//! errors from `rusify` are wrapped and returned as a single variant.
//!
//! ## Testing
//!
//! See the the [tests](tests) directory for tests. Run tests with `cargo test`.
//!
//! **Note**: All tests rely on bringing up a local Vault development server using
//! Docker. The Docker CLI must be installed on the machine running the tests and
//! you must have permission to start new containers.
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

pub mod api;
pub mod auth;
pub mod client;
pub mod error;
pub mod kv2;
pub mod login;
pub mod pki;
pub mod ssh;
pub mod sys;
pub mod token;
