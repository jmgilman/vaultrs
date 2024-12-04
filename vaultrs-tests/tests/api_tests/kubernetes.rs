use tracing::debug;
use vaultrs::api::auth::kubernetes::requests::ConfigureKubernetesAuthRequest;
use vaultrs::client::Client;
use vaultrs::error::ClientError;
use vaultrs::sys::auth;

use crate::common::Test;

#[tokio::test]
async fn test() {
    let test = Test::builder().with_nginx().await;
    let client = test.client();
    let nginx_server_addr = test.nginx_url().unwrap();
    let endpoint = setup(client, nginx_server_addr).await.unwrap();

    // Test pre-configure auth backend
    test_configure(client, &endpoint).await;
    test_read_config(client, &endpoint).await;

    // Test roles
    role::test_create(client, &endpoint).await;
    role::test_read(client, &endpoint).await;
    role::test_list(client, &endpoint).await;

    // That's the only test failing
    test_login(client, &endpoint).await;

    role::test_delete(client, &endpoint).await;
}

pub async fn test_configure(client: &impl Client, endpoint: &KubernetesRoleEndpoint) {
    vaultrs::auth::kubernetes::configure(
        client,
        &endpoint.path,
        &endpoint.kubernetes_host,
        Some(
            &mut ConfigureKubernetesAuthRequest::builder()
                .kubernetes_host(format!("http://{}", &endpoint.kubernetes_host))
                .kubernetes_ca_cert(include_str!("../files/kubernetes/ca.crt"))
                .issuer(&endpoint.jtw_issuer),
        ),
    )
    .await
    .unwrap();
}

pub async fn test_read_config(client: &impl Client, endpoint: &KubernetesRoleEndpoint) {
    vaultrs::auth::kubernetes::read_config(client, endpoint.path.as_str())
        .await
        .unwrap();
}

pub async fn test_login(client: &impl Client, endpoint: &KubernetesRoleEndpoint) {
    use hmac::{Hmac, Mac};
    use jwt::SignWithKey;
    use sha2::Sha256;
    use std::collections::BTreeMap;

    type HmacSha256 = Hmac<Sha256>;
    let key = HmacSha256::new_from_slice(b"test-secret").unwrap();
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

    vaultrs::auth::kubernetes::login(client, &endpoint.path, &endpoint.role_name, &token_str)
        .await
        .unwrap();
}

mod role {
    use super::{Client, KubernetesRoleEndpoint};
    use vaultrs::api::auth::kubernetes::requests::CreateKubernetesRoleRequest;

    pub async fn test_delete(client: &impl Client, endpoint: &KubernetesRoleEndpoint) {
        vaultrs::auth::kubernetes::role::delete(
            client,
            endpoint.path.as_str(),
            endpoint.role_name.as_str(),
        )
        .await
        .unwrap();
    }

    pub async fn test_list(client: &impl Client, endpoint: &KubernetesRoleEndpoint) {
        vaultrs::auth::kubernetes::role::list(client, endpoint.path.as_str())
            .await
            .unwrap();
    }

    pub async fn test_read(client: &impl Client, endpoint: &KubernetesRoleEndpoint) {
        vaultrs::auth::kubernetes::role::read(
            client,
            endpoint.path.as_str(),
            endpoint.role_name.as_str(),
        )
        .await
        .unwrap();
    }

    pub async fn test_create(client: &impl Client, endpoint: &KubernetesRoleEndpoint) {
        vaultrs::auth::kubernetes::role::create(
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
        .await
        .unwrap();
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
    client: &impl Client,
    nginx_server_addr: &str,
) -> Result<KubernetesRoleEndpoint, ClientError> {
    debug!("setting up Kubernetes auth engine");
    let path = "kubernetes_test";
    let role_name = "test";

    // Mount the AppRole auth engine
    auth::enable(client, path, "kubernetes", None)
        .await
        .unwrap();

    Ok(KubernetesRoleEndpoint {
        path: path.to_string(),
        role_name: role_name.to_string(),
        kubernetes_host: nginx_server_addr.to_string(),
        jtw_issuer: "vaultrs/test".to_string(),
        kubernetes_namespace: "testns".to_string(),
    })
}
