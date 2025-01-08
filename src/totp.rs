pub mod key {
    use crate::{
        api::{
            self,
            totp::{
                requests::{
                    CreateKeyRequest, CreateKeyRequestBuilder, DeleteKeyRequest, ListKeysRequest,
                    ReadKeyRequest,
                },
                responses::{ListKeysResponse, ReadKeyResponse},
            },
        },
        client::Client,
        error::ClientError,
    };

    /// Create a new TOTP key.
    ///
    /// See [CreateKeyRequest]
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

    /// Read key information.
    ///
    /// See [ReadKeyRequest]
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
    pub async fn list(client: &impl Client, mount: &str) -> Result<ListKeysResponse, ClientError> {
        let endpoint = ListKeysRequest::builder().mount(mount).build().unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Delete a key.
    ///
    /// See [DeleteKeyRequest]
    pub async fn delete(client: &impl Client, mount: &str, name: &str) -> Result<(), ClientError> {
        let endpoint = DeleteKeyRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }
}

pub mod code {
    use crate::{
        api::totp::{
            requests::{GenerateCodeRequest, ValidateCodeRequest, ValidateCodeRequestBuilder},
            responses::{GenerateCodeResponse, ValidateCodeResponse},
        },
        error::ClientError,
    };

    pub async fn generate(
        client: &impl Client,
        mount: &str,
        name: &str,
    ) -> Result<GenerateCodeResponse, ClientError> {
        let endpoints = GenerateCodeRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    pub async fn validate(
        client: &impl Client,
        mount: &str,
        name: &str,
        code: u32,
        opts: Option<&mut ValidateCodeRequestBuilder>,
    ) -> Result<ValidateCodeResponse, ClientError> {
        let endpoint = ValidateCodeRequest::builder()
            .mount(mount)
            .name(name)
            .code(code)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }
}
