use vaultrs::{
    api::{sys::requests::ListMountsRequest, ResponseWrapper},
    client::Client,
    error::ClientError,
    sys,
};

use crate::common::Test;

#[tokio::test]
async fn test() {
    let test = Test::builder().await;
    let client = test.client();

    // Test wrapping
    test_wrap(client).await;

    // Test health
    test_health(client).await;

    // Test initialization
    test_start_initialization_failure(client).await;

    // Test status
    test_status(client).await;

    // Test mount
    mount::test_create_mount(client).await;
    mount::test_list_mount(client).await;
    mount::test_get_configuration_of_a_secret_engine(client).await;
    mount::test_delete_mount(client).await;

    // Test remount
    remount::test_remount(client).await;

    // Test auth
    auth::test_create_auth(client).await;
    auth::test_list_auth(client).await;
    auth::test_disable_auth(client).await;

    // Test policy
    policy::test_set_policy(client).await;
    policy::test_read_policy(client).await;
    policy::test_list_policies(client).await;
    policy::test_delete_policy(client).await;

    // Test tools
    tools::test_random(client).await;

    // Test sealing
    test_seal(client).await;
}

#[tokio::test]
async fn sys_init() {
    let test = Test::new_prod().await;
    let client = test.client();
    test_start_initialization(client).await;
}

async fn test_wrap(client: &impl Client) {
    let endpoint = ListMountsRequest::builder().build().unwrap();
    let wrap_resp = endpoint.wrap(client).await.unwrap();
    wrap_resp.lookup(client).await.unwrap();

    wrap_resp.unwrap(client).await.unwrap();

    wrap_resp.lookup(client).await.unwrap_err();
}

async fn test_health(client: &impl Client) {
    sys::health(client).await.unwrap();
}

async fn test_start_initialization_failure(client: &impl Client) {
    let resp = sys::start_initialization(client, 1, 1, None)
        .await
        .unwrap_err();
    let ClientError::APIError { code, .. } = resp else {
        panic!("must return an error because already initialized")
    };
    assert_eq!(code, 400);
}

async fn test_start_initialization(client: &impl Client) {
    let resp = sys::start_initialization(client, 1, 1, None).await.unwrap();
    assert_eq!(resp.keys.len(), 1);
}

async fn test_seal(client: &impl Client) {
    sys::seal(client).await.unwrap();
}

async fn test_status(client: &impl Client) {
    assert!(matches!(
        sys::status(client).await.unwrap(),
        sys::ServerStatus::OK
    ));
}

mod mount {
    use super::Client;
    use vaultrs::sys::mount;

    pub async fn test_create_mount(client: &impl Client) {
        mount::enable(client, "pki_temp", "pki", None)
            .await
            .unwrap();
    }

    pub async fn test_list_mount(client: &impl Client) {
        mount::list(client).await.unwrap();
    }
    pub async fn test_get_configuration_of_a_secret_engine(client: &impl Client) {
        mount::get_configuration_of_a_secret_engine(client, "pki_temp")
            .await
            .unwrap();
    }

    pub async fn test_delete_mount(client: &impl Client) {
        mount::disable(client, "pki_temp").await.unwrap();
        mount::get_configuration_of_a_secret_engine(client, "pki_temp")
            .await
            .unwrap_err();
    }
}

mod remount {
    use std::collections::HashMap;
    use std::time::Duration;

    use super::Client;
    use vaultrs::kv1;
    use vaultrs::sys::mount;
    use vaultrs::sys::remount;

    pub async fn test_remount(client: &impl Client) {
        const OLD_MOUNT: &str = "kv_old";
        const NEW_MOUNT: &str = "kv_new";
        const SECRET_PATH: &str = "mysecret/foo";

        // Create new mount.
        mount::enable(client, OLD_MOUNT, "kv", None).await.unwrap();
        let expected_secret = HashMap::from([("key", "value")]);
        kv1::set(client, OLD_MOUNT, SECRET_PATH, &expected_secret)
            .await
            .unwrap();
        let read_secret: HashMap<String, String> =
            kv1::get(client, OLD_MOUNT, SECRET_PATH).await.unwrap();
        assert_eq!(read_secret["key"], expected_secret["key"]);

        // Perform the migration
        let migration_id = remount::remount(client, OLD_MOUNT, NEW_MOUNT)
            .await
            .unwrap()
            .migration_id;
        while remount::remount_status(client, &migration_id)
            .await
            .unwrap()
            .migration_info
            .status
            != "success"
        {
            tokio::time::sleep(Duration::from_millis(500)).await;
        }

        // Ensure the migration succeeded
        kv1::get::<HashMap<String, String>>(client, OLD_MOUNT, SECRET_PATH)
            .await
            .unwrap_err();
        let read_secret: HashMap<String, String> =
            kv1::get(client, NEW_MOUNT, SECRET_PATH).await.unwrap();
        assert_eq!(read_secret["key"], expected_secret["key"]);
    }
}

mod auth {
    use super::Client;
    use vaultrs::sys::auth;

    pub async fn test_create_auth(client: &impl Client) {
        auth::enable(client, "oidc_temp", "oidc", None)
            .await
            .unwrap();
    }

    pub async fn test_list_auth(client: &impl Client) {
        auth::list(client).await.unwrap();
    }

    pub async fn test_disable_auth(client: &impl Client) {
        auth::disable(client, "oidc_temp").await.unwrap();
    }
}

mod policy {
    use super::Client;
    use vaultrs::sys::policy;

    pub async fn test_delete_policy(client: &impl Client) {
        policy::delete(client, "test").await.unwrap();
    }

    pub async fn test_list_policies(client: &impl Client) {
        policy::list(client).await.unwrap();
    }

    pub async fn test_read_policy(client: &impl Client) {
        policy::read(client, "test").await.unwrap();
    }

    pub async fn test_set_policy(client: &impl Client) {
        let policy = r#"
            path "sys" {
                capabilities = ["list"]
            }"#;

        policy::set(client, "test", policy).await.unwrap();
    }
}

mod tools {
    use super::Client;
    use vaultrs::{api::sys::requests::RandomRequestBuilder, sys::tools};
    pub async fn test_random(client: &impl Client) {
        let random = tools::random(client, None).await.unwrap();
        assert!(!random.random_bytes.is_empty());

        let random = tools::random(
            client,
            Some(&mut RandomRequestBuilder::default().bytes(3u64)),
        )
        .await
        .unwrap();
        assert!(!random.random_bytes.is_empty());

        let random = tools::random(
            client,
            Some(&mut RandomRequestBuilder::default().source("platform")),
        )
        .await
        .unwrap();
        assert!(!random.random_bytes.is_empty());

        let random = tools::random(
            client,
            Some(&mut RandomRequestBuilder::default().format("base64")),
        )
        .await
        .unwrap();
        assert!(!random.random_bytes.is_empty());

        let random = tools::random(
            client,
            Some(&mut RandomRequestBuilder::default().format("hex")),
        )
        .await
        .unwrap();
        assert!(!random.random_bytes.is_empty());
    }
}
