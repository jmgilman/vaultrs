use super::KeyType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Response from executing
/// [ReadKeyRequest][crate::api::transit::requests::ReadKeyRequest]
#[derive(Debug, Serialize, Deserialize)]
pub struct ReadKeyResponse {
    #[serde(rename = "type")]
    pub key_type: KeyType,
    pub deletion_allowed: bool,
    pub derived: bool,
    pub exportable: bool,
    pub allow_plaintext_backup: bool,
    /// If the key is asymmetric, the API returns the public keys
    pub keys: ReadKeyData,
    pub min_decryption_version: u64,
    pub min_encryption_version: u64,
    pub name: String,
    pub supports_encryption: bool,
    pub supports_decryption: bool,
    pub supports_derivation: bool,
    pub supports_signing: bool,
    pub imported: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadKeyData {
    /// A key ID integer (string) to unix timestamp.
    Symmetric(HashMap<String, u64>),
    /// A key ID integer (string) to public key mapping.
    Asymmetric(HashMap<String, ReadPublicKeyEntry>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReadPublicKeyEntry {
    /// An ISO8601 timestamp
    pub creation_time: String,
    pub name: String,
    pub public_key: String,
}

/// Response from executing
/// [ListKeysRequest][crate::api::transit::requests::ListKeysRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListKeysResponse {
    pub keys: Vec<String>,
}

/// Response from executing
/// [ExportKeyRequest][crate::api::transit::requests::ExportKeyRequest]
#[derive(Debug, Serialize, Deserialize)]
pub struct ExportKeyResponse {
    pub name: String,
    pub keys: HashMap<String, String>,
}

/// Response from executing
/// [EncryptDataRequest][crate::api::transit::requests::EncryptDataRequest]
#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptDataResponse {
    pub ciphertext: String,
}

/// Response from executing
/// [DecryptDataRequest][crate::api::transit::requests::DecryptDataRequest]
#[derive(Debug, Serialize, Deserialize)]
pub struct DecryptDataResponse {
    pub plaintext: String,
}

/// Response from executing
/// [RewrapDataRequest][crate::api::transit::requests::RewrapDataRequest]
#[derive(Debug, Serialize, Deserialize)]
pub struct RewrapDataResponse {
    pub ciphertext: String,
}

/// Response from executing
/// [GenerateDataKeyRequest][crate::api::transit::requests::GenerateDataKeyRequest]
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateDataKeyResponse {
    pub plaintext: Option<String>,
    pub ciphertext: String,
}

/// Response from executing
/// [GenerateRandomBytesRequest][crate::api::transit::requests::GenerateRandomBytesRequest]
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateRandomBytesResponse {
    pub random_bytes: String,
}

/// Response from executing
/// [HashDataRequest][crate::api::transit::requests::HashDataRequest]
#[derive(Debug, Serialize, Deserialize)]
pub struct HashDataResponse {
    pub sum: String,
}

/// Response from executing
/// [GenerateHmacRequest][crate::api::transit::requests::GenerateHmacRequest]
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateHmacResponse {
    pub hmac: String,
}

/// Response from executing
/// [SignDataRequest][crate::api::transit::requests::SignDataRequest]
#[derive(Debug, Serialize, Deserialize)]
pub struct SignDataResponse {
    pub signature: String,
}

/// Response from executing
/// [VerifySignedDataRequest][crate::api::transit::requests::VerifySignedDataRequest]
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifySignedDataResponse {
    pub valid: bool,
}

/// Response from executing
/// [BackupKeyRequest][crate::api::transit::requests::BackupKeyRequest]
#[derive(Debug, Serialize, Deserialize)]
pub struct BackupKeyResponse {
    pub backup: String,
}

/// Response from executing
/// [ReadTransitCacheConfigurationRequest][crate::api::transit::requests::ReadTransitCacheConfigurationRequest]
#[derive(Debug, Serialize, Deserialize)]
pub struct ReadTransitCacheConfigurationResponse {
    pub size: u64,
}
