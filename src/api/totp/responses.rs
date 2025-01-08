/// Response from executing
/// [CreateKeyRequest][crate::api::totp::requests::CreateKeyRequest]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateKeyResponse {
    pub barcode: Option<String>,
    pub url: Option<String>,
}

/// Response from executing
/// [ReadKeyRequest][crate::api::totp::requests::ReadKeyRequest]
#[derive(Debug, Serialize, Deserialize)]
pub struct ReadKeyResponse {
    pub account_name: String,
    pub algorithm: TotpAlg,
    pub digits: TotpDigits,
    pub issuer: String,
    pub period: u8,
}

/// Response from executing
/// [ListKeysRequest][crate::api::totp::requests::ListKeysRequest]
#[derive(Debug, Serialize, Deserialize)]
pub struct ListKeysResponse {
    pub keys: Vec<String>,
}

/// Response from executing
/// [GenerateCodeRequest][crate::api::totp::requests::GenerateCodeRequest]
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateCodeResponse {
    pub code: u32,
}

/// Response from executing
/// [ValidateCodeRequest][crate::api::totp::requests::ValidateCodeRequest]
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateCodeResponse {
    pub valid: bool,
}
