use super::responses::{GetSecretResponse};

use rustify_derive::Endpoint;
use std::fmt::Debug;

/// ## Read Secret
/// This endpoint retrieves the secret at the specified location.
///
/// * Path: {self.mount}/{self.path}
/// * Method: GET
/// * Response: GetSecretResponse
/// * Reference: https://www.vaultproject.io/api-docs/secret/kv/kv-v1#read-secret
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/{self.path}",
    response = "GetSecretResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct GetSecretRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub path: String
}

/// ## Set Secret
/// This endpoint set or update a secret at the specified location.
///
/// * Path: {self.mount}/{self.path}
/// * Method: POST
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/secret/kv/kv-v1#create-update-secret
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/{self.path}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into))]
pub struct SetSecretRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub path: String,

    // key/value pairs to pass Vault
    // Must be as raw, otherwise payload would be sent as
    // { data: { key: value, key2: value2 } } rather than plain { key: value, key2: value2 }
    // Result in a secret with key "data" and erroneous valué
    #[endpoint(raw)]
    pub data: Vec<u8>
}