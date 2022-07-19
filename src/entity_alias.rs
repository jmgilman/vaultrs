use crate::{api, client::Client, error::ClientError};

use crate::api::entity_alias::requests::CreateEntityAliasRequest;
use crate::api::entity_alias::responses::CreateEntityAliasResponse;

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
