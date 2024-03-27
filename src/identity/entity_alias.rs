use crate::{
    api::{
        self,
        identity::entity_alias::{
            requests::{
                CreateEntityAliasRequest, CreateEntityAliasRequestBuilder,
                DeleteEntityAliasByIdRequest, ListEntityAliasesByIdRequest,
                ReadEntityAliasByIdRequest, UpdateEntityAliasByIdRequest,
                UpdateEntityAliasByIdRequestBuilder,
            },
            responses::{
                CreateEntityAliasResponse, ListEntityAliasesByIdResponse,
                ReadEntityAliasByIdResponse,
            },
        },
    },
    client::Client,
    error::ClientError,
};

/// Create or update an entity alias.
///
/// See [ CreateEntityAliasRequest]
pub async fn create(
    client: &impl Client,
    name: &str,
    canonical_id: &str,
    mount_accessor: &str,
    opts: Option<&mut CreateEntityAliasRequestBuilder>,
) -> Result<CreateEntityAliasResponse, ClientError> {
    let mut t = CreateEntityAliasRequest::builder();
    let endpoint = opts
        .unwrap_or(&mut t)
        .name(name)
        .canonical_id(canonical_id)
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

/// Reads entity alias by `id`.
///
/// See [ReadEntityAliasByIdRequest]
pub async fn read_by_id(
    client: &impl Client,
    id: &str,
) -> Result<ReadEntityAliasByIdResponse, ClientError> {
    let endpoint = ReadEntityAliasByIdRequest::builder()
        .id(id)
        .build()
        .unwrap();

    api::exec_with_result(client, endpoint).await
}

/// Update entity_alias by `id`.
///
/// See [UpdateEntityAliasByIdRequest]
pub async fn update_by_id(
    client: &impl Client,
    id: &str,
    opts: Option<&mut UpdateEntityAliasByIdRequestBuilder>,
) -> Result<(), ClientError> {
    let mut t = UpdateEntityAliasByIdRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).id(id).build().unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// Delete entity alias by `id`.
///
/// See [DeleteEntityAliasByIdRequest]
pub async fn delete_by_id(client: &impl Client, id: &str) -> Result<(), ClientError> {
    let endpoint = DeleteEntityAliasByIdRequest::builder()
        .id(id)
        .build()
        .unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// List entity aliases by ID.
///
/// See [ListEntityAliasesByIdRequest]
pub async fn list_by_id(
    client: &impl Client,
) -> Result<ListEntityAliasesByIdResponse, ClientError> {
    let endpoint = ListEntityAliasesByIdRequest::builder().build().unwrap();
    api::exec_with_result(client, endpoint).await
}
