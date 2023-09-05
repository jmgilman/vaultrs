use crate::api::entity::requests::{CreateEntityRequest, ReadEntityByNameRequest};
use crate::api::entity::responses::{CreateEntityResponse, ReadEntityByNameResponse};
use crate::{
    api::{self},
    client::Client,
    error::ClientError,
};

/// Creates an entity with the given `name` and `policies`.
///
/// See [CreateEntityRequest]
#[instrument(skip(client), err)]
pub async fn create_entity(
    client: &impl Client,
    name: &str,
    policies: &str,
) -> Result<CreateEntityResponse, ClientError> {
    let endpoint = CreateEntityRequest::builder()
        .name(name)
        .policies(policies)
        .build()
        .unwrap();
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
