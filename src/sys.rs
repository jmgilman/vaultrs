use crate::{
    api::{
        self,
        sys::{
            requests::{
                ReadHealthRequest, SealRequest, StartInitializationRequest,
                StartInitializationRequestBuilder, UnsealRequest,
            },
            responses::{ReadHealthResponse, StartInitializationResponse, UnsealResponse},
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

/// Initialize a new Vault. The Vault must not have been previously initialized.
///
/// See [StartInitializationRequest]
pub async fn start_initialization(
    client: &impl Client,
    secret_shares: u64,
    secret_threshold: u64,
    opts: Option<&mut StartInitializationRequestBuilder>,
) -> Result<StartInitializationResponse, ClientError> {
    let mut t = StartInitializationRequest::builder();
    let endpoint = opts
        .unwrap_or(&mut t)
        .secret_shares(secret_shares)
        .secret_threshold(secret_threshold)
        .build()
        .unwrap();
    api::exec_with_no_result(client, endpoint).await
}

/// Seals the Vault server.
///
/// See [SealRequest]
pub async fn seal(client: &impl Client) -> Result<(), ClientError> {
    let endpoint = SealRequest::builder().build().unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// Unseals the Vault server.
///
/// See [UnsealRequest]
pub async fn unseal(
    client: &impl Client,
    key: Option<String>,
    reset: Option<bool>,
    migrate: Option<bool>,
) -> Result<UnsealResponse, ClientError> {
    let endpoint = UnsealRequest::builder()
        .key(key)
        .reset(reset)
        .migrate(migrate)
        .build()
        .unwrap();
    api::exec_with_no_result(client, endpoint).await
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
        DisableAuthRequest, EnableAuthRequest, EnableAuthRequestBuilder, ListAuthsRequest,
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

    /// Disables the auth method at the given auth path.
    ///
    /// `sudo` required - This endpoint requires `sudo` capability in
    ///  addition to any path-specific capabilities.
    ///
    /// See [DisableAuthRequest]
    pub async fn disable(client: &impl Client, path: &str) -> Result<(), ClientError> {
        let endpoint = DisableAuthRequest::builder().path(path).build().unwrap();
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
        DisableEngineRequest, EnableEngineRequest, EnableEngineRequestBuilder,
        GetConfigurationOfTheSecretEngineRequest, ListMountsRequest,
    };
    use crate::api::sys::responses::{GetConfigurationOfTheSecretEngineResponse, MountResponse};
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

    /// Disable a secret engine at the given path
    ///
    /// See [DisableEngineRequest]
    #[instrument(skip(client), err)]
    pub async fn disable(client: &impl Client, path: &str) -> Result<(), ClientError> {
        let endpoint = DisableEngineRequest::builder().path(path).build().unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// This endpoint returns the configuration of a specific secret engine.
    ///
    /// See [GetConfigurationOfTheSecretEngineRequest]
    #[instrument(skip(client), err)]
    pub async fn get_configuration_of_a_secret_engine(
        client: &impl Client,
        path: &str,
    ) -> Result<GetConfigurationOfTheSecretEngineResponse, ClientError> {
        let endpoint = GetConfigurationOfTheSecretEngineRequest::builder()
            .path(path)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Lists all mounted secret engines
    ///
    /// See [ListMountsRequest]
    pub async fn list(client: &impl Client) -> Result<HashMap<String, MountResponse>, ClientError> {
        let endpoint = ListMountsRequest::builder().build().unwrap();
        api::exec_with_result(client, endpoint).await
    }
}

pub mod remount {
    use crate::{
        api::{
            self,
            sys::{
                requests::{RemountRequest, RemountStatusRequest},
                responses::{RemountResponse, RemountStatusResponse},
            },
        },
        client::Client,
        error::ClientError,
    };

    /// This endpoint moves an already-mounted backend to a new mount point.
    ///
    /// See [RemountRequest]
    pub async fn remount(
        client: &impl Client,
        from: &str,
        to: &str,
    ) -> Result<RemountResponse, ClientError> {
        let endpoint = RemountRequest::builder().from(from).to(to).build().unwrap();
        dbg!(&endpoint);
        api::exec_with_result(client, endpoint).await
    }

    /// This endpoint is used to monitor the status of a mount migration operation.
    ///
    /// See [RemountStatusRequest]
    #[instrument(skip(client), err)]
    pub async fn remount_status(
        client: &impl Client,
        migration_id: &str,
    ) -> Result<RemountStatusResponse, ClientError> {
        let endpoint = RemountStatusRequest::builder()
            .migration_id(migration_id)
            .build()
            .unwrap();
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

pub mod tools {
    use crate::{
        api::{
            self,
            sys::{
                requests::{RandomRequest, RandomRequestBuilder},
                responses::RandomResponse,
            },
        },
        client::Client,
        error::ClientError,
    };

    /// Returns high-quality random bytes of the specified length.
    ///
    /// See [RandomResponse]
    #[instrument(skip(client, opts), err)]
    pub async fn random(
        client: &impl Client,
        opts: Option<&mut RandomRequestBuilder>,
    ) -> Result<RandomResponse, ClientError> {
        let mut t = RandomRequest::builder();
        let endpoint = opts.unwrap_or(&mut t).build().unwrap();
        api::exec_with_result(client, endpoint).await
    }
}

pub mod leases {
    use crate::{
        api::{
            self,
            sys::{requests::RenewLeaseRequest, responses::RenewLeaseResponse},
        },
        client::Client,
        error::ClientError,
    };

    /// Renews a lease.
    ///
    /// See [RenewLeaseResponse]
    #[instrument(skip(client), err)]
    pub async fn renew(
        client: &impl Client,
        lease_id: &str,
        increment: Option<&str>,
    ) -> Result<RenewLeaseResponse, ClientError> {
        let mut endpoint = RenewLeaseRequest::builder();
        if let Some(inc) = increment {
            endpoint.increment(inc);
        }
        api::exec_with_no_result(client, endpoint.lease_id(lease_id).build().unwrap()).await
    }
}
