use crate::{
    api::{
        self,
        sys::{
            requests::{ReadHealthRequest, SealRequest},
            responses::ReadHealthResponse,
        },
    },
    client::VaultClient,
    error::ClientError,
};

/// Represents the status of a Vault server.
#[derive(Debug)]
pub enum ServerStatus {
    OK,
    PERFSTANDBY,
    RECOVERY,
    SEALED,
    STANDBY,
    UNINITIALIZED,
    UNKNOWN,
}

/// Returns health information about the Vault server.
///
/// See [ReadHealthRequest]
pub async fn health(client: &VaultClient) -> Result<ReadHealthResponse, ClientError> {
    let endpoint = ReadHealthRequest::builder().build().unwrap();
    api::exec_with_no_result(client, endpoint).await
}

/// Seals the Vault server.
///
/// See [SealRequest]
pub async fn seal(client: &VaultClient) -> Result<(), ClientError> {
    let endpoint = SealRequest::builder().build().unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// Returns the status of the Vault server.
///
/// See [ReadHealthRequest]
pub async fn status(client: &VaultClient) -> ServerStatus {
    let result = health(client).await;
    match result {
        Ok(_) => ServerStatus::OK,
        Err(ref e) => match e {
            ClientError::RestClientError { source } => match source {
                rustify::errors::ClientError::ServerResponseError {
                    code: 429,
                    content: _,
                } => ServerStatus::STANDBY,
                rustify::errors::ClientError::ServerResponseError {
                    code: 472,
                    content: _,
                } => ServerStatus::RECOVERY,
                rustify::errors::ClientError::ServerResponseError {
                    code: 473,
                    content: _,
                } => ServerStatus::PERFSTANDBY,
                rustify::errors::ClientError::ServerResponseError {
                    code: 501,
                    content: _,
                } => ServerStatus::UNINITIALIZED,
                rustify::errors::ClientError::ServerResponseError {
                    code: 503,
                    content: _,
                } => ServerStatus::SEALED,
                _ => ServerStatus::UNKNOWN,
            },
            _ => ServerStatus::UNKNOWN,
        },
    }
}

pub mod auth {
    use std::collections::HashMap;

    use crate::api;
    use crate::api::sys::requests::{
        EnableAuthRequest, EnableAuthRequestBuilder, ListAuthsRequest,
    };
    use crate::api::sys::responses::AuthResponse;
    use crate::client::VaultClient;
    use crate::error::ClientError;

    /// Enables an auth engine at the given path
    ///
    /// See [EnableAuthRequest]
    pub async fn enable(
        client: &VaultClient,
        path: &str,
        engine_type: &str,
        opts: Option<&mut EnableAuthRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = EnableAuthRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut t)
            .path(path)
            .engine_type(engine_type)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Lists all mounted auth engines
    ///
    /// See [ListAuthsRequest]
    pub async fn list(client: &VaultClient) -> Result<HashMap<String, AuthResponse>, ClientError> {
        let endpoint = ListAuthsRequest::builder().build().unwrap();
        api::exec_with_result(client, endpoint).await
    }
}

pub mod mount {
    use std::collections::HashMap;

    use crate::api;
    use crate::api::sys::requests::{
        EnableEngineRequest, EnableEngineRequestBuilder, ListMountsRequest,
    };
    use crate::api::sys::responses::MountResponse;
    use crate::client::VaultClient;
    use crate::error::ClientError;

    /// Enables a secret engine at the given path
    ///
    /// See [EnableEngineRequest]
    pub async fn enable(
        client: &VaultClient,
        path: &str,
        engine_type: &str,
        opts: Option<&mut EnableEngineRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = EnableEngineRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut t)
            .path(path)
            .engine_type(engine_type)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Lists all mounted secret engines
    ///
    /// See [ListMountsRequest]
    pub async fn list(client: &VaultClient) -> Result<HashMap<String, MountResponse>, ClientError> {
        let endpoint = ListMountsRequest::builder().build().unwrap();
        api::exec_with_result(client, endpoint).await
    }
}

pub mod wrapping {
    use serde::de::DeserializeOwned;

    use crate::{
        api::{
            self,
            sys::{
                requests::{UnwrapRequest, WrappingLookupRequest},
                responses::WrappingLookupResponse,
            },
        },
        client::VaultClient,
        error::ClientError,
    };

    /// Looks up information about a token wrapping response
    ///
    /// See [WrappingLookupResponse]
    pub async fn lookup(
        client: &VaultClient,
        token: &str,
    ) -> Result<WrappingLookupResponse, ClientError> {
        let endpoint = WrappingLookupRequest::builder()
            .token(token)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Unwraps a token wrapped response
    ///
    /// See [UnwrapRequest]
    pub async fn unwrap<D: DeserializeOwned>(
        client: &VaultClient,
        token: Option<&str>,
    ) -> Result<D, ClientError> {
        let endpoint = UnwrapRequest {
            token: token.map(|v| v.to_string()),
        };
        let res = api::exec_with_result(client, endpoint).await?;
        serde_json::value::from_value(res).map_err(|e| ClientError::JsonParseError { source: e })
    }
}
