use crate::{
    api::{
        self,
        identity::{
            requests::{
                CreateEntityAliasRequest, CreateEntityRequest, CreateEntityRequestBuilder,
                ReadEntityByNameRequest,
            },
            responses::{
                CreateEntityAliasResponse, CreateEntityResponse, ReadEntityByNameResponse,
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
pub async fn create_entity(
    client: &impl Client,
    name: &str,
    opts: Option<&mut CreateEntityRequestBuilder>,
) -> Result<CreateEntityResponse, ClientError> {
    let mut t = CreateEntityRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).name(name).build().unwrap();
    api::exec_with_no_result(client, endpoint).await
}

/// Reads entity by `name`.
///
/// See [ReadEntityByNameRequest]
#[instrument(skip(client), err)]
pub async fn read_entity_by_name(
    client: &impl Client,
    name: &str,
) -> Result<ReadEntityByNameResponse, ClientError> {
    let endpoint = ReadEntityByNameRequest::builder()
        .name(name)
        .build()
        .unwrap();

    api::exec_with_no_result(client, endpoint).await
}

#[instrument(skip(client), err)]
pub async fn create_entity_alias(
    client: &impl Client,
    name: &str,
    canonical_id: &str,
    mount_accessor: &str,
) -> Result<CreateEntityAliasResponse, ClientError> {
    let endpoint = CreateEntityAliasRequest::builder()
        .name(name)
        .canonical_id(canonical_id)
        .mount_accessor(mount_accessor)
        .build()
        .unwrap();
    api::exec_with_no_result(client, endpoint).await
}
