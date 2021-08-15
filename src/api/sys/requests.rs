use super::responses::MountResponse;
use crate::api::strip;
use rustify_derive::Endpoint;
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::collections::HashMap;

// Enable engine
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

// List mounted engines
#[skip_serializing_none]
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "sys/mounts",
    result = "HashMap<String, MountResponse>",
    transform = "strip::<HashMap<String, MountResponse>>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListMountsRequest {}
