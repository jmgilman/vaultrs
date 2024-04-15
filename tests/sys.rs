mod common;
mod vault_prod_container;

use std::collections::HashMap;

use common::{VaultServer, VaultServerHelper, PORT, VERSION};
use dockertest_server::Test;
use test_log::test;
use vaultrs::{
    api::{sys::requests::ListMountsRequest, ResponseWrapper},
    client::{Client, VaultClient, VaultClientSettingsBuilder},
    error::ClientError,
    sys,
};

#[test]
fn test() {
    let test = common::new_test();
    test.run(|instance| async move {
        let server: VaultServer = instance.server();
        let client = server.client();

        // Test wrapping
        test_wrap(&client).await;

        // Test health
        test_health(&client).await;

        // Test initialization
        test_start_initialization_failure(&client).await;

        // Test status
        test_status(&client).await;

        // Test mount
        crate::mount::test_create_mount(&client).await;
        crate::mount::test_list_mount(&client).await;
        crate::mount::test_get_configuration_of_a_secret_engine(&client).await;
        crate::mount::test_delete_mount(&client).await;

        // Test auth
        crate::auth::test_create_auth(&client).await;
        crate::auth::test_list_auth(&client).await;

        // Test policy
        crate::policy::test_set_policy(&client).await;
        crate::policy::test_read_policy(&client).await;
        crate::policy::test_list_policies(&client).await;
        crate::policy::test_delete_policy(&client).await;

        // Test tools
        crate::tools::test_random(&client).await;

        // Test sealing
        test_seal(&client).await;
    });
}

#[test]
fn sys_init() {
    let test = new_prod_test();
    test.run(|instance| async move {
        let server: vault_prod_container::VaultServer = instance.server();
        let client = VaultClient::new(
            VaultClientSettingsBuilder::default()
                .address(server.external_url())
                .build()
                .unwrap(),
        )
        .unwrap();
        test_start_initialization(&client).await;
    });
}

async fn test_wrap(client: &impl Client) {
    let endpoint = ListMountsRequest::builder().build().unwrap();
    let wrap_resp = endpoint.wrap(client).await;
    assert!(wrap_resp.is_ok());

    let wrap_resp = wrap_resp.unwrap();
    let info = wrap_resp.lookup(client).await;
    assert!(info.is_ok());

    let unwrap_resp = wrap_resp.unwrap(client).await;
    assert!(unwrap_resp.is_ok());

    let info = wrap_resp.lookup(client).await;
    assert!(info.is_err());
}

async fn test_health(client: &impl Client) {
    let resp = sys::health(client).await;
    assert!(resp.is_ok());
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
    let resp = sys::seal(client).await;
    assert!(resp.is_ok());
}

async fn test_status(client: &impl Client) {
    let resp = sys::status(client).await;
    assert!(resp.is_ok());
    assert!(matches!(resp.unwrap(), sys::ServerStatus::OK));
}

mod mount {
    use super::Client;
    use vaultrs::sys::mount;

    pub async fn test_create_mount(client: &impl Client) {
        let resp = mount::enable(client, "pki_temp", "pki", None).await;
        assert!(resp.is_ok());
    }

    pub async fn test_list_mount(client: &impl Client) {
        let resp = mount::list(client).await;
        assert!(resp.is_ok());
    }
    pub async fn test_get_configuration_of_a_secret_engine(client: &impl Client) {
        mount::get_configuration_of_a_secret_engine(client, "pki_temp")
            .await
            .unwrap();
    }

    pub async fn test_delete_mount(client: &impl Client) {
        mount::disable(client, "pki_temp").await.unwrap();
        assert!(
            mount::get_configuration_of_a_secret_engine(client, "pki_temp")
                .await
                .is_err()
        );
    }
}

mod auth {
    use super::Client;
    use vaultrs::sys::auth;

    pub async fn test_create_auth(client: &impl Client) {
        let resp = auth::enable(client, "oidc_temp", "oidc", None).await;
        assert!(resp.is_ok());
    }

    pub async fn test_list_auth(client: &impl Client) {
        let resp = auth::list(client).await;
        assert!(resp.is_ok());
    }
}

mod policy {
    use super::Client;
    use vaultrs::sys::policy;

    pub async fn test_delete_policy(client: &impl Client) {
        let resp = policy::delete(client, "test").await;
        assert!(resp.is_ok());
    }

    pub async fn test_list_policies(client: &impl Client) {
        let resp = policy::list(client).await;
        assert!(resp.is_ok());
    }

    pub async fn test_read_policy(client: &impl Client) {
        let resp = policy::read(client, "test").await;
        assert!(resp.is_ok());
    }

    pub async fn test_set_policy(client: &impl Client) {
        let policy = r#"
            path "sys" {
                capabilities = ["list"]
            }"#;

        let resp = policy::set(client, "test", policy).await;
        assert!(resp.is_ok());
    }
}

// Sets up a new test using the vault production server.
pub fn new_prod_test() -> Test {
    let mut test = Test::default();
    let vault_config = serde_json::json!({
        "listener": [
            {
                "tcp": {
                    "address": "0.0.0.0:8300",
                    "tls_disable": "true"
                }
            }
        ],
        "storage": [
            {
                "inmem": {}
            }
        ],
        "disable_mlock": true

    });
    let env = HashMap::from([("VAULT_LOCAL_CONFIG".to_string(), vault_config.to_string())]);
    let config = vault_prod_container::VaultServerConfig::builder()
        .port(PORT)
        .version(VERSION.into())
        .env(env)
        .build()
        .unwrap();
    test.register(config);
    test
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
