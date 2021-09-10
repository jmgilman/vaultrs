use crate::{
    api::{
        self,
        sys::{
            requests::{ReadHealthRequest, SealRequest},
            responses::ReadHealthResponse,
        },
    },
    client::Client,
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
pub async fn health(client: &impl Client) -> Result<ReadHealthResponse, ClientError> {
    let endpoint = ReadHealthRequest::builder().build().unwrap();
    api::exec_with_no_result(client, endpoint).await
}

/// Seals the Vault server.
///
/// See [SealRequest]
pub async fn seal(client: &impl Client) -> Result<(), ClientError> {
    let endpoint = SealRequest::builder().build().unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// Returns the status of the Vault server.
///
/// See [ReadHealthRequest]
pub async fn status(client: &impl Client) -> Result<ServerStatus, ClientError> {
    let result = health(client).await;
    match result {
        Ok(_) => Ok(ServerStatus::OK),
        Err(e) => match e {
            ClientError::RestClientError { source } => match source {
                rustify::errors::ClientError::ServerResponseError {
                    code: 429,
                    content: _,
                } => Ok(ServerStatus::STANDBY),
                rustify::errors::ClientError::ServerResponseError {
                    code: 472,
                    content: _,
                } => Ok(ServerStatus::RECOVERY),
                rustify::errors::ClientError::ServerResponseError {
                    code: 473,
                    content: _,
                } => Ok(ServerStatus::PERFSTANDBY),
                rustify::errors::ClientError::ServerResponseError {
                    code: 501,
                    content: _,
                } => Ok(ServerStatus::UNINITIALIZED),
                rustify::errors::ClientError::ServerResponseError {
                    code: 503,
                    content: _,
                } => Ok(ServerStatus::SEALED),
                _ => Err(ClientError::RestClientError { source }),
            },
            _ => Err(e),
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
    use crate::client::Client;
    use crate::error::ClientError;

    /// Enables an auth engine at the given path
    ///
    /// See [EnableAuthRequest]
    pub async fn enable(
        client: &impl Client,
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
    pub async fn list(client: &impl Client) -> Result<HashMap<String, AuthResponse>, ClientError> {
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
    use crate::client::Client;
    use crate::error::ClientError;

    /// Enables a secret engine at the given path
    ///
    /// See [EnableEngineRequest]
    pub async fn enable(
        client: &impl Client,
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
    pub async fn list(client: &impl Client) -> Result<HashMap<String, MountResponse>, ClientError> {
        let endpoint = ListMountsRequest::builder().build().unwrap();
        api::exec_with_result(client, endpoint).await
    }
}

pub mod policy {
    use crate::{
        api::{
            self,
            sys::{
                requests::{
                    CreatePolicyRequest, DeletePolicyRequest, ListPoliciesRequest,
                    ReadPolicyRequest,
                },
                responses::{ListPoliciesResponse, ReadPolicyResponse},
            },
        },
        client::Client,
        error::ClientError,
    };

    /// Deletes the given policy.
    ///
    /// See [DeletePolicyRequest]
    pub async fn delete(client: &impl Client, name: &str) -> Result<(), ClientError> {
        let endpoint = DeletePolicyRequest::builder().name(name).build().unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Lists all configured policies.
    ///
    /// See [ListPoliciesRequest]
    pub async fn list(client: &impl Client) -> Result<ListPoliciesResponse, ClientError> {
        let endpoint = ListPoliciesRequest::builder().build().unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Reads the given policy.
    ///
    /// See [ReadPolicyRequest]
    pub async fn read(client: &impl Client, name: &str) -> Result<ReadPolicyResponse, ClientError> {
        let endpoint = ReadPolicyRequest::builder().name(name).build().unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Sets the given policy.
    ///
    /// See [CreatePolicyRequest]
    pub async fn set(client: &impl Client, name: &str, policy: &str) -> Result<(), ClientError> {
        let endpoint = CreatePolicyRequest::builder()
            .name(name)
            .policy(policy)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
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
        client::Client,
        error::ClientError,
    };

    /// Looks up information about a token wrapping response
    ///
    /// See [WrappingLookupResponse]
    pub async fn lookup(
        client: &impl Client,
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
        client: &impl Client,
        token: Option<&str>,
    ) -> Result<D, ClientError> {
        let endpoint = UnwrapRequest {
            token: token.map(|v| v.to_string()),
        };
        let res = api::exec_with_result(client, endpoint).await?;
        serde_json::value::from_value(res).map_err(|e| ClientError::JsonParseError { source: e })
    }
}
