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
        token: &str,
    ) -> Result<D, ClientError> {
        let endpoint = UnwrapRequest {
            token: token.to_string(),
        };
        let res = api::exec_with_result(client, endpoint).await?;
        serde_json::value::from_value(res).map_err(|e| ClientError::JsonParseError {
            source: Box::new(e),
        })
    }
}
