use std::str::FromStr;

use crate::common::VaultServerHelper;
use dockertest_server::servers::hashi::VaultServer;
use tracing::log::debug;
use uuid::Uuid;
use vaultrs::client::VaultClient;
use vaultrs::error::ClientError;
use vaultrs::{entity, entity_alias, sys};

mod common;

const ENTITY_NAME: &str = "test-entity";
const ENTITY_ALIAS_NAME: &str = "test-entity-alias";
const POLICY: &str = "default";

#[test]
fn test_create_entity_and_alias() {
    let test = common::new_test();

    test.run(|instance| async move {
        let server: VaultServer = instance.server();
        let client = server.client();

        let res = test_create_entity(&client).await;
        assert!(res.is_ok());
        let entity_id = res.unwrap();

        let res = test_create_entity_alias(&client, &entity_id).await;
        assert!(res.is_ok());

        let res = test_read_entity_by_name(&client, &entity_id).await;
        assert!(res.is_ok());
    });
}

async fn test_create_entity(client: &VaultClient) -> Result<Uuid, ClientError> {
    let create_entity_response = entity::create_entity(client, ENTITY_NAME, POLICY).await;
    assert!(create_entity_response.is_ok());
    debug!("Create entity response: {:?}", create_entity_response);

    let create_entity_response_data = create_entity_response?.data;
    assert_eq!(create_entity_response_data.name, ENTITY_NAME);
    Uuid::from_str(create_entity_response_data.id.as_str())
        .map_err(|e| ClientError::UuidParseError { source: e })
}

async fn test_create_entity_alias(
    client: &VaultClient,
    entity_id: &Uuid,
) -> Result<(), ClientError> {
    let auth_response = sys::auth::list(client).await;
    assert!(auth_response.is_ok());
    let auth_response = auth_response?;
    debug!("Auth response {:?}", auth_response);

    let token_auth_response = auth_response
        .get("token/")
        .ok_or(ClientError::InfallibleError)?;
    let token_auth_accessor = &token_auth_response.accessor;
    debug!("Token auth accessor: {:?}", token_auth_accessor);

    let create_entity_alias_response = entity_alias::create_entity_alias(
        client,
        ENTITY_ALIAS_NAME,
        entity_id.to_string().as_str(),
        token_auth_accessor,
    )
    .await;
    assert!(create_entity_alias_response.is_ok());
    debug!(
        "Create entity-alias response: {:?}",
        create_entity_alias_response
    );

    let create_entity_alias_response_data = create_entity_alias_response?.data;
    assert_eq!(
        create_entity_alias_response_data.canonical_id,
        entity_id.to_string().as_str()
    );
    Ok(())
}

async fn test_read_entity_by_name(
    client: &VaultClient,
    expected_id: &Uuid,
) -> Result<(), ClientError> {
    let read_entity_by_name_response = entity::read_entity_by_name(client, ENTITY_NAME).await;
    assert!(read_entity_by_name_response.is_ok());

    let response_data = read_entity_by_name_response?.data;
    assert_eq!(response_data.name, ENTITY_NAME);
    assert_eq!(response_data.id, expected_id.to_string());
    Ok(())
}
