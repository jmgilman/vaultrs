use super::responses::GenerateCredentialsResponse;
use rustify_derive::Endpoint;
use std::fmt::Debug;

/// ## Generate Credentials
/// This endpoint generates a new set of dynamic credentials based on the named
/// role.
///
/// * Path: {self.mount}/creds/{self.name}
/// * Method: GET
/// * Response: [GenerateCredentialsResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/rabbitmq#generate-credentials>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/creds/{self.name}",
    response = "GenerateCredentialsResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct GenerateCredentialsRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}
