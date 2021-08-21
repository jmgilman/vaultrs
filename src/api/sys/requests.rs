use super::responses::MountResponse;
use crate::api::EndpointResult;
use rustify_derive::Endpoint;
use serde::{de::DeserializeOwned, Serialize};
use serde_with::skip_serializing_none;
use std::{collections::HashMap, marker::PhantomData};

/// ## Enable Secrets Engine
/// This endpoint enables a new secrets engine at the given path.
///
/// * Path: {sys/mounts/{self.path}
/// * Method: POST
/// * Response: N/A
/// * Reference: https://www.vaultproject.io/api-docs/system/mounts#enable-secrets-engine
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(path = "sys/mounts/{self.path}", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct EnableEngineRequest {
    #[serde(skip)]
    pub path: String,
    #[serde(rename = "type")]
    pub engine_type: Option<String>,
    pub description: Option<String>,
    pub config: Option<EnableEngineDataConfig>,
    pub options: Option<HashMap<String, String>>,
}

#[skip_serializing_none]
#[derive(Clone, Builder, Debug, Default, Serialize)]
#[builder(setter(into, strip_option), default)]
pub struct EnableEngineDataConfig {
    pub default_lease_ttl: Option<String>,
    pub max_lease_ttl: Option<String>,
    pub force_no_cache: Option<bool>,
    pub audit_non_hmac_request_keys: Option<Vec<String>>,
    pub audit_non_hmac_response_keys: Option<Vec<String>>,
    pub listing_visibility: Option<String>,
    pub passthrough_request_headers: Option<Vec<String>>,
    pub allowed_response_headers: Option<Vec<String>>,
}

/// ## List Mounted Secrets Engines
/// This endpoints lists all the mounted secrets engines.
///
/// * Path: sys/mounts
/// * Method: GET
/// * Response: [HashMap<String, MountResponse>]
/// * Reference: https://www.vaultproject.io/api-docs/system/mounts#list-mounted-secrets-engines
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "sys/mounts",
    result = "EndpointResult<HashMap<String, MountResponse>>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListMountsRequest {}

/// ## List Mounted Secrets Engines
/// This endpoints lists all the mounted secrets engines.
///
/// * Path: /sys/wrapping/unwrap
/// * Method: POST
/// * Response: T
/// * Reference: https://www.vaultproject.io/api-docs/system/wrapping-unwrap#wrapping-unwrap
#[skip_serializing_none]
#[derive(Builder, Endpoint, Serialize)]
#[endpoint(path = "/sys/wrapping/unwrap", method = "POST", result = "T")]
#[builder(setter(into))]
pub struct UnwrapRequest<T: DeserializeOwned> {
    pub token: String,
    #[serde(skip)]
    pub _ty: PhantomData<T>,
}
