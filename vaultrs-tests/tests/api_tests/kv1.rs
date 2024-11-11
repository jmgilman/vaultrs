use crate::common::Test;
use std::collections::HashMap;
use vaultrs::{api::kv1::responses::GetSecretResponse, error::ClientError, kv1, sys::mount};

#[tokio::test]
async fn test_kv1() {
    let test = Test::builder().await;
    let client = test.client();

    // Mount KV v1 secret engine
    let mount = "kv_v1";
    let secret_path = "mysecret/foo";
    mount::enable(client, mount, "kv", None).await.unwrap();

    // Create test secrets
    let expected_secret = HashMap::from([("key1", "value1"), ("key2", "value2")]);
    kv1::set(client, mount, secret_path, &expected_secret)
        .await
        .unwrap();

    // Read it
    let read_secret: HashMap<String, String> = kv1::get(client, mount, secret_path).await.unwrap();

    println!("{:?}", read_secret);

    assert_eq!(
        read_secret.get("key1").unwrap(),
        expected_secret.get("key1").unwrap()
    );
    assert_eq!(
        read_secret.get("key2").unwrap(),
        expected_secret.get("key2").unwrap()
    );

    // Read it as raw value
    let read_secret_raw: GetSecretResponse =
        kv1::get_raw(client, mount, secret_path).await.unwrap();

    println!("{:?}", read_secret_raw);

    assert_eq!(
        read_secret_raw.data.get("key1").unwrap(),
        expected_secret.get("key1").unwrap()
    );
    assert_eq!(
        read_secret_raw.data.get("key2").unwrap(),
        expected_secret.get("key2").unwrap()
    );

    // List secret keys
    let list_secret = kv1::list(client, mount, "mysecret").await.unwrap();

    println!("{:?}", list_secret);

    assert_eq!(list_secret.data.keys, vec!["foo"]);

    // Delete secret and read again and expect 404 to check deletion
    kv1::delete(client, mount, secret_path).await.unwrap();

    let r = kv1::get_raw(client, mount, secret_path).await;

    match r.expect_err(&format!(
        "Expected error when reading {} after delete.",
        &secret_path
    )) {
        ClientError::APIError { code, .. } => {
            assert_eq!(code, 404, "Expected error code 404 for non-existing secret")
        }
        e => {
            panic!("Expected error to be APIError with code 404, got {:?}", e)
        }
    };

    let my_secrets = HashMap::from([("key1", "value1"), ("key2", "value2")]);

    kv1::set(client, mount, "my/secrets", &my_secrets)
        .await
        .unwrap();

    let read_secrets: HashMap<String, String> =
        kv1::get(client, mount, "my/secrets").await.unwrap();

    println!("{:}", read_secrets.get("key1").unwrap()); // value1

    let list_secret = kv1::list(client, mount, "my").await.unwrap();

    println!("{:?}", list_secret.data.keys); // [ "secrets" ]

    kv1::delete(client, mount, "my/secrets").await.unwrap();
}
