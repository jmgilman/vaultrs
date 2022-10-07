extern crate tracing;

mod common;

use common::{VaultServer, VaultServerHelper};
use test_log::test;
use vaultrs::{kv};
use std::collections::HashMap;

use vaultrs::api::kv::responses::GetSecretResponse;


#[test]
fn test_kv1() {
    let test = common::new_test();
    test.run(|instance| async move {
        let server: VaultServer = instance.server();
        let client = server.client();

        // Mount KV v1 secret engine
        let mount = "kv_v1";
        let secret_path = "mysecret/foo";
        server.mount_secret(&client, mount, "kv").await.unwrap();

        // Create test secrets
        let expected_secret = HashMap::from([ 
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string())
        ]); 
        kv::set(&client, mount, &secret_path, &expected_secret).await.unwrap();

        // Read it
        let read_secret: HashMap<String, String> = kv::get(&client, &mount, &secret_path).await.unwrap();

        println!("{:?}", read_secret);

        assert_eq!(read_secret.get("key1").unwrap(), expected_secret.get("key1").unwrap());
        assert_eq!(read_secret.get("key2").unwrap(), expected_secret.get("key2").unwrap());

        // Read it as raw value
        let read_secret_raw: GetSecretResponse = kv::get_raw(&client, &mount, &secret_path).await.unwrap();

        println!("{:?}", read_secret_raw);

        assert_eq!(read_secret_raw.data.get("key1").unwrap(), expected_secret.get("key1").unwrap());
        assert_eq!(read_secret_raw.data.get("key2").unwrap(), expected_secret.get("key2").unwrap());

        // List secret keys
        let list_secret = kv::list(&client, &mount, "mysecret").await.unwrap();

        println!("{:?}", list_secret);

        assert_eq!(list_secret.data.keys, vec!["foo"]);
    });
}
