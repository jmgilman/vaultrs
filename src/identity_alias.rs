use crate::{api, client::Client, error::ClientError};

use crate::api::identity_alias::requests::CreateIdentityAliasRequest;
use crate::api::identity_alias::responses::CreateIdentityAliasResponse;

#[instrument(skip(client), err)]
pub async fn create_identity_alias(
    client: &impl Client,
    name: &str,
    canonical_id: &str,
    mount_accessor: &str,
) -> Result<CreateIdentityAliasResponse, ClientError> {
    let endpoint = CreateIdentityAliasRequest::builder()
        .name(name)
        .canonical_id(canonical_id)
        .mount_accessor(mount_accessor)
        .build()
        .unwrap();
    api::exec_with_no_result(client, endpoint).await
}
