use base64::{engine::general_purpose, Engine as _};
use data_encoding::HEXLOWER;
use sha2::{Digest, Sha256};
use tracing::debug;
use vaultrs::{client::VaultClient, error::ClientError, sys::mount};

use crate::common::Test;

#[tokio::test]
async fn test() {
    let test = Test::builder().await;
    let client = test.client();
    let endpoint = TransitEndpoint::setup(client).await.unwrap();

    key::test_create(&endpoint).await;
    key::test_read(&endpoint).await;
    key::test_list(&endpoint).await;
    key::test_rotate(&endpoint).await;
    key::test_update(&endpoint).await;
    key::test_delete(&endpoint).await;
    key::test_export(&endpoint).await;
    key::test_backup_and_restore(&endpoint).await;
    key::test_trim(&endpoint).await;

    data::test_encrypt_and_rewrap_and_decrypt(&endpoint).await;
    data::test_sign_and_verify(&endpoint).await;

    generate::test_data_key(&endpoint).await;
    generate::test_random_bytes(&endpoint).await;
    generate::test_hash(&endpoint).await;
    generate::test_hmac(&endpoint).await;

    cache::test_configure_and_read(&endpoint).await
}

mod key {
    use super::TransitEndpoint;
    use vaultrs::api::transit::requests::{
        CreateKeyRequest, ExportKeyType, ExportVersion, RestoreKeyRequest,
        UpdateKeyConfigurationRequest,
    };
    use vaultrs::api::transit::KeyType;
    use vaultrs::transit::key;

    pub async fn test_create(endpoint: &TransitEndpoint<'_>) {
        key::create(endpoint.client, &endpoint.path, &endpoint.keys.basic, None)
            .await
            .unwrap();

        key::create(
            endpoint.client,
            &endpoint.path,
            &endpoint.keys.export,
            Some(
                CreateKeyRequest::builder()
                    .derived(true)
                    .exportable(true)
                    .allow_plaintext_backup(true)
                    .key_type(KeyType::Aes256Gcm96)
                    .auto_rotate_period("30d"),
            ),
        )
        .await
        .unwrap();

        key::create(endpoint.client, &endpoint.path, &endpoint.keys.delete, None)
            .await
            .unwrap();

        key::create(
            endpoint.client,
            &endpoint.path,
            &endpoint.keys.signing,
            Some(
                CreateKeyRequest::builder()
                    .derived(true)
                    .key_type(KeyType::Ed25519),
            ),
        )
        .await
        .unwrap();

        key::create(
            endpoint.client,
            &endpoint.path,
            &endpoint.keys.asymmetric,
            Some(
                CreateKeyRequest::builder()
                    .exportable(false)
                    .derived(false)
                    .key_type(KeyType::Rsa2048),
            ),
        )
        .await
        .unwrap();
    }

    pub async fn test_read(endpoint: &TransitEndpoint<'_>) {
        let resp = key::read(endpoint.client, &endpoint.path, &endpoint.keys.basic)
            .await
            .unwrap();
        assert_eq!(&resp.name, &endpoint.keys.basic);

        let resp = key::read(endpoint.client, &endpoint.path, &endpoint.keys.export)
            .await
            .unwrap();
        assert!(&resp.exportable);

        let resp = key::read(endpoint.client, &endpoint.path, &endpoint.keys.delete)
            .await
            .unwrap();
        // requires config update first
        assert!(!&resp.deletion_allowed);

        let resp = key::read(endpoint.client, &endpoint.path, &endpoint.keys.asymmetric)
            .await
            .unwrap();
        assert_eq!(&resp.name, &endpoint.keys.asymmetric);
        assert!(matches!(&resp.key_type, KeyType::Rsa2048));
        match &resp.keys {
            vaultrs::api::transit::responses::ReadKeyData::Symmetric(_) => {
                panic!("Key must be asymmetric")
            }
            vaultrs::api::transit::responses::ReadKeyData::Asymmetric(keys) => {
                for key_metadata in keys.values() {
                    let _datetime: chrono::DateTime<chrono::Utc> = key_metadata
                        .creation_time
                        .parse()
                        .expect("Parse ISO8601 timestamp correctly");
                    assert!(key_metadata
                        .public_key
                        .starts_with("-----BEGIN PUBLIC KEY-----\n"));
                    assert!(key_metadata
                        .public_key
                        .ends_with("\n-----END PUBLIC KEY-----\n"));
                }
            }
        }
    }

    pub async fn test_list(endpoint: &TransitEndpoint<'_>) {
        let resp = key::list(endpoint.client, &endpoint.path).await.unwrap();
        assert!(&resp.keys.contains(&endpoint.keys.basic));
        assert!(&resp.keys.contains(&endpoint.keys.export));
    }

    pub async fn test_rotate(endpoint: &TransitEndpoint<'_>) {
        // key version 2
        key::rotate(endpoint.client, &endpoint.path, &endpoint.keys.export)
            .await
            .unwrap();

        // key version 3
        key::rotate(endpoint.client, &endpoint.path, &endpoint.keys.export)
            .await
            .unwrap();
    }

    pub async fn test_update(endpoint: &TransitEndpoint<'_>) {
        key::update(
            endpoint.client,
            &endpoint.path,
            &endpoint.keys.export,
            Some(
                UpdateKeyConfigurationRequest::builder()
                    .min_encryption_version(2u64)
                    .min_decryption_version(2u64),
            ),
        )
        .await
        .unwrap();

        key::update(
            endpoint.client,
            &endpoint.path,
            &endpoint.keys.delete,
            Some(UpdateKeyConfigurationRequest::builder().deletion_allowed(true)),
        )
        .await
        .unwrap();
    }

    pub async fn test_delete(endpoint: &TransitEndpoint<'_>) {
        key::delete(endpoint.client, &endpoint.path, &endpoint.keys.basic)
            .await
            .unwrap_err();

        key::delete(endpoint.client, &endpoint.path, &endpoint.keys.delete)
            .await
            .unwrap();
    }

    pub async fn test_export(endpoint: &TransitEndpoint<'_>) {
        key::export(
            endpoint.client,
            &endpoint.path,
            &endpoint.keys.basic,
            ExportKeyType::EncryptionKey,
            ExportVersion::All,
        )
        .await
        .unwrap_err();

        let latest = key::export(
            endpoint.client,
            &endpoint.path,
            &endpoint.keys.export,
            ExportKeyType::EncryptionKey,
            ExportVersion::Latest,
        )
        .await
        .unwrap();

        let resp = key::export(
            endpoint.client,
            &endpoint.path,
            &endpoint.keys.export,
            ExportKeyType::EncryptionKey,
            ExportVersion::Version(3),
        )
        .await
        .unwrap();
        assert_eq!(&resp.name, &endpoint.keys.export);
        assert_eq!(resp.keys.len(), 1);
        assert_eq!(&resp.keys, &latest.keys);

        let resp = key::export(
            endpoint.client,
            &endpoint.path,
            &endpoint.keys.export,
            ExportKeyType::EncryptionKey,
            ExportVersion::All,
        )
        .await
        .unwrap();
        assert_eq!(resp.keys.len(), 2);
        assert!(resp.keys.contains_key("2"));
        assert!(resp.keys.contains_key("3"));
    }

    pub async fn test_backup_and_restore(endpoint: &TransitEndpoint<'_>) {
        key::backup(endpoint.client, &endpoint.path, &endpoint.keys.basic)
            .await
            .unwrap_err();

        let backup = key::backup(endpoint.client, &endpoint.path, &endpoint.keys.export)
            .await
            .unwrap()
            .backup;

        key::restore(endpoint.client, &endpoint.path, &endpoint.keys.export, None)
            .await
            .unwrap_err();

        key::restore(
            endpoint.client,
            &endpoint.path,
            &backup,
            Some(RestoreKeyRequest::builder().force(true)),
        )
        .await
        .unwrap();
    }

    pub async fn test_trim(endpoint: &TransitEndpoint<'_>) {
        key::trim(endpoint.client, &endpoint.path, &endpoint.keys.export, 2)
            .await
            .unwrap();
    }
}

mod data {
    use super::TransitEndpoint;
    use vaultrs::api::transit::requests::{
        DecryptDataRequest, EncryptDataRequest, RewrapDataRequest, SignDataRequest,
        VerifySignedDataRequest,
    };
    use vaultrs::api::transit::SignatureAlgorithm;
    use vaultrs::transit::{data, key};

    pub async fn test_encrypt_and_rewrap_and_decrypt(endpoint: &TransitEndpoint<'_>) {
        let encrypted = data::encrypt(
            endpoint.client,
            &endpoint.path,
            &endpoint.keys.export,
            &endpoint.data.secret,
            Some(EncryptDataRequest::builder().context(&endpoint.data.context)),
        )
        .await
        .unwrap();

        // key version 4
        key::rotate(endpoint.client, &endpoint.path, &endpoint.keys.export)
            .await
            .unwrap();

        let rewrapped = data::rewrap(
            endpoint.client,
            &endpoint.path,
            &endpoint.keys.export,
            &encrypted.ciphertext,
            Some(RewrapDataRequest::builder().context(&endpoint.data.context)),
        )
        .await
        .unwrap();
        assert!(encrypted.ciphertext != rewrapped.ciphertext);

        let decrypted = data::decrypt(
            endpoint.client,
            &endpoint.path,
            &endpoint.keys.export,
            &encrypted.ciphertext,
            Some(DecryptDataRequest::builder().context(&endpoint.data.context)),
        )
        .await
        .unwrap();
        assert_eq!(&decrypted.plaintext, &endpoint.data.secret);

        let decrypted = data::decrypt(
            endpoint.client,
            &endpoint.path,
            &endpoint.keys.export,
            &rewrapped.ciphertext,
            Some(DecryptDataRequest::builder().context(&endpoint.data.context)),
        )
        .await
        .unwrap();
        assert_eq!(&decrypted.plaintext, &endpoint.data.secret);
    }

    pub async fn test_sign_and_verify(endpoint: &TransitEndpoint<'_>) {
        let signed = data::sign(
            endpoint.client,
            &endpoint.path,
            &endpoint.keys.signing,
            &endpoint.data.secret,
            Some(
                SignDataRequest::builder()
                    .context(&endpoint.data.context)
                    .signature_algorithm(SignatureAlgorithm::Pkcs1v15),
            ),
        )
        .await
        .unwrap();

        let verified = data::verify(
            endpoint.client,
            &endpoint.path,
            &endpoint.keys.signing,
            &endpoint.data.secret,
            Some(
                VerifySignedDataRequest::builder()
                    .context(&endpoint.data.context)
                    .signature(&signed.signature)
                    .signature_algorithm(SignatureAlgorithm::Pkcs1v15),
            ),
        )
        .await
        .unwrap();
        assert!(verified.valid);
    }
}

mod generate {
    use super::TransitEndpoint;
    use vaultrs::api::transit::requests::{
        DataKeyType, GenerateDataKeyRequest, GenerateRandomBytesRequest, HashDataRequest,
        RandomBytesSource,
    };
    use vaultrs::api::transit::{HashAlgorithm, OutputFormat};
    use vaultrs::transit::generate;

    pub async fn test_data_key(endpoint: &TransitEndpoint<'_>) {
        let resp = generate::data_key(
            endpoint.client,
            &endpoint.path,
            &endpoint.keys.basic,
            DataKeyType::Plaintext,
            Some(GenerateDataKeyRequest::builder().bits(512u16)),
        )
        .await
        .unwrap();
        assert!(&resp.plaintext.is_some())
    }

    pub async fn test_random_bytes(endpoint: &TransitEndpoint<'_>) {
        let resp = generate::random_bytes(
            endpoint.client,
            &endpoint.path,
            OutputFormat::Hex,
            RandomBytesSource::Platform,
            Some(GenerateRandomBytesRequest::builder().bytes(10u32)),
        )
        .await
        .unwrap();
        assert_eq!(resp.random_bytes.len(), 20)
    }

    pub async fn test_hash(endpoint: &TransitEndpoint<'_>) {
        let resp = generate::hash(
            endpoint.client,
            &endpoint.path,
            &endpoint.data.context,
            Some(
                HashDataRequest::builder()
                    .algorithm(HashAlgorithm::Sha2_256)
                    .format(OutputFormat::Hex),
            ),
        )
        .await
        .unwrap();
        assert_eq!(resp.sum, endpoint.data.context_shasum_hex);
    }

    pub async fn test_hmac(endpoint: &TransitEndpoint<'_>) {
        generate::hmac(
            endpoint.client,
            &endpoint.path,
            &endpoint.keys.basic,
            &endpoint.data.context,
            None,
        )
        .await
        .unwrap();
    }
}

mod cache {
    use super::TransitEndpoint;
    use vaultrs::api::transit::requests::ConfigureCacheRequest;
    use vaultrs::transit::cache;

    pub async fn test_configure_and_read(endpoint: &TransitEndpoint<'_>) {
        cache::configure(
            endpoint.client,
            &endpoint.path,
            Some(ConfigureCacheRequest::builder().size(123u64)),
        )
        .await
        .unwrap();

        assert_eq!(
            cache::read(endpoint.client, &endpoint.path)
                .await
                .unwrap()
                .size,
            123
        );
    }
}

pub struct TestKeys {
    pub basic: String,
    pub export: String,
    pub delete: String,
    pub signing: String,
    pub asymmetric: String,
}

pub struct TestData {
    pub context: String,
    pub context_shasum_hex: String,
    pub secret: String,
}

impl TestData {
    fn new(context: &str, secret: &str) -> Self {
        let mut context_sha = Sha256::new();
        context_sha.update(context);

        TestData {
            context: general_purpose::STANDARD.encode(context),
            context_shasum_hex: HEXLOWER.encode(&context_sha.finalize()),
            secret: general_purpose::STANDARD.encode(secret),
        }
    }
}

pub struct TransitEndpoint<'a> {
    pub client: &'a VaultClient,
    pub path: String,
    pub keys: TestKeys,
    pub data: TestData,
}

impl<'a> TransitEndpoint<'a> {
    async fn setup(client: &'a VaultClient) -> Result<Self, ClientError> {
        debug!("setting up transit secrets engine");

        let endpoint = TransitEndpoint {
            client,
            path: "transit-test".into(),
            keys: TestKeys {
                basic: "basic-key".into(),
                export: "export-key".into(),
                delete: "delete-key".into(),
                signing: "signing-key".into(),
                asymmetric: "asymmetric-key".into(),
            },
            data: TestData::new("test-context", "super secret data"),
        };

        mount::enable(endpoint.client, &endpoint.path, "transit", None)
            .await
            .unwrap();

        Ok(endpoint)
    }
}
