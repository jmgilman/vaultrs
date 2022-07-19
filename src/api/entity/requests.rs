use super::responses::CreateEntityResponse;
use rustify_derive::Endpoint;
use std::fmt::Debug;

#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "identity/entity",
    response = "CreateEntityResponse",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into), default)]
pub struct CreateEntityRequest {
    pub name: String,
    pub policies: String,
}
