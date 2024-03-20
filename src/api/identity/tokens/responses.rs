use serde::{Deserialize, Serialize};

/// Response from executing
/// [GenerateSignedIdTokenRequest][crate::api::identity::requests::GenerateSignedIdTokenRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct GenerateSignedIdTokenResponse {
    pub client_id: String,
    pub token: String,
    pub ttl: u64,
}
