pub mod entity {
    use std::fmt;

    use crate::{
        api::{
            self,
            identity::{
                requests::{
                    BatchDeleteRequest, CreateEntityByNameRequest,
                    CreateEntityByNameRequestBuilder, CreateEntityRequest,
                    CreateEntityRequestBuilder, DeleteEntityByIdRequest, DeleteEntityByNameRequest,
                    ListEntitiesByIdRequest, ListEntitiesByNameRequest, MergeEntitiesRequest,
                    MergeEntitiesRequestBuilder, ReadEntityByIdRequest, ReadEntityByNameRequest,
                    UpdateEntityByIdRequest, UpdateEntityByIdRequestBuilder,
                },
                responses::{
                    ListEntitiesByIdResponse, ListEntitiesByNameResponse, ReadEntityByIdResponse,
                    ReadEntityByNameResponse,
                },
            },
        },
        client::Client,
        error::ClientError,
    };

    /// Creates an entity with the given `name`.
    ///
    /// See [CreateEntityRequest]
    #[instrument(skip(client, opts), err)]
    pub async fn create(
        client: &impl Client,
        name: &str,
        opts: Option<&mut CreateEntityRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = CreateEntityRequest::builder();
        let endpoint = opts.unwrap_or(&mut t).name(name).build().unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Reads entity by `id`.
    ///
    /// See [ReadEntityByIdRequest]
    #[instrument(skip(client), err)]
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
    #[instrument(skip(client, opts), err)]
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
    #[instrument(skip(client), err)]
    pub async fn delete_by_id(client: &impl Client, id: &str) -> Result<(), ClientError> {
        let endpoint = DeleteEntityByIdRequest::builder().id(id).build().unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Batch delete entity.
    ///
    /// See [BatchDeleteRequest]
    #[instrument(skip(client), err)]
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
    #[instrument(skip(client), err)]
    pub async fn list_by_id(client: &impl Client) -> Result<ListEntitiesByIdResponse, ClientError> {
        let endpoint = ListEntitiesByIdRequest::builder().build().unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Creates or update an entity with the given `name`.
    ///
    /// See [CreateEntityByNameRequest]
    #[instrument(skip(client, opts), err)]
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
    #[instrument(skip(client), err)]
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
    #[instrument(skip(client), err)]
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
    #[instrument(skip(client), err)]
    pub async fn list_by_name(
        client: &impl Client,
    ) -> Result<ListEntitiesByNameResponse, ClientError> {
        let endpoint = ListEntitiesByNameRequest::builder().build().unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Merge entities.
    ///
    /// See [MergeEntitiesRequest]
    #[instrument(skip(client, opts), err)]
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
}

pub mod entity_alias {
    use crate::{
        api::{
            self,
            identity::{
                requests::{
                    CreateEntityAliasRequest, CreateEntityAliasRequestBuilder,
                    DeleteEntityAliasByIdRequest, ListEntityAliasesByIdRequest,
                    ReadEntityAliasByIdRequest, UpdateEntityAliasByIdRequest,
                    UpdateEntityAliasByIdRequestBuilder,
                },
                responses::{
                    CreateEntityAliasResponse, ListEntitiyAliasesByIdResponse,
                    ReadEntityAliasByIdResponse,
                },
            },
        },
        client::Client,
        error::ClientError,
    };

    #[instrument(skip(client, opts), err)]
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
        api::exec_with_no_result(client, endpoint).await
    }

    /// Reads entity alias by `id`.
    ///
    /// See [ReadEntityAliasByIdRequest]
    #[instrument(skip(client), err)]
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

    /// Update entity by `id`.
    ///
    /// See [UpdateEntityByIdRequest]
    #[instrument(skip(client, opts), err)]
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
    #[instrument(skip(client), err)]
    pub async fn delete_by_id(client: &impl Client, id: &str) -> Result<(), ClientError> {
        let endpoint = DeleteEntityAliasByIdRequest::builder()
            .id(id)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// List entities by ID.
    ///
    /// See [ListEntityAliasByIdRequest]
    #[instrument(skip(client), err)]
    pub async fn list_by_id(
        client: &impl Client,
    ) -> Result<ListEntitiyAliasesByIdResponse, ClientError> {
        let endpoint = ListEntityAliasesByIdRequest::builder().build().unwrap();
        api::exec_with_result(client, endpoint).await
    }
}
