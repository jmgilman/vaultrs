use crate::api::identity::requests::CreateIdentityRequest;
use crate::api::identity::responses::CreateIdentityResponse;
use crate::{
    api::{self},
    client::Client,
    error::ClientError,
};

#[instrument(skip(client), err)]
pub async fn create_identity(
    client: &impl Client,
    name: &str,
    policies: &str,
) -> Result<CreateIdentityResponse, ClientError> {
    let endpoint = CreateIdentityRequest::builder()
        .name(name)
        .policies(policies)
        .build()
        .unwrap();
    api::exec_with_no_result(client, endpoint).await
}
