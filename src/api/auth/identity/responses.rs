use serde::{Deserialize, Serialize};
use serde_json::Value;

/// TODO
#[derive(Deserialize, Debug, Serialize)]
pub struct ConfigureIdentityTokensBackendResponse {
    /// TODO
    pub data: Value,
    /// TODO
    pub warnings: Vec<String>,
}

/// TODO
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadConfigurationsIdentityTokensBackendResponse {
    /// TODO
    pub data: Value,
}

/// Response from executing
/// [GenerateSignedIdTokenRequest][crate::api::identity::requests::GenerateSignedIdTokenRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct GenerateSignedIdTokenResponse {
    pub data: TokenData,
}

/// Data payload for [GenerateSignedIdTokenResponse].
#[derive(Deserialize, Debug, Serialize)]
pub struct TokenData {
    pub client_id: String,
    pub token: String,
    pub ttl: u64,
}

/// Response from executing
/// [IntrospectSignedIdTokenRequest][crate::api::identity::requests::IntrospectSignedIdTokenRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct IntrospectSignedIdTokenResponse {
    /// Whether the signed ID token is currently active.
    pub active: bool,
}
