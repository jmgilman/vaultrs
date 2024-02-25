use crate::api;
use crate::api::consul::requests::GenerateConsulCredsRequest;
use crate::api::consul::responses::GenerateConsulCredsResponse;
use crate::client::Client;
use crate::error::ClientError;

/// Generates Consul credentials for the given role
///
/// See [GenerateConsulCredsRequest]
#[instrument(skip(client), err)]
pub async fn generate(
    client: &impl Client,
    mount: &str,
    name: &str,
) -> Result<GenerateConsulCredsResponse, ClientError> {
    let endpoint = GenerateConsulCredsRequest::builder()
        .mount(mount)
        .name(name)
        .build()
        .unwrap();
    api::exec_with_result(client, endpoint).await
}

pub mod config {
    use crate::api;
    use crate::api::consul::requests::{SetAccessConfigRequest, SetAccessConfigRequestBuilder};
    use crate::client::Client;
    use crate::error::ClientError;

    /// Creates or updates Consul access config
    ///
    /// See [SetAccessConfigRequest]
    #[instrument(skip(client, opts), err)]
    pub async fn set(
        client: &impl Client,
        mount: &str,
        opts: Option<&mut SetAccessConfigRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = SetAccessConfigRequest::builder();
        let endpoint = opts.unwrap_or(&mut t).mount(mount).build().unwrap();
        api::exec_with_empty(client, endpoint).await
    }
}

pub mod role {
    use crate::api;
    use crate::api::consul::{
        requests::{
            DeleteRoleRequest, ListRolesRequest, ReadRoleRequest, SetRoleRequest,
            SetRoleRequestBuilder,
        },
        responses::{ListRolesResponse, ReadRoleResponse},
    };
    use crate::client::Client;
    use crate::error::ClientError;

    /// Deletes a role
    ///
    /// See [DeleteRoleRequest]
    #[instrument(skip(client), err)]
    pub async fn delete(client: &impl Client, mount: &str, name: &str) -> Result<(), ClientError> {
        let endpoint = DeleteRoleRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Lists all roles
    ///
    /// See [ListRolesRequest]
    #[instrument(skip(client), err)]
    pub async fn list(client: &impl Client, mount: &str) -> Result<ListRolesResponse, ClientError> {
        let endpoint = ListRolesRequest::builder().mount(mount).build().unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Reads a role
    ///
    /// See [ReadRoleRequest]
    #[instrument(skip(client), err)]
    pub async fn read(
        client: &impl Client,
        mount: &str,
        name: &str,
    ) -> Result<ReadRoleResponse, ClientError> {
        let endpoint = ReadRoleRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Creates or updates a role
    ///
    /// See [SetRoleRequest]
    #[instrument(skip(client, opts), err)]
    pub async fn set(
        client: &impl Client,
        mount: &str,
        name: &str,
        opts: Option<&mut SetRoleRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = SetRoleRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut t)
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }
}
