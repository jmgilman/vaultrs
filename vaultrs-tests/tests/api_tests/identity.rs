use std::collections::HashMap;

use vaultrs::{
    api::identity::{
        entity::requests::{
            CreateEntityByNameRequestBuilder, CreateEntityRequestBuilder,
            UpdateEntityByIdRequestBuilder,
        },
        entity_alias::requests::{
            CreateEntityAliasRequestBuilder, UpdateEntityAliasByIdRequestBuilder,
        },
        group::requests::{
            CreateGroupByNameRequestBuilder, CreateGroupRequestBuilder,
            UpdateGroupByIdRequestBuilder,
        },
        group_alias::requests::{
            CreateGroupAliasRequestBuilder, UpdateGroupAliasByIdRequestBuilder,
        },
    },
    client::VaultClient,
    error::ClientError,
    identity, sys,
};

use crate::common::Test;

const ENTITY_NAME: &str = "test-entity";
const ENTITY_NEW_NAME: &str = "new-test-entity";
const ENTITY_ALIAS_NAME: &str = "test-entity-alias";
const POLICY: &str = "default";

const GROUP_NAME: &str = "test-group";
const GROUP_ALIAS_NAME: &str = "test-group-alias";

#[tokio::test]
async fn test_entity_and_entity_alias() {
    let test = Test::builder().await;

    let client = test.client();

    let entity_id = test_create_entity(client).await;
    let alias_id = test_create_entity_alias(client, &entity_id).await;
    create_anonymous_entity(client).await;
    test_list_entity_by_id(client, &entity_id).await;
    test_read_entity_by_id(client, &entity_id).await;
    test_update_entity_by_id(client, &entity_id).await;

    test_list_entity_by_name(client).await;
    test_read_entity_by_name(client, &entity_id).await;
    test_create_or_update_entity_by_name(client).await;
    test_delete_entity_by_name(client).await;

    test_batch_delete_entity(client).await;
    test_merge_entity(client).await;

    test_read_entity_alias_id(client, &alias_id).await;
    test_update_entity_alias_by_id(client, &alias_id).await;
    test_list_entity_alias_by_id(client, &alias_id, &entity_id).await;
    test_delete_entity_alias_by_id(client, &alias_id).await;
}

#[tokio::test]
async fn test_group_and_group_alias() {
    let test = Test::builder().await;
    let client = test.client();

    let group_id = test_create_group(client).await;
    test_read_group_by_id(client, &group_id).await;
    test_update_group_by_id(client, &group_id).await;
    test_list_groups_by_id(client, &group_id).await;
    test_delete_group_by_id(client, &group_id).await;

    test_create_group_by_name(client).await;
    test_read_group_by_name(client).await;
    test_list_groups_by_name(client).await;
    test_delete_group_by_name(client).await;

    let group_alias_id = test_group_alias(client).await;
    test_update_group_alias_by_id(client, &group_alias_id).await;
    test_list_group_aliases_by_id(client, &group_alias_id).await;
    test_delete_group_alias_by_id(client, &group_alias_id).await;
}

async fn test_create_entity(client: &VaultClient) -> String {
    identity::entity::create(
        client,
        Some(
            &mut CreateEntityRequestBuilder::default()
                .policies(vec![POLICY.to_string()])
                .name(ENTITY_NAME),
        ),
    )
    .await
    .unwrap()
    .id
}

async fn create_anonymous_entity(client: &VaultClient) {
    // Without specifying anything Vault will create an entity for us and make sure it got an unique id.
    let entity = identity::entity::create(client, None).await.unwrap();
    assert!(!entity.id.is_empty());
    assert!(entity.alias.is_none());
    identity::entity::delete_by_id(client, &entity.id)
        .await
        .unwrap();
}

async fn test_read_entity_by_id(client: &VaultClient, expected_id: &str) {
    let entity = identity::entity::read_by_id(client, expected_id)
        .await
        .unwrap();

    assert_eq!(entity.name, ENTITY_NAME);
    assert_eq!(entity.id, expected_id.to_string());
}

async fn test_list_entity_by_id(client: &VaultClient, expected_id: &str) {
    let entities = identity::entity::list_by_id(client).await.unwrap();
    assert_eq!(entities.keys, [expected_id]);
}

async fn test_list_entity_by_name(client: &VaultClient) {
    let entities = identity::entity::list_by_name(client).await.unwrap();
    assert_eq!(entities.keys, [ENTITY_NEW_NAME]);
}

async fn test_update_entity_by_id(client: &VaultClient, expected_id: &str) {
    identity::entity::update_by_id(
        client,
        expected_id,
        Some(&mut UpdateEntityByIdRequestBuilder::default().name(ENTITY_NEW_NAME)),
    )
    .await
    .unwrap();

    let entity = identity::entity::read_by_id(client, expected_id)
        .await
        .unwrap();

    assert_eq!(entity.name, ENTITY_NEW_NAME);
}

async fn test_read_entity_by_name(client: &VaultClient, expected_id: &str) {
    let entity = identity::entity::read_by_name(client, ENTITY_NEW_NAME)
        .await
        .unwrap();

    assert_eq!(entity.name, ENTITY_NEW_NAME);
    assert_eq!(entity.id, expected_id.to_string());
}

async fn test_create_or_update_entity_by_name(client: &VaultClient) {
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
async fn test_delete_entity_by_name(client: &VaultClient) {
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

async fn test_batch_delete_entity(client: &VaultClient) {
    identity::entity::create(
        client,
        Some(CreateEntityRequestBuilder::default().name("test-entity1")),
    )
    .await
    .unwrap();
    identity::entity::create(
        client,
        Some(CreateEntityRequestBuilder::default().name("test-entity2")),
    )
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

async fn test_merge_entity(client: &VaultClient) {
    identity::entity::create(
        client,
        Some(CreateEntityRequestBuilder::default().name("test-entity1")),
    )
    .await
    .unwrap();
    identity::entity::create(
        client,
        Some(CreateEntityRequestBuilder::default().name("test-entity2")),
    )
    .await
    .unwrap();
    identity::entity::create(
        client,
        Some(CreateEntityRequestBuilder::default().name("test-entity3")),
    )
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

async fn test_create_entity_alias(client: &VaultClient, entity_id: &str) -> String {
    let auth_response = sys::auth::list(client).await.unwrap();

    let token_auth_response = auth_response.get("token/").unwrap();
    let token_auth_accessor = &token_auth_response.accessor;

    let entity_alias = identity::entity_alias::create(
        client,
        ENTITY_ALIAS_NAME,
        entity_id.to_string().as_str(),
        token_auth_accessor,
        None,
    )
    .await
    .unwrap();

    assert_eq!(
        entity_alias.canonical_id.clone(),
        entity_id.to_string().as_str()
    );
    // We call the same function but this time for updating.
    let metadata = HashMap::from([(String::from("foo"), String::from("bar"))]);
    identity::entity_alias::create(
        client,
        ENTITY_ALIAS_NAME,
        entity_id.to_string().as_str(),
        token_auth_accessor,
        Some(
            &mut CreateEntityAliasRequestBuilder::default()
                .id(entity_alias.id.clone())
                .custom_metadata(metadata.clone()),
        ),
    )
    .await
    .unwrap();
    let entity_alias = identity::entity_alias::read_by_id(client, &entity_alias.id.clone())
        .await
        .unwrap();
    assert_eq!(entity_alias.custom_metadata.unwrap(), metadata);

    // If we use the function for updating but without giving field to be updated
    assert!(matches!(
        identity::entity_alias::create(
            client,
            ENTITY_ALIAS_NAME,
            entity_id.to_string().as_str(),
            token_auth_accessor,
            Some(&mut CreateEntityAliasRequestBuilder::default().id(entity_alias.id.clone())),
        )
        .await
        .err()
        .unwrap(),
        ClientError::InvalidUpdateParameter
    ));

    entity_alias.id
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

    let entity_alias = identity::entity_alias::read_by_id(client, alias_id)
        .await
        .unwrap();

    assert_eq!(entity_alias.name, NEW_NAME);
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
    assert_eq!(aliases.keys, [alias_id]);
    assert_eq!(aliases.key_info[alias_id].canonical_id, expected_id)
}

async fn test_create_group(client: &VaultClient) -> String {
    let group = identity::group::create(
        client,
        Some(
            &mut CreateGroupRequestBuilder::default().policies(vec![POLICY.to_string()]), // .name(GROUP_NAME),
        ),
    )
    .await
    .unwrap();
    identity::group::read_by_id(client, &group.id)
        .await
        .unwrap();
    identity::group::read_by_name(client, &group.name)
        .await
        .unwrap();
    identity::group::delete_by_id(client, &group.id)
        .await
        .unwrap();

    // We create a group without policy to see if we can also parse the response.
    let group = identity::group::create(client, Some(&mut CreateGroupRequestBuilder::default()))
        .await
        .unwrap();
    identity::group::read_by_id(client, &group.id)
        .await
        .unwrap();
    identity::group::read_by_name(client, &group.name)
        .await
        .unwrap();
    identity::group::delete_by_id(client, &group.id)
        .await
        .unwrap();

    identity::group::create_or_update_by_name(
        client,
        GROUP_NAME,
        Some(&mut CreateGroupByNameRequestBuilder::default().policies(vec![POLICY.to_string()])),
    )
    .await
    .unwrap();
    let group = identity::group::read_by_name(client, GROUP_NAME)
        .await
        .unwrap();

    group.id
}

async fn test_read_group_by_id(client: &VaultClient, group_id: &str) {
    let group = identity::group::read_by_id(client, group_id).await.unwrap();
    assert_eq!(group.name, GROUP_NAME);
}

async fn test_update_group_by_id(client: &VaultClient, group_id: &str) {
    const NEW_NAME: &str = "new-name";
    identity::group::update_by_id(
        client,
        group_id,
        Some(&mut UpdateGroupByIdRequestBuilder::default().name(NEW_NAME)),
    )
    .await
    .unwrap();
    let group = identity::group::read_by_id(client, group_id).await.unwrap();
    assert_eq!(group.name, NEW_NAME);
}

async fn test_delete_group_by_id(client: &VaultClient, group_id: &str) {
    identity::group::delete_by_id(client, group_id)
        .await
        .unwrap();

    assert!(matches!(
        identity::group::read_by_id(client, group_id)
            .await
            .err()
            .unwrap(),
        ClientError::APIError { code: 404, .. }
    ));
}

async fn test_list_groups_by_id(client: &VaultClient, group_id: &str) {
    let groups = identity::group::list_by_id(client).await.unwrap();
    assert_eq!(groups.keys.len(), 1);
    assert_eq!(groups.keys[0], group_id);
}

async fn test_create_group_by_name(client: &VaultClient) {
    identity::group::create_or_update_by_name(
        client,
        GROUP_NAME,
        Some(&mut CreateGroupByNameRequestBuilder::default().policies(vec![POLICY.to_string()])),
    )
    .await
    .unwrap();
    let group = identity::group::read_by_name(client, GROUP_NAME)
        .await
        .unwrap();

    assert!(group.metadata.is_none());
    let metadata = HashMap::from([(String::from("company"), String::from("example-company"))]);

    identity::group::create_or_update_by_name(
        client,
        GROUP_NAME,
        Some(&mut CreateGroupByNameRequestBuilder::default().metadata(metadata.clone())),
    )
    .await
    .unwrap();
    let group = identity::group::read_by_name(client, GROUP_NAME)
        .await
        .unwrap();
    assert_eq!(group.metadata, Some(metadata));
}

async fn test_read_group_by_name(client: &VaultClient) {
    let group = identity::group::read_by_name(client, GROUP_NAME)
        .await
        .unwrap();
    assert_eq!(group.name, GROUP_NAME);
}

async fn test_delete_group_by_name(client: &VaultClient) {
    identity::group::delete_by_name(client, GROUP_NAME)
        .await
        .unwrap();

    assert!(matches!(
        identity::group::read_by_name(client, GROUP_NAME)
            .await
            .err()
            .unwrap(),
        ClientError::APIError { code: 404, .. }
    ));
}

async fn test_list_groups_by_name(client: &VaultClient) {
    let groups = identity::group::list_by_name(client).await.unwrap();
    assert_eq!(groups.keys, [GROUP_NAME]);
}

async fn test_group_alias(client: &VaultClient) -> String {
    // We create an external group.
    identity::group::create_or_update_by_name(
        client,
        GROUP_NAME,
        Some(
            &mut CreateGroupByNameRequestBuilder::default()
                .policies(vec![POLICY.to_string()])
                .group_type("external"),
        ),
    )
    .await
    .unwrap();
    let auth_response = sys::auth::list(client).await.unwrap();
    let token_auth_response = auth_response.get("token/").unwrap();
    let token_auth_accessor = &token_auth_response.accessor;

    let group = identity::group::read_by_name(client, GROUP_NAME)
        .await
        .unwrap();
    let group_alias = identity::group_alias::create(
        client,
        GROUP_ALIAS_NAME,
        token_auth_accessor,
        Some(&mut CreateGroupAliasRequestBuilder::default().canonical_id(&group.id)),
    )
    .await
    .unwrap();

    assert_eq!(&group_alias.canonical_id, &group.id);
    group_alias.id
}

async fn test_update_group_alias_by_id(client: &VaultClient, group_alias_id: &str) {
    const NEW_NAME: &str = "new-name";
    let auth_response = sys::auth::list(client).await.unwrap();
    let token_auth_response = auth_response.get("token/").unwrap();
    let token_auth_accessor = &token_auth_response.accessor;
    identity::group_alias::update_by_id(
        client,
        group_alias_id,
        token_auth_accessor,
        Some(&mut UpdateGroupAliasByIdRequestBuilder::default().name(NEW_NAME)),
    )
    .await
    .unwrap();
    let group_alias = identity::group_alias::read_by_id(client, group_alias_id)
        .await
        .unwrap();
    assert_eq!(group_alias.name, NEW_NAME);
}

async fn test_list_group_aliases_by_id(client: &VaultClient, group_alias_id: &str) {
    let groups = identity::group_alias::list_by_id(client).await.unwrap();
    assert_eq!(groups.keys, [group_alias_id]);
}

async fn test_delete_group_alias_by_id(client: &VaultClient, group_alias_id: &str) {
    identity::group_alias::delete_by_id(client, group_alias_id)
        .await
        .unwrap();

    assert!(matches!(
        identity::group_alias::read_by_id(client, group_alias_id)
            .await
            .err()
            .unwrap(),
        ClientError::APIError { code: 404, .. }
    ));
}
