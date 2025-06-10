use crate::common::Test;
use std::collections::HashMap;
use vaultrs::{api::cubbyhole::responses::GetSecretResponse, cubbyhole, error::ClientError};

#[tokio::test]
async fn test_cubbyhole() {
    let test = Test::builder().await;
    let client = test.client();

    // Use pre-mounted cubbyhole secret engine
    let mount = "cubbyhole";
    let secret_path = "mysecret/foo";

    // Create test secrets
    let expected_secret = HashMap::from([("key1", "value1"), ("key2", "value2")]);
    cubbyhole::set(client, mount, secret_path, &expected_secret)
        .await
        .unwrap();

    // Read it
    let read_secret: HashMap<String, String> =
        cubbyhole::get(client, mount, secret_path).await.unwrap();

    println!("{:?}", read_secret);

    assert_eq!(read_secret["key1"], expected_secret["key1"]);
    assert_eq!(read_secret["key2"], expected_secret["key2"]);

    // Read it as raw value
    let read_secret_raw: GetSecretResponse = cubbyhole::get_raw(client, mount, secret_path)
        .await
        .unwrap();

    println!("{:?}", read_secret_raw);

    assert_eq!(read_secret_raw.data["key1"], expected_secret["key1"]);
    assert_eq!(read_secret_raw.data["key2"], expected_secret["key2"]);

    // List secret keys
    let list_secret = cubbyhole::list(client, mount, "mysecret").await.unwrap();

    println!("{:?}", list_secret);

    assert_eq!(list_secret.data.keys, vec!["foo"]);

    // Delete secret and read again and expect 404 to check deletion
    cubbyhole::delete(client, mount, secret_path).await.unwrap();

    let r = cubbyhole::get_raw(client, mount, secret_path).await;

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

    cubbyhole::set(client, mount, "my/secrets", &my_secrets)
        .await
        .unwrap();

    let read_secrets: HashMap<String, String> =
        cubbyhole::get(client, mount, "my/secrets").await.unwrap();

    println!("{:}", read_secrets["key1"]); // value1

    let list_secret = cubbyhole::list(client, mount, "my").await.unwrap();

    println!("{:?}", list_secret.data.keys); // [ "secrets" ]

    cubbyhole::delete(client, mount, "my/secrets")
        .await
        .unwrap();
}
