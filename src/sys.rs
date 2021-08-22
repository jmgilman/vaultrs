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
    pub fn enable(
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
        api::exec_with_empty(client, endpoint)
    }

    /// Lists all mounted secret engines
    ///
    /// See [ListMountsRequest]
    pub fn list(client: &VaultClient) -> Result<HashMap<String, MountResponse>, ClientError> {
        let endpoint = ListMountsRequest::builder().build().unwrap();
        api::exec_with_result(client, endpoint)
    }
}

pub mod wrapping {
    use std::marker::PhantomData;

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
    pub fn lookup(
        client: &VaultClient,
        token: &str,
    ) -> Result<WrappingLookupResponse, ClientError> {
        let endpoint = WrappingLookupRequest::builder()
            .token(token)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint)
    }

    /// Unwraps a token wrapped response
    ///
    /// See [UnwrapRequest]
    pub fn unwrap<D: DeserializeOwned>(
        client: &VaultClient,
        token: &str,
    ) -> Result<D, ClientError> {
        let endpoint = UnwrapRequest {
            token: token.to_string(),
            _ty: PhantomData,
        };
        api::exec_with_result(client, endpoint)
    }
}
