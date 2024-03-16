use super::responses::{GetSecretResponse, ListSecretResponse};

use rustify_derive::Endpoint;
use std::fmt::Debug;

/// ## Read Secret
/// This endpoint retrieves the secret at the specified location.
///
/// * Path: {self.mount}/{self.path}
/// * Method: GET
/// * Response: GetSecretResponse
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/kv/kv-v1#read-secret>
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
    pub path: String,
}

/// ## Set Secret
/// This endpoint set or update a secret at the specified location.
///
/// * Path: {self.mount}/{self.path}
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/kv/kv-v1#create-update-secret>
#[derive(Builder, Debug, Endpoint)]
#[endpoint(path = "{self.mount}/{self.path}", method = "POST", builder = "true")]
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
    pub data: Vec<u8>,
}

/// ## List secret keys
/// This endpoint list secrets at given location
///
/// * Path: {self.mount}/{self.path}
/// * Method: LIST
/// * Response: ListSecretResponse
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/kv/kv-v1#list-secrets>
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "{self.mount}/{self.path}",
    method = "LIST",
    builder = "true",
    response = "ListSecretResponse"
)]
#[builder(setter(into))]
pub struct ListSecretRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub path: String,
}

/// ## Delete secret
/// This endpoint delete a secret at given location
///
/// * Path: {self.mount}/{self.path}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/kv/kv-v1#delete-secret>
#[derive(Builder, Debug, Endpoint)]
#[endpoint(path = "{self.mount}/{self.path}", method = "DELETE", builder = "true")]
#[builder(setter(into))]
pub struct DeleteSecretRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub path: String,
}
