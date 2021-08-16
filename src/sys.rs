pub mod mount {
    use std::collections::HashMap;

    use crate::api;
    use crate::api::sys::requests::{
        EnableEngineRequest, EnableEngineRequestBuilder, ListMountsRequest,
    };
    use crate::api::sys::responses::MountResponse;
    use crate::client::VaultClient;
    use crate::error::ClientError;

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

    pub fn list(client: &VaultClient) -> Result<HashMap<String, MountResponse>, ClientError> {
        let endpoint = ListMountsRequest::builder().build().unwrap();
        api::exec_with_result(client, endpoint)
    }
}
