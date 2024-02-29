use dockertest_server::servers::hashi::VaultServer;
use tracing::log::debug;
use vaultrs::api::identity::requests::{
    CreateEntityByNameRequestBuilder, CreateEntityRequestBuilder,
    UpdateEntityAliasByIdRequestBuilder, UpdateEntityByIdRequestBuilder,
};
use vaultrs::client::VaultClient;
use vaultrs::error::ClientError;
use vaultrs::{identity, sys};

use crate::common::VaultServerHelper;

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

        let entity_id = test_create_entity(&client).await.unwrap();

        test_list_by_id(&client, &entity_id).await;
        test_list_by_name(&client).await;
        let alias_id = test_create_entity_alias(&client, &entity_id).await.unwrap();
        test_read_entity_alias_id(&client, &alias_id).await;
        test_update_entity_alias_by_id(&client, &alias_id).await;
        test_list_entity_alias_by_id(&client, &alias_id, &entity_id).await;
        test_delete_entity_alias_by_id(&client, &alias_id).await;
        let res = test_read_entity_by_name(&client, &entity_id).await;
        assert!(res.is_ok());

        let res = test_read_entity_by_id(&client, &entity_id).await;
        assert!(res.is_ok());

        let res = test_update_entity_by_id(&client, &entity_id).await;

        assert!(res.is_ok());
        test_create_or_update_by_name(&client).await;
        test_delete_by_name(&client).await;
        test_batch_delete(&client).await;
        test_merge(&client).await;
    });
}

async fn test_create_entity(client: &VaultClient) -> Result<String, ClientError> {
    identity::entity::create(
        client,
        "test-entity",
        Some(&mut CreateEntityRequestBuilder::default().policies(vec![POLICY.to_string()])),
    )
    .await
    .unwrap();
    let entity = identity::entity::read_by_name(client, "test-entity")
        .await
        .unwrap();

    assert!(!entity.disabled);

    identity::entity::create(
        client,
        "test-entity",
        Some(
            &mut CreateEntityRequestBuilder::default()
                .disabled(true)
                .id(&entity.id),
        ),
    )
    .await
    .unwrap();
    let entity = identity::entity::read_by_name(client, "test-entity")
        .await
        .unwrap();
    assert!(entity.disabled);
    Ok(entity.id)
}

async fn test_create_entity_alias(
    client: &VaultClient,
    entity_id: &str,
) -> Result<String, ClientError> {
    let auth_response = sys::auth::list(client).await;
    assert!(auth_response.is_ok());
    let auth_response = auth_response?;
    debug!("Auth response {:?}", auth_response);

    let token_auth_response = auth_response.get("token/").unwrap();
    let token_auth_accessor = &token_auth_response.accessor;
    debug!("Token auth accessor: {:?}", token_auth_accessor);

    let create_entity_alias_response = identity::entity_alias::create(
        client,
        ENTITY_ALIAS_NAME,
        entity_id.to_string().as_str(),
        token_auth_accessor,
        None,
    )
    .await;
    debug!(
        "Create entity-alias response: {:?}",
        create_entity_alias_response
    );
    assert!(create_entity_alias_response.is_ok());

    let create_entity_alias_response_data = create_entity_alias_response?.data;
    assert_eq!(
        create_entity_alias_response_data.canonical_id,
        entity_id.to_string().as_str()
    );
    Ok(create_entity_alias_response_data.id)
}

async fn test_read_entity_by_id(
    client: &VaultClient,
    expected_id: &str,
) -> Result<(), ClientError> {
    let read_entity_by_id_response = identity::entity::read_by_id(client, expected_id)
        .await
        .unwrap();

    assert_eq!(read_entity_by_id_response.name, ENTITY_NAME);
    assert_eq!(read_entity_by_id_response.id, expected_id.to_string());
    Ok(())
}

async fn test_list_by_id(client: &VaultClient, expected_id: &str) {
    let entitites = identity::entity::list_by_id(client).await.unwrap();
    assert_eq!(entitites.keys.len(), 1);
    assert_eq!(entitites.keys[0], expected_id);
}

async fn test_list_by_name(client: &VaultClient) {
    let entitites = identity::entity::list_by_name(client).await.unwrap();
    assert_eq!(entitites.keys.len(), 1);
    assert_eq!(entitites.keys[0], ENTITY_NAME);
}

async fn test_update_entity_by_id(
    client: &VaultClient,
    expected_id: &str,
) -> Result<(), ClientError> {
    const NEW_NAME: &str = "new-name";
    identity::entity::update_by_id(
        client,
        expected_id,
        Some(&mut UpdateEntityByIdRequestBuilder::default().name(NEW_NAME)),
    )
    .await
    .unwrap();

    let read_entity_by_id_response = identity::entity::read_by_id(client, expected_id)
        .await
        .unwrap();

    assert_eq!(read_entity_by_id_response.name, NEW_NAME);
    Ok(())
}

async fn test_read_entity_by_name(
    client: &VaultClient,
    expected_id: &str,
) -> Result<(), ClientError> {
    let read_entity_by_name_response = identity::entity::read_by_name(client, ENTITY_NAME)
        .await
        .unwrap();

    assert_eq!(read_entity_by_name_response.name, ENTITY_NAME);
    assert_eq!(read_entity_by_name_response.id, expected_id.to_string());
    Ok(())
}

async fn test_create_or_update_by_name(client: &VaultClient) {
    identity::entity::create_or_update_by_name(client, "test-foo", None)
        .await
        .unwrap();
    let entity = identity::entity::read_by_name(client, "test-foo")
        .await
        .unwrap();
    assert!(!entity.disabled);

    // Here the update part work but require to ignore the result
    identity::entity::create_or_update_by_name(
        client,
        "test-entity",
        Some(&mut CreateEntityByNameRequestBuilder::default().disabled(true)),
    )
    .await
    .unwrap();
    let entity = identity::entity::read_by_name(client, "test-foo")
        .await
        .unwrap();
    assert!(!entity.disabled);
}
async fn test_delete_by_name(client: &VaultClient) {
    identity::entity::create_or_update_by_name(client, "test-bar", None)
        .await
        .unwrap();
    identity::entity::delete_by_name(client, "test-bar")
        .await
        .unwrap();

    assert!(matches!(
        identity::entity::read_by_name(client, "test-bar")
            .await
            .err()
            .unwrap(),
        ClientError::APIError { code: 404, .. }
    ));
}

async fn test_batch_delete(client: &VaultClient) {
    identity::entity::create(client, "test-entity1", None)
        .await
        .unwrap();
    identity::entity::create(client, "test-entity2", None)
        .await
        .unwrap();
    let entity1 = identity::entity::read_by_name(client, "test-entity1")
        .await
        .unwrap();
    let entity2 = identity::entity::read_by_name(client, "test-entity2")
        .await
        .unwrap();

    identity::entity::batch_delete(client, &[entity1.id.to_string(), entity2.id.to_string()])
        .await
        .unwrap();

    assert!(matches!(
        identity::entity::read_by_name(client, "test-entity1")
            .await
            .err()
            .unwrap(),
        ClientError::APIError { code: 404, .. }
    ));
    assert!(matches!(
        identity::entity::read_by_name(client, "test-entity2")
            .await
            .err()
            .unwrap(),
        ClientError::APIError { code: 404, .. }
    ));
}

async fn test_merge(client: &VaultClient) {
    identity::entity::create(client, "test-entity1", None)
        .await
        .unwrap();
    identity::entity::create(client, "test-entity2", None)
        .await
        .unwrap();
    identity::entity::create(client, "test-entity3", None)
        .await
        .unwrap();
    let entity1 = identity::entity::read_by_name(client, "test-entity1")
        .await
        .unwrap();
    let entity2 = identity::entity::read_by_name(client, "test-entity2")
        .await
        .unwrap();
    let entity3 = identity::entity::read_by_name(client, "test-entity3")
        .await
        .unwrap();
    identity::entity::merge(
        client,
        vec![entity1.id.to_string(), entity2.id],
        entity3.id,
        None,
    )
    .await
    .unwrap();
}

async fn test_read_entity_alias_id(client: &VaultClient, alias_id: &str) {
    let entity_alias = identity::entity_alias::read_by_id(client, alias_id)
        .await
        .unwrap();
    assert_eq!(entity_alias.name, ENTITY_ALIAS_NAME);
}

async fn test_update_entity_alias_by_id(client: &VaultClient, alias_id: &str) {
    const NEW_NAME: &str = "new-name";
    identity::entity_alias::update_by_id(
        client,
        alias_id,
        Some(&mut UpdateEntityAliasByIdRequestBuilder::default().name(NEW_NAME)),
    )
    .await
    .unwrap();

    let read_entity_alias_by_id_response = identity::entity_alias::read_by_id(client, alias_id)
        .await
        .unwrap();

    assert_eq!(read_entity_alias_by_id_response.name, NEW_NAME);
}

async fn test_delete_entity_alias_by_id(client: &VaultClient, alias_id: &str) {
    identity::entity_alias::delete_by_id(client, alias_id)
        .await
        .unwrap();

    assert!(matches!(
        identity::entity_alias::read_by_id(client, alias_id)
            .await
            .err()
            .unwrap(),
        ClientError::APIError { code: 404, .. }
    ));
}

async fn test_list_entity_alias_by_id(client: &VaultClient, alias_id: &str, expected_id: &str) {
    let aliases = identity::entity_alias::list_by_id(client).await.unwrap();
    assert_eq!(aliases.keys.len(), 1);
    assert_eq!(aliases.keys[0], alias_id);
    assert_eq!(aliases.key_info[alias_id].canonical_id, expected_id)
}
