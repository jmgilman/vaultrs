use crate::{
    api::{
        self,
        identity::group_alias::{
            requests::{
                CreateGroupAliasRequest, CreateGroupAliasRequestBuilder,
                DeleteGroupAliasByIdRequest, ListGroupAliasesByIdRequest,
                ReadGroupAliasByIdRequest, UpdateGroupAliasByIdRequest,
                UpdateGroupAliasByIdRequestBuilder,
            },
            responses::{
                CreateGroupAliasResponse, ListGroupAliasesByIdResponse, ReadGroupAliasByIdResponse,
            },
        },
    },
    client::Client,
    error::ClientError,
};

pub async fn create(
    client: &impl Client,
    name: &str,
    mount_accessor: &str,
    opts: Option<&mut CreateGroupAliasRequestBuilder>,
) -> Result<CreateGroupAliasResponse, ClientError> {
    let mut t = CreateGroupAliasRequest::builder();
    let endpoint = opts
        .unwrap_or(&mut t)
        .name(name)
        .mount_accessor(mount_accessor)
        .build()
        .unwrap();
    api::exec_with_result(client, endpoint)
        .await
        .map_err(|err| {
            // In the case the response as an empty HTTP Body
            if matches!(
                err,
                ClientError::RestClientError {
                    source: rustify::errors::ClientError::ResponseParseError { .. }
                }
            ) {
                return ClientError::InvalidUpdateParameter;
            }
            err
        })
}

/// Reads group alias by `id`.
///
/// See [ReadGroupAliasByIdRequest]
pub async fn read_by_id(
    client: &impl Client,
    id: &str,
) -> Result<ReadGroupAliasByIdResponse, ClientError> {
    let endpoint = ReadGroupAliasByIdRequest::builder().id(id).build().unwrap();

    api::exec_with_result(client, endpoint).await
}

/// Update group alias by `id`.
///
/// See [UpdateGroupAliasByIdRequest]
pub async fn update_by_id(
    client: &impl Client,
    id: &str,
    mount_accessor: &str,
    opts: Option<&mut UpdateGroupAliasByIdRequestBuilder>,
) -> Result<(), ClientError> {
    let mut t = UpdateGroupAliasByIdRequest::builder();
    let endpoint = opts
        .unwrap_or(&mut t)
        .id(id)
        .mount_accessor(mount_accessor)
        .build()
        .unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// Delete group alias by `id`.
///
/// See [DeleteGroupAliasByIdRequest]
pub async fn delete_by_id(client: &impl Client, id: &str) -> Result<(), ClientError> {
    let endpoint = DeleteGroupAliasByIdRequest::builder()
        .id(id)
        .build()
        .unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// List groups aliases by ID.
///
/// See [ListGroupAliasesByIdRequest]
pub async fn list_by_id(client: &impl Client) -> Result<ListGroupAliasesByIdResponse, ClientError> {
    let endpoint = ListGroupAliasesByIdRequest::builder().build().unwrap();
    api::exec_with_result(client, endpoint).await
}
