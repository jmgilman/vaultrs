use super::responses::{
    BackupKeyResponse, DecryptDataResponse, EncryptDataResponse, ExportKeyResponse,
    GenerateDataKeyResponse, GenerateHmacResponse, GenerateRandomBytesResponse, HashDataResponse,
    ListKeysResponse, ReadKeyResponse, ReadTransitCacheConfigurationResponse, RewrapDataResponse,
    SignDataResponse, VerifySignedDataResponse,
};
use super::{HashAlgorithm, KeyType, MarshalingAlgorithm, OutputFormat, SignatureAlgorithm};
use rustify_derive::Endpoint;
use serde::Serialize;
use std::fmt::Debug;

/// ## Create Key
/// This endpoint creates a new named encryption key of the specified type. The
/// values set here cannot be changed after key creation.
///
/// * Path: {self.mount}/keys/{self.name}
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/transit#create-key>
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "{self.mount}/keys/{self.name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct CreateKeyRequest {
    #[endpoint(skip)]
    pub mount: String,
    /// Specifies the name of the encryption key to create.
    #[endpoint(skip)]
    pub name: String,
    /// If enabled, the key will support convergent encryption, where the same
    /// plaintext creates the same ciphertext. This requires derived to be set
    /// to true. When enabled, each encryption(/decryption/rewrap/datakey)
    /// operation will derive a nonce value rather than randomly generate it.
    pub convergent_encryption: Option<bool>,
    /// Specifies if key derivation is to be used. If enabled, all
    /// encrypt/decrypt requests to this named key must provide a context which
    /// is used for key derivation.
    pub derived: Option<bool>,
    /// Enables keys to be exportable. This allows for all the valid keys in the
    /// key ring to be exported. Once set, this cannot be disabled.
    pub exportable: Option<bool>,
    /// If set, enables taking backup of named key in the plaintext format. Once
    /// set, this cannot be disabled.
    pub allow_plaintext_backup: Option<bool>,
    /// Specifies the type of key to create.
    #[serde(rename = "type")]
    pub key_type: Option<KeyType>,
    /// The period at which this key should be rotated automatically. Setting
    /// this to "0" (the default) will disable automatic key rotation. This
    /// value cannot be shorter than one hour.
    pub auto_rotate_period: Option<String>,
}

/// ## Read Key
/// This endpoint returns information about a named encryption key. The keys
/// object shows the creation time of each key version; the values are not the
/// keys themselves. Depending on the type of key, different information may be
/// returned, e.g. an asymmetric key will return its public key in a standard
/// format for the type.
///
/// * Path: {self.mount}/keys/{self.name}
/// * Method: GET
/// * Response: ReadKeyResponse
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/transit#read-key>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/keys/{self.name}",
    response = "ReadKeyResponse",
    builder = "true"
)]
#[builder(setter(into), default)]
pub struct ReadKeyRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}

/// ## List Keys
/// This endpoint returns a list of keys. Only the key names are returned (not
/// the actual keys themselves).
///
/// * Path: {self.mount}/keys
/// * Method: LIST
/// * Response: ListKeysResponse
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/transit#list-keys>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/keys",
    response = "ListKeysResponse",
    method = "LIST",
    builder = "true"
)]
#[builder(setter(into), default)]
pub struct ListKeysRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Update Key Configuration
/// This endpoint allows tuning configuration values for a given key. (These
/// values are returned during a read operation on the named key.)
///
/// * Path: {self.mount}/keys/{self.name}/config
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/transit#update-key-configuration>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/keys/{self.name}/config",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into), default)]
pub struct UpdateKeyConfigurationRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
    /// Specifies the minimum version of ciphertext allowed to be decrypted.
    /// Adjusting this as part of a key rotation policy can prevent old copies
    /// of ciphertext from being decrypted, should they fall into the wrong
    /// hands. For signatures, this value controls the minimum version of
    /// signature that can be verified against. For HMACs, this controls the
    /// minimum version of a key allowed to be used as the key for verification.
    pub min_decryption_version: Option<u64>,
    /// Specifies the minimum version of the key that can be used to encrypt
    /// plaintext, sign payloads, or generate HMACs. Must be 0 (which will use
    /// the latest version) or a value greater or equal to
    /// min_decryption_version.
    pub min_encryption_version: Option<u64>,
    /// Specifies if the key is allowed to be deleted.
    pub deletion_allowed: Option<bool>,
    /// Enables keys to be exportable. This allows for all the valid keys in the
    /// key ring to be exported. Once set, this cannot be disabled.
    pub exportable: Option<bool>,
    /// If set, enables taking backup of named key in the plaintext format. Once
    /// set, this cannot be disabled.
    pub allow_plaintext_backup: Option<bool>,
    /// The period at which this key should be rotated automatically. Setting
    /// this to "0" will disable automatic key rotation. This value cannot be
    /// shorter than one hour. When no value is provided, the period remains
    /// unchanged.
    pub auto_rotate_period: Option<String>,
}

/// ## Delete Key
/// This endpoint deletes a named encryption key. It will no longer be possible
/// to decrypt any data encrypted with the named key. Because this is a
/// potentially catastrophic operation, the deletion_allowed tunable must be set
/// in the key's `/config` endpoint.
///
/// * Path: {self.mount}/keys/{self.name}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/transit#delete-key>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/keys/{self.name}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into), default)]
pub struct DeleteKeyRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}

/// ## Rotate Key
/// This endpoint rotates the version of the named key. After rotation, new
/// plaintext requests will be encrypted with the new version of the key. To
/// upgrade ciphertext to be encrypted with the latest version of the key, use
/// the rewrap endpoint. This is only supported with keys that support
/// encryption and decryption operations.
///
/// * Path: {self.mount}/keys/{self.name}/rotate
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/transit#rotate-key>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/keys/{self.name}/rotate",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into), default)]
pub struct RotateKeyRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}

/// ## Export Key
/// This endpoint returns the named key. The keys object shows the value of the
/// key for each version. If version is specified, the specific version will be
/// returned. If latest is provided as the version, the current key will be
/// provided. Depending on the type of key, different information may be
/// returned. The key must be exportable to support this operation and the
/// version must still be valid.
///
/// * Path: {self.mount}/export/{self.key_type}/{self.name}(/{self.version})
/// * Method: GET
/// * Response: ExportKeyResponse
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/transit#export-key>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/export/{self.key_type}/{self.name}{self.version}",
    response = "ExportKeyResponse",
    builder = "true"
)]
#[builder(setter(into), default)]
pub struct ExportKeyRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub key_type: ExportKeyType,
    #[endpoint(skip)]
    pub name: String,
    #[endpoint(skip)]
    pub version: ExportVersion,
}

#[derive(Clone, Copy, Debug)]
pub enum ExportKeyType {
    EncryptionKey,
    SigningKey,
    HmacKey,
}

#[allow(clippy::derivable_impls)]
impl Default for ExportKeyType {
    fn default() -> Self {
        ExportKeyType::EncryptionKey
    }
}

impl std::fmt::Display for ExportKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EncryptionKey => write!(f, "encryption-key"),
            Self::SigningKey => write!(f, "signing-key"),
            Self::HmacKey => write!(f, "hmac-key"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ExportVersion {
    All,
    Latest,
    Version(u64),
}

#[allow(clippy::derivable_impls)]
impl Default for ExportVersion {
    fn default() -> Self {
        ExportVersion::Latest
    }
}

impl std::fmt::Display for ExportVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => Ok(()),
            Self::Latest => write!(f, "/latest"),
            Self::Version(n) => write!(f, "/{}", n),
        }
    }
}

/// ## Encrypt Data
/// This endpoint encrypts the provided plaintext using the named key. This path
/// supports the create and update policy capabilities as follows: if the user
/// has the create capability for this endpoint in their policies, and the key
/// does not exist, it will be upserted with default values (whether the key
/// requires derivation depends on whether the context parameter is empty or
/// not). If the user only has update capability and the key does not exist, an
/// error will be returned.
///
/// * Path: {self.mount}/encrypt/{self.name}
/// * Method: POST
/// * Response: EncryptDataResponse
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/transit#encrypt-data>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/encrypt/{self.name}",
    method = "POST",
    response = "EncryptDataResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct EncryptDataRequest {
    #[endpoint(skip)]
    pub mount: String,
    /// Specifies the name of the encryption key to encrypt against.
    #[endpoint(skip)]
    pub name: String,
    /// Specifies base64 encoded plaintext to be encoded.
    /// NOTE: All plaintext data must be base64-encoded. The reason for this
    /// requirement is that Vault does not require that the plaintext is "text".
    /// It could be a binary file such as a PDF or image. The easiest safe
    /// transport mechanism for this data as part of a JSON payload is to
    /// base64-encode it.
    pub plaintext: String,
    /// Specifies the base64 encoded context for key derivation. This is
    /// required if key derivation is enabled for this key.
    pub context: Option<String>,
    /// Specifies the version of the key to use for encryption. If not set, uses
    /// the latest version. Must be greater than or equal to the key's
    /// min_encryption_version, if set.
    pub key_version: Option<u64>,
    /// Specifies the base64 encoded nonce value. This must be provided if
    /// convergent encryption is enabled for this key and the key was generated
    /// with Vault 0.6.1. Not required for keys created in 0.6.2+. The value
    /// must be exactly 96 bits (12 bytes) long and the user must ensure that
    /// for any given context (and thus, any given encryption key) this nonce
    /// value is never reused.
    pub nonce: Option<String>,
    /// This parameter is required when encryption key is expected to be
    /// created. When performing an upsert operation, the type of key to create.
    pub key_type: Option<KeyType>,
    /// This parameter will only be used when a key is expected to be created.
    /// Whether to support convergent encryption. This is only supported when
    /// using a key with key derivation enabled and will require all requests to
    /// carry both a context and 96-bit (12-byte) nonce. The given nonce will be
    /// used in place of a randomly generated nonce. As a result, when the same
    /// context and nonce are supplied, the same ciphertext is generated. It is
    /// very important when using this mode that you ensure that all nonces are
    /// unique for a given context. Failing to do so will severely impact the
    /// ciphertext's security.
    pub convergent_encryption: Option<String>,
}

/// ## Decrypt Data
/// This endpoint decrypts the provided ciphertext using the named key.
///
/// * Path: {self.mount}/decrypt/{self.name}
/// * Method: POST
/// * Response: DecryptDataResponse
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/transit#decrypt-data>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/decrypt/{self.name}",
    method = "POST",
    response = "DecryptDataResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DecryptDataRequest {
    #[endpoint(skip)]
    pub mount: String,
    /// Specifies the name of the encryption key to decrypt against.
    #[endpoint(skip)]
    pub name: String,
    /// Specifies the ciphertext to decrypt.
    pub ciphertext: String,
    /// Specifies the base64 encoded context for key derivation. This is
    /// required if key derivation is enabled.
    pub context: Option<String>,
    /// Specifies a base64 encoded nonce value used during encryption. Must be
    /// provided if convergent encryption is enabled for this key and the key
    /// was generated with Vault 0.6.1. Not required for keys created in 0.6.2+.
    pub nonce: Option<String>,
}

/// ## Rewrap Data
/// This endpoint rewraps the provided ciphertext using the latest version of
/// the named key. Because this never returns plaintext, it is possible to
/// delegate this functionality to untrusted users or scripts.
///
/// * Path: {self.mount}/rewrap/{self.name}
/// * Method: POST
/// * Response: RewrapDataResponse
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/transit#rewrap-data>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/rewrap/{self.name}",
    method = "POST",
    response = "RewrapDataResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct RewrapDataRequest {
    #[endpoint(skip)]
    pub mount: String,
    /// Specifies the name of the encryption key to re-encrypt against.
    #[endpoint(skip)]
    pub name: String,
    /// Specifies the ciphertext to re-encrypt.
    pub ciphertext: String,
    /// Specifies the base64 encoded context for key derivation. This is
    /// required if key derivation is enabled.
    pub context: Option<String>,
    /// Specifies the version of the key to use for the operation. If not set,
    /// uses the latest version. Must be greater than or equal to the key's
    /// min_encryption_version, if set.
    pub key_version: Option<u64>,
    /// Specifies a base64 encoded nonce value used during encryption. Must be
    /// provided if convergent encryption is enabled for this key and the key
    /// was generated with Vault 0.6.1. Not required for keys created in 0.6.2+.
    pub nonce: Option<String>,
}

/// ## Generate Data Key
/// This endpoint generates a new high-entropy key and the value encrypted with
/// the named key. Optionally return the plaintext of the key as well. Whether
/// plaintext is returned depends on the path; as a result, you can use Vault
/// ACL policies to control whether a user is allowed to retrieve the plaintext
/// value of a key. This is useful if you want an untrusted user or operation to
/// generate keys that are then made available to trusted users.
///
/// * Path: {self.mount}/datakey/{self.key_type}/{self.name}
/// * Method: POST
/// * Response: GenerateDataKeyResponse
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/transit#generate-data-key>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/datakey/{self.key_type}/{self.name}",
    method = "POST",
    response = "GenerateDataKeyResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct GenerateDataKeyRequest {
    #[endpoint(skip)]
    pub mount: String,
    /// Specifies the type of key to generate. If plaintext, the plaintext key
    /// will be returned along with the ciphertext. If wrapped, only the
    /// ciphertext value will be returned.
    #[endpoint(skip)]
    pub key_type: DataKeyType,
    /// Specifies the name of the encryption key to use to encrypt the datakey.
    #[endpoint(skip)]
    pub name: String,
    /// Specifies the key derivation context, provided as a base64-encoded
    /// string. This must be provided if derivation is enabled.
    pub context: Option<String>,
    /// Specifies a nonce value, provided as base64 encoded. Must be provided if
    /// convergent encryption is enabled for this key and the key was generated
    /// with Vault 0.6.1. Not required for keys created in 0.6.2+. The value
    /// must be exactly 96 bits (12 bytes) long and the user must ensure that
    /// for any given context (and thus, any given encryption key) this nonce
    /// value is never reused.
    pub nonce: Option<String>,
    /// Specifies the number of bits in the desired key. Can be 128, 256, or
    /// 512. Default is 256 bits.
    pub bits: Option<u16>,
}

#[derive(Clone, Copy, Debug)]
pub enum DataKeyType {
    Plaintext,
    Wrapped,
}

#[allow(clippy::derivable_impls)]
impl Default for DataKeyType {
    fn default() -> Self {
        DataKeyType::Wrapped
    }
}

impl std::fmt::Display for DataKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plaintext => write!(f, "plaintext"),
            Self::Wrapped => write!(f, "wrapped"),
        }
    }
}

/// ## Generate Random Bytes
/// This endpoint returns high-quality random bytes of the specified length.
///
/// * Path: {self.mount}/random(/{self.source})(/{self.bytes})
/// * Method: POST
/// * Response: GenerateRandomBytesResponse
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/transit#generate-random-bytes>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/random",
    method = "POST",
    response = "GenerateRandomBytesResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct GenerateRandomBytesRequest {
    #[endpoint(skip)]
    pub mount: String,
    /// Specifies the number of bytes to return. Default is 32.
    pub bytes: Option<u32>,
    /// Specifies the output encoding.
    pub format: OutputFormat,
    /// Specifies the source of the requested bytes.
    pub source: RandomBytesSource,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RandomBytesSource {
    /// Sources bytes from the platform's entropy source.
    Platform,
    /// Sources from entropy augmentation (enterprise only).
    Seal,
    /// Mixes bytes from all available sources.
    All,
}

#[allow(clippy::derivable_impls)]
impl Default for RandomBytesSource {
    fn default() -> Self {
        RandomBytesSource::Platform
    }
}

/// ## Hash Data
/// This endpoint returns the cryptographic hash of given data using the
/// specified algorithm.
///
/// * Path: {self.mount}/hash(/{self.algorithm)
/// * Method: POST
/// * Response: HashDataResponse
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/transit#hash-data>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/hash",
    method = "POST",
    response = "HashDataResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct HashDataRequest {
    #[endpoint(skip)]
    pub mount: String,
    /// Specifies the hash algorithm to use.
    pub algorithm: Option<HashAlgorithm>,
    /// Specifies the base64 encoded input data.
    pub input: String,
    /// Specifies the output encoding.
    pub format: Option<OutputFormat>,
}

/// ## Generate HMAC
/// This endpoint returns the digest of given data using the specified hash
/// algorithm and the named key. The key can be of any type supported by
/// transit; the raw key will be marshaled into bytes to be used for the HMAC
/// function. If the key is of a type that supports rotation, the latest
/// (current) version will be used.
///
/// * Path: {self.mount}/hmac/{self.name}(/{self.algorithm)
/// * Method: POST
/// * Response: GenerateHmacResponse
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/transit#generate-hmac>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/hmac/{self.name}",
    method = "POST",
    response = "GenerateHmacResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct GenerateHmacRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
    /// Specifies the version of the key to use for the operation. If not set,
    /// uses the latest version. Must be greater than or equal to the key's
    /// min_encryption_version, if set.
    pub key_version: Option<u64>,
    /// Specifies the hash algorithm to use.
    pub algorithm: Option<HashAlgorithm>,
    /// Specifies the base64 encoded input data.
    pub input: String,
}

/// ## Sign Data
/// This endpoint returns the cryptographic signature of the given data using
/// the named key and the specified hash algorithm. The key must be of a type
/// that supports signing.
///
/// * Path: {self.mount}/sign/{self.name}(/{self.hash_algorithm)
/// * Method: POST
/// * Response: SignDataResponse
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/transit#sign-data>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/sign/{self.name}",
    method = "POST",
    response = "SignDataResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct SignDataRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
    /// Specifies the version of the key to use for the operation. If not set,
    /// uses the latest version. Must be greater than or equal to the key's
    /// min_encryption_version, if set.
    pub key_version: Option<u64>,
    /// Specifies the hash algorithm to use.
    pub hash_algorithm: Option<HashAlgorithm>,
    /// Specifies the base64 encoded input data.
    pub input: String,
    /// Base64 encoded context for key derivation. Required if key derivation is
    /// enabled; currently only available with ed25519 keys.
    pub context: Option<String>,
    /// Set to true when the input is already hashed. If the key type is
    /// rsa-2048, rsa-3072 or rsa-4096, then the algorithm used to hash the
    /// input should be indicated by the hash_algorithm parameter. Just as the
    /// value to sign should be the base64-encoded representation of the exact
    /// binary data you want signed, when set, input is expected to be
    /// base64-encoded binary hashed data, not hex-formatted.
    pub prehashed: Option<bool>,
    /// When using a RSA key, specifies the RSA signature algorithm to use for
    /// signing.
    pub signature_algorithm: Option<SignatureAlgorithm>,
    /// Specifies the way in which the signature should be marshaled. This
    /// currently only applies to ECDSA keys.
    pub marshaling_algorithm: Option<MarshalingAlgorithm>,
}

/// ## Verify Signed Data
/// This endpoint returns whether the provided signature is valid for the given
/// data.
///
/// * Path: {self.mount}/verify/{self.name}(/{self.hash_algorithm)
/// * Method: POST
/// * Response: VerifySignedDataResponse
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/transit#verify-signed-data>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/verify/{self.name}",
    method = "POST",
    response = "VerifySignedDataResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct VerifySignedDataRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
    /// Specifies the hash algorithm to use.
    pub hash_algorithm: Option<HashAlgorithm>,
    /// Specifies the base64 encoded input data.
    pub input: String,
    /// Specifies the signature output from the /transit/sign function. Either
    /// this must be supplied or hmac must be supplied.
    pub signature: Option<String>,
    /// Specifies the signature output from the /transit/hmac function. Either
    /// this must be supplied or signature must be supplied.
    pub hmac: Option<String>,
    /// Base64 encoded context for key derivation. Required if key derivation is
    /// enabled; currently only available with ed25519 keys.
    pub context: Option<String>,
    /// Set to true when the input is already hashed. If the key type is
    /// rsa-2048, rsa-3072 or rsa-4096, then the algorithm used to hash the
    /// input should be indicated by the hash_algorithm parameter.
    pub prehashed: Option<bool>,
    /// When using a RSA key, specifies the RSA signature algorithm to use for
    /// signature verification.
    pub signature_algorithm: Option<SignatureAlgorithm>,
    /// Specifies the way in which the signature was originally marshaled. This
    /// currently only applies to ECDSA keys.
    pub marshaling_algorithm: Option<MarshalingAlgorithm>,
}

/// ## Backup Key
/// This endpoint returns a plaintext backup of a named key. The backup contains
/// all the configuration data and keys of all the versions along with the HMAC
/// key. The response from this endpoint can be used with the /restore endpoint
/// to restore the key.
///
/// * Path: {self.mount}/backup/{self.name}
/// * Method: GET
/// * Response: BackupKeyResponse
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/transit#backup-key>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/backup/{self.name}",
    response = "BackupKeyResponse",
    builder = "true"
)]
#[builder(setter(into), default)]
pub struct BackupKeyRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}

/// ## Restore Key
/// This endpoint restores the backup as a named key. This will restore the key
/// configurations and all the versions of the named key along with HMAC keys.
/// The input to this endpoint should be the output of /backup endpoint.
///
/// * Path: {self.mount}/restore(/{self.name})
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/transit#restore-key>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "{self.mount}/restore", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct RestoreKeyRequest {
    #[endpoint(skip)]
    pub mount: String,
    /// Backed up key data to be restored. This should be the output from the
    /// /backup endpoint.
    pub backup: String,
    /// If set, this will be the name of the restored key.
    pub name: Option<String>,
    /// If set, force the restore to proceed even if a key by this name already
    /// exists.
    pub force: Option<bool>,
}

/// ## Trim Key
/// This endpoint trims older key versions setting a minimum version for the
/// keyring. Once trimmed, previous versions of the key cannot be recovered.
///
/// * Path: {self.mount}/keys/{self.name}/trim
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/transit#trim-key>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/keys/{self.name}/trim",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into), default)]
pub struct TrimKeyRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
    /// The minimum available version for the key ring. All versions before this
    /// version will be permanently deleted. This value can at most be equal to
    /// the lesser of min_decryption_version and min_encryption_version. This is
    /// not allowed to be set when either min_encryption_version or
    /// min_decryption_version is set to zero.
    pub min_available_version: u64,
}

/// ## Configure Cache
/// This endpoint is used to configure the transit engine's cache. Note that
/// configuration changes will not be applied until the transit plugin is
/// reloaded which can be achieved using the /sys/plugins/reload/backend
/// endpoint.
///
/// * Path: {self.mount}/transit/cache-config
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/transit#configure-cache>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "{self.mount}/cache-config", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct ConfigureCacheRequest {
    #[endpoint(skip)]
    pub mount: String,
    /// Specifies the size in terms of number of entries. A size of 0 means
    /// unlimited. A Least Recently Used (LRU) caching strategy is used for a
    /// non-zero cache size. Must be 0 (default) or a value greater or equal to
    /// 10 (minimum cache size).
    pub size: Option<u64>,
}

/// ## Read Transit Cache Configuration
/// This endpoint retrieves configurations for the transit engine's cache.
///
/// * Path: {self.mount}/transit/cache-config
/// * Method: GET
/// * Response: ReadTransitCacheConfigurationResponse
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/transit#read-transit-cache-configuration>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/cache-config",
    response = "ReadTransitCacheConfigurationResponse",
    builder = "true"
)]
#[builder(setter(into), default)]
pub struct ReadTransitCacheConfigurationRequest {
    #[endpoint(skip)]
    pub mount: String,
}
