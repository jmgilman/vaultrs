use crate::api;
use crate::api::identity::oidc::role::requests::{DeleteRoleRequest, ReadRoleRequest, SetRoleRequest, SetRoleRequestBuilder};
use crate::api::identity::oidc::role::responses::ReadRoleResponse;
use crate::client::Client;
use crate::error::ClientError;

/// Create or update a role.
///
/// See [SetRoleRequest]
pub async fn set(
    client: &impl Client,
    name: &str,
    key: &str,
    opts: Option<&mut SetRoleRequestBuilder>
) -> Result<(), ClientError> {
    let mut t = SetRoleRequest::builder();
    let endpoint = opts
        .unwrap_or(&mut t)
        .name(name)
        .key(key)
        .build()
        .unwrap();

    api::exec_with_empty(client, endpoint).await
}

/// Read a role.
///
/// See [ReadRoleRequest]
pub  async fn read(
    client: &impl Client,
    name: &str,
) -> Result<ReadRoleResponse, ClientError> {
    let endpoint = ReadRoleRequest::builder()
        .name(name).build().unwrap();

    api::exec_with_result(client, endpoint).await
}

/// Delete a role.
///
/// See [DeleteRoleRequest]
pub async fn delete(
    client: &impl Client,
    name: &str,
) -> Result<(), ClientError> {
    let endpoint = DeleteRoleRequest::builder()
        .name(name).build().unwrap();

    api::exec_with_empty(client, endpoint).await
}
