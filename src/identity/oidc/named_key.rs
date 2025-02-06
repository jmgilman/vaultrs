use crate::api;
use crate::api::identity::oidc::named_key::requests::{CreateNamedKeyRequest, CreateNamedKeyRequestBuilder};
use crate::client::Client;
use crate::error::ClientError;

/// Create a named key.
///
/// See [CreateNamedKeyRequest]
pub async fn set(
    client: &impl Client,
    name: &str,
    opts: Option<&mut CreateNamedKeyRequestBuilder>
) -> Result<(), ClientError> {
    let mut t = CreateNamedKeyRequest::builder();
    let endpoint = opts
        .unwrap_or(&mut t)
        .name(name)
        .build()
        .unwrap();

    api::exec_with_empty(client, endpoint).await
}

/// Delete a named key.
///
/// See [DeleteNamedKeyRequest]
pub async fn delete(
    client: &impl Client,
    name: &str,
) -> Result<(), ClientError> {
    let endpoint = CreateNamedKeyRequest::builder()
        .name(name).build().unwrap();

    api::exec_with_empty(client, endpoint).await
}
