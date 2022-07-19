use crate::api::entity::requests::CreateEntityRequest;
use crate::api::entity::responses::CreateEntityResponse;
use crate::{
    api::{self},
    client::Client,
    error::ClientError,
};

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
