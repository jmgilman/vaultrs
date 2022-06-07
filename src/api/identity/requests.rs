use super::responses::CreateIdentityResponse;
use rustify_derive::Endpoint;
use std::fmt::Debug;

#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "identity/entity",
    response = "CreateIdentityResponse",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into), default)]
pub struct CreateIdentityRequest {
    pub name: String,
    pub policies: String,
}
