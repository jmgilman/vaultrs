#[macro_use]
extern crate tracing;

mod common;

use common::{VaultServer, VaultServerHelper};
use dockertest_server::servers::webserver::nginx::NginxServer;
use test_env_log::test;
use vaultrs::api::auth::kubernetes::requests::ConfigureKubernetesAuthRequest;
use vaultrs::client::Client;
use vaultrs::error::ClientError;

#[test]
fn test() {
    let (test, _content) = common::new_webserver_test();
    test.run(|instance| async move {
        let server: VaultServer = instance.server();
        let webserver: NginxServer = instance.server();
        let client = server.client();
        let endpoint = setup(&server, &client, &webserver).await.unwrap();

        // Test pre-configure auth backend
        test_configure(&client, &endpoint).await;
        test_read_config(&client, &endpoint).await;

        // Test roles
        crate::role::test_create(&client, &endpoint).await;
        crate::role::test_read(&client, &endpoint).await;
        crate::role::test_list(&client, &endpoint).await;

        crate::test_login(&client, &endpoint).await;

        crate::role::test_delete(&client, &endpoint).await;
    })
}

pub async fn test_configure(client: &impl Client, endpoint: &KubernetesRoleEndpoint) {
    let resp = vaultrs::auth::kubernetes::configure(
        client,
        &endpoint.path,
        &endpoint.kubernetes_host,
        Some(
            &mut ConfigureKubernetesAuthRequest::builder()
                .kubernetes_host(&format!("https://{}", &endpoint.kubernetes_host))
                .kubernetes_ca_cert(include_str!("files/kubernetes/ca.crt"))
                .issuer(&endpoint.jtw_issuer),
        ),
    )
    .await;
    assert!(resp.is_ok());
}

pub async fn test_read_config(client: &impl Client, endpoint: &KubernetesRoleEndpoint) {
    let res = vaultrs::auth::kubernetes::read_config(client, endpoint.path.as_str()).await;
    assert!(res.is_ok());
}

pub async fn test_login(client: &impl Client, endpoint: &KubernetesRoleEndpoint) {
    use hmac::{Hmac, NewMac};
    use jwt::SignWithKey;
    use sha2::Sha256;
    use std::collections::BTreeMap;

    let key: Hmac<Sha256> = Hmac::new_from_slice(b"test-secret").unwrap();
    let mut claims = BTreeMap::new();
    let subject = format!(
        "system:serviceaccount:{}:test",
        &endpoint.kubernetes_namespace
    );
    claims.insert("iss", endpoint.jtw_issuer.as_str());
    claims.insert("kubernetes.io/serviceaccount/service-account.name", "test");
    claims.insert(
        "kubernetes.io/serviceaccount/service-account.uid",
        "testuid",
    );
    claims.insert(
        "kubernetes.io/serviceaccount/namespace",
        endpoint.kubernetes_namespace.as_str(),
    );
    claims.insert("sub", &subject);

    let token_str = claims.sign_with_key(&key).unwrap();

    let resp =
        vaultrs::auth::kubernetes::login(client, &endpoint.path, &endpoint.role_name, &token_str)
            .await;

    assert!(resp.is_ok());
}

mod role {
    use super::{Client, KubernetesRoleEndpoint};
    use vaultrs::api::auth::kubernetes::requests::CreateKubernetesRoleRequest;

    pub async fn test_delete(client: &impl Client, endpoint: &KubernetesRoleEndpoint) {
        let res = vaultrs::auth::kubernetes::role::delete(
            client,
            endpoint.path.as_str(),
            endpoint.role_name.as_str(),
        )
        .await;
        assert!(res.is_ok());
    }

    pub async fn test_list(client: &impl Client, endpoint: &KubernetesRoleEndpoint) {
        let res = vaultrs::auth::kubernetes::role::list(client, endpoint.path.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_read(client: &impl Client, endpoint: &KubernetesRoleEndpoint) {
        let res = vaultrs::auth::kubernetes::role::read(
            client,
            endpoint.path.as_str(),
            endpoint.role_name.as_str(),
        )
        .await;
        assert!(res.is_ok());
    }

    pub async fn test_create(client: &impl Client, endpoint: &KubernetesRoleEndpoint) {
        let res = vaultrs::auth::kubernetes::role::create(
            client,
            &endpoint.path,
            &endpoint.role_name,
            Some(
                &mut CreateKubernetesRoleRequest::builder()
                    .bound_service_account_names(vec!["test".to_string()])
                    .bound_service_account_namespaces(vec![endpoint.kubernetes_namespace.clone()])
                    .token_ttl("10m"),
            ),
        )
        .await;
        assert!(res.is_ok());
    }
}

#[derive(Clone, Debug)]
pub struct KubernetesRoleEndpoint {
    pub path: String,
    pub role_name: String,
    pub kubernetes_host: String,
    pub jtw_issuer: String,
    pub kubernetes_namespace: String,
}

async fn setup(
    server: &VaultServer,
    client: &impl Client,
    webserver: &NginxServer,
) -> Result<KubernetesRoleEndpoint, ClientError> {
    debug!("setting up Kubernetes auth engine");
    let path = "kubernetes_test";
    let role_name = "test";

    // Mount the AppRole auth engine
    server.mount_auth(client, path, "kubernetes").await?;

    Ok(KubernetesRoleEndpoint {
        path: path.to_string(),
        role_name: role_name.to_string(),
        kubernetes_host: webserver.internal_url().to_string(),
        jtw_issuer: "vaultrs/test".to_string(),
        kubernetes_namespace: "testns".to_string(),
    })
}
