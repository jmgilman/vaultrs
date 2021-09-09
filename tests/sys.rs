pub const VERSION: &str = "1.8.2";

use vaultrs::{
    api::{sys::requests::ListMountsRequest, ResponseWrapper},
    sys::{self},
};
use vaultrs_test::docker::{Server, ServerConfig};
use vaultrs_test::{VaultServer, VaultServerConfig};

#[test]
fn test() {
    let config = VaultServerConfig::default(Some(VERSION));
    let instance = config.to_instance();

    instance.run(|ops| async move {
        let server = VaultServer::new(&ops, &config);

        // Test wrapping
        test_wrap(&server).await;

        // Test health
        test_health(&server).await;

        // Test status
        test_status(&server).await;

        // Test mount
        crate::mount::test_create_mount(&server).await;
        crate::mount::test_list_mount(&server).await;

        // Test auth
        crate::auth::test_create_auth(&server).await;
        crate::auth::test_list_auth(&server).await;

        // Test policy
        crate::policy::test_set_policy(&server).await;
        crate::policy::test_read_policy(&server).await;
        crate::policy::test_list_policies(&server).await;
        crate::policy::test_delete_policy(&server).await;

        // Test sealing
        test_seal(&server).await;
    });
}

async fn test_wrap(server: &VaultServer) {
    let endpoint = ListMountsRequest::builder().build().unwrap();
    let wrap_resp = endpoint.wrap(&server.client).await;
    assert!(wrap_resp.is_ok());

    let wrap_resp = wrap_resp.unwrap();
    let info = wrap_resp.lookup(&server.client).await;
    assert!(info.is_ok());

    let unwrap_resp = wrap_resp.unwrap(&server.client).await;
    assert!(unwrap_resp.is_ok());

    let info = wrap_resp.lookup(&server.client).await;
    assert!(info.is_err());
}

async fn test_health(server: &VaultServer) {
    let resp = sys::health(&server.client).await;
    assert!(resp.is_ok());
}

async fn test_seal(server: &VaultServer) {
    let resp = sys::seal(&server.client).await;
    assert!(resp.is_ok());
}

async fn test_status(server: &VaultServer) {
    let resp = sys::status(&server.client).await;
    assert!(matches!(resp, sys::ServerStatus::OK));
}

mod mount {
    use super::VaultServer;
    use vaultrs::sys::mount;

    pub async fn test_create_mount(server: &VaultServer) {
        let resp = mount::enable(&server.client, "pki_temp", "pki", None).await;
        assert!(resp.is_ok());
    }

    pub async fn test_list_mount(server: &VaultServer) {
        let resp = mount::list(&server.client).await;
        assert!(resp.is_ok());
    }
}

mod auth {
    use super::VaultServer;
    use vaultrs::sys::auth;

    pub async fn test_create_auth(server: &VaultServer) {
        let resp = auth::enable(&server.client, "oidc_temp", "oidc", None).await;
        assert!(resp.is_ok());
    }

    pub async fn test_list_auth(server: &VaultServer) {
        let resp = auth::list(&server.client).await;
        assert!(resp.is_ok());
    }
}

mod policy {
    use super::VaultServer;
    use vaultrs::sys::policy;

    pub async fn test_delete_policy(server: &VaultServer) {
        let resp = policy::delete(&server.client, "test").await;
        assert!(resp.is_ok());
    }

    pub async fn test_list_policies(server: &VaultServer) {
        let resp = policy::list(&server.client).await;
        assert!(resp.is_ok());
    }

    pub async fn test_read_policy(server: &VaultServer) {
        let resp = policy::read(&server.client, "test").await;
        assert!(resp.is_ok());
    }

    pub async fn test_set_policy(server: &VaultServer) {
        let policy = r#"
            path "sys" {
                capabilities = ["list"]
            }"#;

        let resp = policy::set(&server.client, "test", policy).await;
        assert!(resp.is_ok());
    }
}
