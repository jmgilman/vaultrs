#![cfg_attr(docsrs, feature(doc_cfg))]

//! # vaultrs
//! An asynchronous Rust client library for the [Hashicorp Vault] API.
//!
//! ## Usages
//!
//! ### AWS
//!
//! The library currently supports all operations available for the
//! AWS Secret Engine.
//!
//! See [aws tests] for more examples.
//!
//! ```no_run
//! use vaultrs::sys::mount;
//! use vaultrs::aws;
//! use vaultrs::api::aws::requests::{SetConfigurationRequest, CreateUpdateRoleRequest, GenerateCredentialsRequest};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # use vaultrs::client::{VaultClientSettingsBuilder, VaultClient};
//! # let client = VaultClient::new(
//! #     VaultClientSettingsBuilder::default()
//! #         .address("https://127.0.0.1:8200")
//! #         .token("TOKEN")
//! #         .build()
//! #         .unwrap()
//! # ).unwrap();
//!
//! // Mount AWS SE
//! mount::enable(&client, "aws_test", "aws", None).await?;
//!
//! // Configure AWS SE
//! aws::config::set(&client, "aws_test", "access_key", "secret_key", Some(SetConfigurationRequest::builder()        
//!     .max_retries(3)
//!     .region("eu-central-1")
//! )).await?;
//!
//! // Create HVault role
//! aws::roles::create_update(&client, "aws_test", "my_role", "assumed_role", Some(CreateUpdateRoleRequest::builder()
//!         .role_arns( vec!["arn:aws:iam::123456789012:role/test_role".to_string()] )
//! )).await?;
//!
//! // Generate credentials
//! let res = aws::roles::credentials(&client, "aws_test", "my_role", Some(GenerateCredentialsRequest::builder()
//!     .ttl("3h")
//! )).await?;
//!
//! let creds = res;
//! // creds.access_key
//! // creds.secret_key
//! // creds.security_token
//! #    Ok(())
//! # }
//! ```
//!
//! ### Key Value v2
//!
//! The library currently supports all operations available for version 2 of the
//! key/value store.
//!
//! ```no_run
//! use serde::{Deserialize, Serialize};
//! use vaultrs::kv2;
//!
//! // Create and read secrets
//! #[derive(Debug, Deserialize, Serialize)]
//! struct MySecret {
//!     key: String,
//!     password: String,
//! }
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # use vaultrs::client::{VaultClientSettingsBuilder, VaultClient};
//! # let client = VaultClient::new(
//! #     VaultClientSettingsBuilder::default()
//! #         .address("https://127.0.0.1:8200")
//! #         .token("TOKEN")
//! #         .build()
//! #         .unwrap()
//! # ).unwrap();
//!
//! let secret = MySecret {
//!     key: "super".to_string(),
//!     password: "secret".to_string(),
//! };
//! kv2::set(
//!     &client,
//!     "secret",
//!     "mysecret",
//!     &secret,
//! ).await;
//!
//! let secret: MySecret = kv2::read(&client, "secret", "mysecret").await.unwrap();
//! println!("{}", secret.password); // "secret"
//! # Ok(())
//! # }
//! ```
//!
//! ### Key Value v1
//!
//! The library currently supports all operations available for version 1 of the
//! key/value store.
//!
//! ```no_run
//! use vaultrs::kv1;
//! use std::collections::HashMap;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # use vaultrs::client::{VaultClientSettingsBuilder, VaultClient};
//! # let client = VaultClient::new(
//! #     VaultClientSettingsBuilder::default()
//! #         .address("https://127.0.0.1:8200")
//! #         .token("TOKEN")
//! #         .build()
//! #         .unwrap()
//! # ).unwrap();
//!
//! let my_secrets = HashMap::from([
//!     ("key1", "value1"),
//!     ("key2", "value2")
//! ]);
//!
//! kv1::set(&client, "secret", "my/secrets", &my_secrets).await.unwrap();
//!
//! let read_secrets: HashMap<String, String> = kv1::get(&client, "secret", "my/secrets").await.unwrap();
//!
//! println!("{:}", read_secrets.get("key1").unwrap()); // value1
//!
//! let list_secret = kv1::list(&client, "secret", "my").await.unwrap();
//!
//! println!("{:?}", list_secret.data.keys); // [ "secrets" ]
//!
//! kv1::delete(&client, "secret", "my/secrets").await.unwrap();
//! # Ok(())
//! # }
//! ```
//!
//! ### PKI
//!
//! The library currently supports all operations available for the PKI secrets
//! engine.
//!
//! ```no_run
//! use vaultrs::api::pki::requests::GenerateCertificateRequest;
//! use vaultrs::pki::cert;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # use vaultrs::client::{VaultClientSettingsBuilder, VaultClient};
//! # let client = VaultClient::new(
//! #     VaultClientSettingsBuilder::default()
//! #         .address("https://127.0.0.1:8200")
//! #         .token("TOKEN")
//! #         .build()
//! #         .unwrap()
//! # ).unwrap();
//!
//! // Generate a certificate using the PKI backend
//! let cert = cert::generate(
//!     &client,
//!     "pki",
//!     "my_role",
//!     Some(GenerateCertificateRequest::builder().common_name("test.com")),
//! ).await?;
//! println!("{}", cert.certificate); // "{PEM encoded certificate}"
//! # Ok(())
//! # }
//! ```
//!
//! ### Transit
//!
//! The library supports most operations for the
//! [Transit](https://developer.hashicorp.com/vault/api-docs/secret/transit) secrets engine,
//! other than importing keys or `batch_input` parameters.
//!
//! ```no_run
//! use vaultrs::api::transit::requests::CreateKeyRequest;
//! use vaultrs::api::transit::KeyType;
//! use vaultrs::transit::key;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # use vaultrs::client::{VaultClientSettingsBuilder, VaultClient};
//! # let client = VaultClient::new(
//! #     VaultClientSettingsBuilder::default()
//! #         .address("https://127.0.0.1:8200")
//! #         .token("TOKEN")
//! #         .build()
//! #         .unwrap()
//! # ).unwrap();
//!
//! // Create an encryption key using the /transit backend
//! key::create(
//!     &client,
//!     "transit",
//!     "my-transit-key",
//!     Some(CreateKeyRequest::builder()
//!        .derived(true)
//!        .key_type(KeyType::Aes256Gcm96)
//!        .auto_rotate_period("30d")),
//! ).await.unwrap();
//! # Ok(())
//! # }
//! ```
//!
//! ### Wrapping
//!
//! All requests implement the ability to be
//! [wrapped](https://developer.hashicorp.com/vault/docs/concepts/response-wrapping). These
//! can be passed in your application internally before being unwrapped.
//!
//! ```no_run
//! use vaultrs::api::ResponseWrapper;
//! use vaultrs::api::sys::requests::ListMountsRequest;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # use vaultrs::client::{VaultClientSettingsBuilder, VaultClient};
//! # let client = VaultClient::new(
//! #     VaultClientSettingsBuilder::default()
//! #         .address("https://127.0.0.1:8200")
//! #         .token("TOKEN")
//! #         .build()
//! #         .unwrap()
//! # ).unwrap();
//!
//!
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
//! # Ok(())
//! # }
//! ```
//!
//!
//! [Hashicorp Vault]: https://developer.hashicorp.com/vault
//! [aws tests]: https://github.com/jmgilman/vaultrs/blob/master/vaultrs-tests/tests/api_tests/aws.rs
//!

#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate tracing;

pub mod api;
pub mod auth;
pub mod aws;
pub mod client;
pub mod cubbyhole;
pub mod database;
pub mod error;
pub mod identity;
pub mod kv1;
pub mod kv2;
pub mod pki;
pub mod rabbitmq;
pub mod ssh;
pub mod sys;
pub mod token;
pub mod transit;
