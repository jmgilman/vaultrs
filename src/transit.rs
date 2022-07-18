pub mod key {
    use crate::api::transit::{
        requests::{
            BackupKeyRequest, CreateKeyRequest, CreateKeyRequestBuilder, DeleteKeyRequest,
            ExportKeyRequest, ExportKeyType, ExportVersion, ListKeysRequest, ReadKeyRequest,
            RestoreKeyRequest, RestoreKeyRequestBuilder, RotateKeyRequest, TrimKeyRequest,
            UpdateKeyConfigurationRequest, UpdateKeyConfigurationRequestBuilder,
        },
        responses::{BackupKeyResponse, ExportKeyResponse, ListKeysResponse, ReadKeyResponse},
    };
    use crate::{api, client::Client, error::ClientError};

    /// Create a new encryption key.
    ///
    /// See [CreateKeyRequest]
    #[instrument(skip(client, opts), err)]
    pub async fn create(
        client: &impl Client,
        mount: &str,
        name: &str,
        opts: Option<&mut CreateKeyRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut builder = CreateKeyRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut builder)
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Read encryption key information.
    ///
    /// See [ReadKeyRequest]
    #[instrument(skip(client), err)]
    pub async fn read(
        client: &impl Client,
        mount: &str,
        name: &str,
    ) -> Result<ReadKeyResponse, ClientError> {
        let endpoint = ReadKeyRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// List key names.
    ///
    /// See [ListKeysRequest]
    #[instrument(skip(client), err)]
    pub async fn list(client: &impl Client, mount: &str) -> Result<ListKeysResponse, ClientError> {
        let endpoint = ListKeysRequest::builder().mount(mount).build().unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Update a key's configuration.
    ///
    /// See [UpdateKeyConfigurationRequest]
    #[instrument(skip(client, opts), err)]
    pub async fn update(
        client: &impl Client,
        mount: &str,
        name: &str,
        opts: Option<&mut UpdateKeyConfigurationRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut builder = UpdateKeyConfigurationRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut builder)
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Delete a named encryption key.
    ///
    /// See [DeleteKeyRequest]
    #[instrument(skip(client), err)]
    pub async fn delete(client: &impl Client, mount: &str, name: &str) -> Result<(), ClientError> {
        let endpoint = DeleteKeyRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Rotate the version of a named key.
    ///
    /// See [RotateKeyRequest]
    #[instrument(skip(client), err)]
    pub async fn rotate(client: &impl Client, mount: &str, name: &str) -> Result<(), ClientError> {
        let endpoint = RotateKeyRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Export a named key.
    ///
    /// See [ExportKeyRequest]
    #[instrument(skip(client), err)]
    pub async fn export(
        client: &impl Client,
        mount: &str,
        name: &str,
        key_type: ExportKeyType,
        version: ExportVersion,
    ) -> Result<ExportKeyResponse, ClientError> {
        let endpoint = ExportKeyRequest::builder()
            .mount(mount)
            .name(name)
            .key_type(key_type)
            .version(version)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Return a plaintext backup of a named key.
    ///
    /// See [BackupKeyRequest]
    #[instrument(skip(client), err)]
    pub async fn backup(
        client: &impl Client,
        mount: &str,
        name: &str,
    ) -> Result<BackupKeyResponse, ClientError> {
        let endpoint = BackupKeyRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Restores the backup of a named key.
    ///
    /// See [RestoreKeyRequest]
    #[instrument(skip(client, opts), err)]
    pub async fn restore(
        client: &impl Client,
        mount: &str,
        backup: &str,
        opts: Option<&mut RestoreKeyRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut builder = RestoreKeyRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut builder)
            .mount(mount)
            .backup(backup)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Trim older key versions setting a minimum version for the keyring.
    ///
    /// See [TrimKeyRequest]
    #[instrument(skip(client), err)]
    pub async fn trim(
        client: &impl Client,
        mount: &str,
        name: &str,
        min_available_version: u64,
    ) -> Result<(), ClientError> {
        let endpoint = TrimKeyRequest::builder()
            .mount(mount)
            .name(name)
            .min_available_version(min_available_version)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }
}

pub mod data {
    use crate::api::transit::{
        requests::{
            DecryptDataRequest, DecryptDataRequestBuilder, EncryptDataRequest,
            EncryptDataRequestBuilder, RewrapDataRequest, RewrapDataRequestBuilder,
            SignDataRequest, SignDataRequestBuilder, VerifySignedDataRequest,
            VerifySignedDataRequestBuilder,
        },
        responses::{
            DecryptDataResponse, EncryptDataResponse, RewrapDataResponse, SignDataResponse,
            VerifySignedDataResponse,
        },
    };
    use crate::{api, client::Client, error::ClientError};

    /// Encrypt base64-encoded plaintext data using the named key.
    ///
    /// See [EncryptDataRequest]
    #[instrument(skip(client, opts), err)]
    pub async fn encrypt(
        client: &impl Client,
        mount: &str,
        name: &str,
        plaintext: &str,
        opts: Option<&mut EncryptDataRequestBuilder>,
    ) -> Result<EncryptDataResponse, ClientError> {
        let mut builder = EncryptDataRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut builder)
            .mount(mount)
            .name(name)
            .plaintext(plaintext)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Decrypt the provided ciphertext using the named key.
    ///
    /// See [DecryptDataRequest]
    #[instrument(skip(client, opts), err)]
    pub async fn decrypt(
        client: &impl Client,
        mount: &str,
        name: &str,
        ciphertext: &str,
        opts: Option<&mut DecryptDataRequestBuilder>,
    ) -> Result<DecryptDataResponse, ClientError> {
        let mut builder = DecryptDataRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut builder)
            .mount(mount)
            .name(name)
            .ciphertext(ciphertext)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Rewrap the provided ciphertext using the latest version of the named
    /// key.
    ///
    /// See [RewrapDataRequest]
    #[instrument(skip(client, opts), err)]
    pub async fn rewrap(
        client: &impl Client,
        mount: &str,
        name: &str,
        ciphertext: &str,
        opts: Option<&mut RewrapDataRequestBuilder>,
    ) -> Result<RewrapDataResponse, ClientError> {
        let mut builder = RewrapDataRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut builder)
            .mount(mount)
            .name(name)
            .ciphertext(ciphertext)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Return the cryptographic signature of the base64-encoded input data.
    ///
    /// See [SignDataRequest]
    #[instrument(skip(client, opts), err)]
    pub async fn sign(
        client: &impl Client,
        mount: &str,
        name: &str,
        input: &str,
        opts: Option<&mut SignDataRequestBuilder>,
    ) -> Result<SignDataResponse, ClientError> {
        let mut builder = SignDataRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut builder)
            .mount(mount)
            .name(name)
            .input(input)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Return whether the provided signature is valid for the base64-encoded
    /// input data.
    ///
    /// See [SignDataRequest]
    #[instrument(skip(client, opts), err)]
    pub async fn verify(
        client: &impl Client,
        mount: &str,
        name: &str,
        input: &str,
        opts: Option<&mut VerifySignedDataRequestBuilder>,
    ) -> Result<VerifySignedDataResponse, ClientError> {
        let mut builder = VerifySignedDataRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut builder)
            .mount(mount)
            .name(name)
            .input(input)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }
}

pub mod generate {
    use crate::api::transit::{
        requests::{
            DataKeyType, GenerateDataKeyRequest, GenerateDataKeyRequestBuilder,
            GenerateHmacRequest, GenerateHmacRequestBuilder, GenerateRandomBytesRequest,
            GenerateRandomBytesRequestBuilder, HashDataRequest, HashDataRequestBuilder,
            RandomBytesSource,
        },
        responses::{
            GenerateDataKeyResponse, GenerateHmacResponse, GenerateRandomBytesResponse,
            HashDataResponse,
        },
        OutputFormat,
    };
    use crate::{api, client::Client, error::ClientError};

    /// Generate a new high-entropy key and the value encrypted with the named
    /// key.
    ///
    /// See [GenerateDataKeyRequest]
    #[instrument(skip(client, opts), err)]
    pub async fn data_key(
        client: &impl Client,
        mount: &str,
        name: &str,
        key_type: DataKeyType,
        opts: Option<&mut GenerateDataKeyRequestBuilder>,
    ) -> Result<GenerateDataKeyResponse, ClientError> {
        let mut builder = GenerateDataKeyRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut builder)
            .mount(mount)
            .name(name)
            .key_type(key_type)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Generate random bytes.
    ///
    /// See [GenerateRandomBytesRequest]
    #[instrument(skip(client, opts), err)]
    pub async fn random_bytes(
        client: &impl Client,
        mount: &str,
        format: OutputFormat,
        source: RandomBytesSource,
        opts: Option<&mut GenerateRandomBytesRequestBuilder>,
    ) -> Result<GenerateRandomBytesResponse, ClientError> {
        let mut builder = GenerateRandomBytesRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut builder)
            .mount(mount)
            .format(format)
            .source(source)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Return the cryptographic hash of the base64-encoded input data.
    ///
    /// See [HashDataRequest]
    #[instrument(skip(client, opts), err)]
    pub async fn hash(
        client: &impl Client,
        mount: &str,
        input: &str,
        opts: Option<&mut HashDataRequestBuilder>,
    ) -> Result<HashDataResponse, ClientError> {
        let mut builder = HashDataRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut builder)
            .mount(mount)
            .input(input)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Return the digest of the base64-encoded input data.
    ///
    /// See [GenerateHmacRequest]
    #[instrument(skip(client, opts), err)]
    pub async fn hmac(
        client: &impl Client,
        mount: &str,
        name: &str,
        input: &str,
        opts: Option<&mut GenerateHmacRequestBuilder>,
    ) -> Result<GenerateHmacResponse, ClientError> {
        let mut builder = GenerateHmacRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut builder)
            .mount(mount)
            .name(name)
            .input(input)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }
}

pub mod cache {
    use crate::api::transit::{
        requests::{
            ConfigureCacheRequest, ConfigureCacheRequestBuilder,
            ReadTransitCacheConfigurationRequest,
        },
        responses::ReadTransitCacheConfigurationResponse,
    };
    use crate::{api, client::Client, error::ClientError};

    /// Read the transit cache configuration.
    ///
    /// See [ReadTransitCacheConfigurationRequest]
    #[instrument(skip(client), err)]
    pub async fn read(
        client: &impl Client,
        mount: &str,
    ) -> Result<ReadTransitCacheConfigurationResponse, ClientError> {
        let endpoint = ReadTransitCacheConfigurationRequest::builder()
            .mount(mount)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Configure the transit engine's cache.
    ///
    /// See [ConfigureCacheRequest]
    #[instrument(skip(client, opts), err)]
    pub async fn configure(
        client: &impl Client,
        mount: &str,
        opts: Option<&mut ConfigureCacheRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut builder = ConfigureCacheRequest::builder();
        let endpoint = opts.unwrap_or(&mut builder).mount(mount).build().unwrap();
        api::exec_with_empty(client, endpoint).await
    }
}
