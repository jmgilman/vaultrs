#[macro_use]
extern crate tracing;

mod common;

use common::VaultServerHelper;
use vaultrs::{
    api::{sys::requests::ListMountsRequest, ResponseWrapper},
    client::Client,
    sys::{self},
};
use vaultrs_test::docker::{Server, ServerConfig};
use vaultrs_test::{VaultServer, VaultServerConfig};

#[tracing_test::traced_test]
#[test]
fn test() {
    let config = VaultServerConfig::default(Some(common::VERSION));
    let instance = config.to_instance();

    instance.run(|ops| async move {
        let server = VaultServer::new(&ops, &config);
        let client = server.client();

        // Test wrapping
        test_wrap(&client).await;

        // Test health
        test_health(&client).await;

        // Test status
        test_status(&client).await;

        // Test mount
        crate::mount::test_create_mount(&client).await;
        crate::mount::test_list_mount(&client).await;

        // Test auth
        crate::auth::test_create_auth(&client).await;
        crate::auth::test_list_auth(&client).await;

        // Test policy
        crate::policy::test_set_policy(&client).await;
        crate::policy::test_read_policy(&client).await;
        crate::policy::test_list_policies(&client).await;
        crate::policy::test_delete_policy(&client).await;

        // Test sealing
        test_seal(&client).await;
    });
}

#[instrument(skip(client))]
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

#[instrument(skip(client))]
async fn test_health(client: &impl Client) {
    let resp = sys::health(client).await;
    assert!(resp.is_ok());
}

#[instrument(skip(client))]
async fn test_seal(client: &impl Client) {
    let resp = sys::seal(client).await;
    assert!(resp.is_ok());
}

#[instrument(skip(client))]
async fn test_status(client: &impl Client) {
    let resp = sys::status(client).await;
    assert!(resp.is_ok());
    assert!(matches!(resp.unwrap(), sys::ServerStatus::OK));
}

mod mount {
    use super::Client;
    use vaultrs::sys::mount;

    #[instrument(skip(client))]
    pub async fn test_create_mount(client: &impl Client) {
        let resp = mount::enable(client, "pki_temp", "pki", None).await;
        assert!(resp.is_ok());
    }

    #[instrument(skip(client))]
    pub async fn test_list_mount(client: &impl Client) {
        let resp = mount::list(client).await;
        assert!(resp.is_ok());
    }
}

mod auth {
    use super::Client;
    use vaultrs::sys::auth;

    #[instrument(skip(client))]
    pub async fn test_create_auth(client: &impl Client) {
        let resp = auth::enable(client, "oidc_temp", "oidc", None).await;
        assert!(resp.is_ok());
    }

    #[instrument(skip(client))]
    pub async fn test_list_auth(client: &impl Client) {
        let resp = auth::list(client).await;
        assert!(resp.is_ok());
    }
}

mod policy {
    use super::Client;
    use vaultrs::sys::policy;

    #[instrument(skip(client))]
    pub async fn test_delete_policy(client: &impl Client) {
        let resp = policy::delete(client, "test").await;
        assert!(resp.is_ok());
    }

    #[instrument(skip(client))]
    pub async fn test_list_policies(client: &impl Client) {
        let resp = policy::list(client).await;
        assert!(resp.is_ok());
    }

    #[instrument(skip(client))]
    pub async fn test_read_policy(client: &impl Client) {
        let resp = policy::read(client, "test").await;
        assert!(resp.is_ok());
    }

    #[instrument(skip(client))]
    pub async fn test_set_policy(client: &impl Client) {
        let policy = r#"
            path "sys" {
                capabilities = ["list"]
            }"#;

        let resp = policy::set(client, "test", policy).await;
        assert!(resp.is_ok());
    }
}
