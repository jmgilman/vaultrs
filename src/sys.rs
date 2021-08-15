pub mod mount {
    use crate::api::sys::requests::{
        EnableEngineRequest, EnableEngineRequestBuilder, ListMountsRequest,
        ListMountsRequestBuilder,
    };

    pub fn enable(path: &str) -> EnableEngineRequestBuilder {
        EnableEngineRequest::builder().path(path).to_owned()
    }

    pub fn list() -> ListMountsRequestBuilder {
        ListMountsRequest::builder().to_owned()
    }
}
