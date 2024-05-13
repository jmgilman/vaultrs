pub mod requests;
pub mod responses;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum KeyType {
    /// AES-128 wrapped with GCM using a 96-bit nonce size AEAD (symmetric,
    /// supports derivation and convergent encryption)
    Aes128Gcm96,
    /// AES-256 wrapped with GCM using a 96-bit nonce size AEAD (symmetric,
    /// supports derivation and convergent encryption, default)
    Aes256Gcm96,
    /// ChaCha20-Poly1305 AEAD (symmetric, supports derivation and convergent
    /// encryption)
    Chacha20Poly1305,
    /// ED25519 (asymmetric, supports derivation). When using derivation, a sign
    /// operation with the same context will derive the same key and signature;
    /// this is a signing analogue to convergent_encryption.
    Ed25519,
    /// ECDSA using the P-256 elliptic curve (asymmetric)
    EcdsaP256,
    /// ECDSA using the P-384 elliptic curve (asymmetric)
    EcdsaP384,
    /// ECDSA using the P-521 elliptic curve (asymmetric)
    EcdsaP521,
    /// RSA with bit size of 2048 (asymmetric)
    // kebab-case conversion doesn't work for words starting with a digit.
    #[serde(rename = "rsa-2048")]
    Rsa2048,
    /// RSA with bit size of 3072 (asymmetric)
    #[serde(rename = "rsa-3072")]
    Rsa3072,
    /// RSA with bit size of 4096 (asymmetric)
    #[serde(rename = "rsa-4096")]
    Rsa4096,
}

impl Default for KeyType {
    fn default() -> Self {
        Self::Aes256Gcm96
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OutputFormat {
    Base64,
    Hex,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Base64
    }
}

/// Note: In FIPS 140-2 mode, the following algorithms are not certified and
/// thus should not be used: sha3-224, sha3-256, sha3-384, and sha3-512.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HashAlgorithm {
    Sha2_224,
    Sha2_256,
    Sha2_384,
    Sha2_512,
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SignatureAlgorithm {
    Pss,
    Pkcs1v15,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MarshalingAlgorithm {
    /// The default, used by OpenSSL and X.509
    Asn1,
    /// The version used by JWS (and thus for JWTs). Selecting this will also
    /// change the output encoding to URL-safe Base64 encoding instead of
    /// standard Base64-encoding.
    Jws,
}
