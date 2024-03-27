use std::fmt;

use crate::{
    api::{
        self,
        identity::entity::{
            requests::{
                BatchDeleteRequest, CreateEntityByNameRequest, CreateEntityByNameRequestBuilder,
                CreateEntityRequest, CreateEntityRequestBuilder, DeleteEntityByIdRequest,
                DeleteEntityByNameRequest, ListEntitiesByIdRequest, ListEntitiesByNameRequest,
                MergeEntitiesRequest, MergeEntitiesRequestBuilder, ReadEntityByIdRequest,
                ReadEntityByNameRequest, UpdateEntityByIdRequest, UpdateEntityByIdRequestBuilder,
            },
            responses::{
                CreateEntityResponse, ListEntitiesByIdResponse, ListEntitiesByNameResponse,
                ReadEntityByIdResponse, ReadEntityByNameResponse,
            },
        },
    },
    client::Client,
    error::ClientError,
};

/// Create an entity.
///
/// See [CreateEntityRequest]
pub async fn create(
    client: &impl Client,
    opts: Option<&mut CreateEntityRequestBuilder>,
) -> Result<CreateEntityResponse, ClientError> {
    let mut t = CreateEntityRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Reads entity by `id`.
///
/// See [ReadEntityByIdRequest]
pub async fn read_by_id(
    client: &impl Client,
    id: &str,
) -> Result<ReadEntityByIdResponse, ClientError> {
    let endpoint = ReadEntityByIdRequest::builder().id(id).build().unwrap();

    api::exec_with_result(client, endpoint).await
}

/// Update entity by `id`.
///
/// See [UpdateEntityByIdRequest]
pub async fn update_by_id(
    client: &impl Client,
    id: &str,
    opts: Option<&mut UpdateEntityByIdRequestBuilder>,
) -> Result<(), ClientError> {
    let mut t = UpdateEntityByIdRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).id(id).build().unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// Delete entity by `id`.
///
/// See [DeleteEntityByIdRequest]
pub async fn delete_by_id(client: &impl Client, id: &str) -> Result<(), ClientError> {
    let endpoint = DeleteEntityByIdRequest::builder().id(id).build().unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// Batch delete entity.
///
/// See [BatchDeleteRequest]
pub async fn batch_delete<T: fmt::Debug + Into<Vec<String>>>(
    client: &impl Client,
    entity_ids: T,
) -> Result<(), ClientError> {
    let endpoint = BatchDeleteRequest::builder()
        .entity_ids(entity_ids)
        .build()
        .unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// List entities by ID.
///
/// See [ListEntitiesByIdRequest]
pub async fn list_by_id(client: &impl Client) -> Result<ListEntitiesByIdResponse, ClientError> {
    let endpoint = ListEntitiesByIdRequest::builder().build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Creates or update an entity with the given `name`.
///
/// See [CreateEntityByNameRequest]
pub async fn create_or_update_by_name(
    client: &impl Client,
    name: &str,
    opts: Option<&mut CreateEntityByNameRequestBuilder>,
) -> Result<(), ClientError> {
    let mut t = CreateEntityByNameRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).name(name).build().unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// Reads entity by `name`.
///
/// See [ReadEntityByNameRequest]
pub async fn read_by_name(
    client: &impl Client,
    name: &str,
) -> Result<ReadEntityByNameResponse, ClientError> {
    let endpoint = ReadEntityByNameRequest::builder()
        .name(name)
        .build()
        .unwrap();

    api::exec_with_result(client, endpoint).await
}

/// Delete entity by `name`.
///
/// See [DeleteEntityByIdRequest]
pub async fn delete_by_name(client: &impl Client, name: &str) -> Result<(), ClientError> {
    let endpoint = DeleteEntityByNameRequest::builder()
        .name(name)
        .build()
        .unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// List entities by Name.
///
/// See [ListEntitiesByNameRequest]
pub async fn list_by_name(client: &impl Client) -> Result<ListEntitiesByNameResponse, ClientError> {
    let endpoint = ListEntitiesByNameRequest::builder().build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Merge entities.
///
/// See [MergeEntitiesRequest]
pub async fn merge(
    client: &impl Client,
    from_entity_ids: Vec<String>,
    to_entity_id: String,
    opts: Option<&mut MergeEntitiesRequestBuilder>,
) -> Result<(), ClientError> {
    let mut t = MergeEntitiesRequest::builder();
    let endpoint = opts
        .unwrap_or(&mut t)
        .from_entity_ids(from_entity_ids)
        .to_entity_id(to_entity_id)
        .build()
        .unwrap();
    api::exec_with_empty(client, endpoint).await
}
