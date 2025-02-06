use serde::{Deserialize, Serialize};

/// Response from executing
/// [GeneratedSignedIdTokenRequest](crate::api::identity::oidc::requests::GeneratedSignedIdTokenRequest)
#[derive(Deserialize, Debug, Serialize)]
pub struct GeneratedSignedIdTokenResponse {
    pub client_id: String,
    pub token: String,
    pub ttl: i64,
}
