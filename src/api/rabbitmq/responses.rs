use serde::{Deserialize, Serialize};
/// Response from executing
#[derive(Deserialize, Debug, Serialize)]
pub struct GenerateCredentialsResponseData {
    pub username: String,
    pub password: String,
}

/// Response from executing
/// [GetSecretRequest][crate::api::rabbitmq::requests::GenerateCredentialsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct GenerateCredentialsResponse {
  pub data: GenerateCredentialsResponseData,

  /// Auth is always null, official doc does not document this field
  pub auth: Option<String>,
  pub lease_duration: i32,
  pub lease_id: String,
  pub renewable: bool,
  pub request_id: String,
}
