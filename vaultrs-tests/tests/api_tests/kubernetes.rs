use tracing::debug;
use vaultrs::api::auth::kubernetes::requests::ConfigureKubernetesAuthRequest;
use vaultrs::client::Client;
use vaultrs::error::ClientError;
use vaultrs::sys::auth;

use crate::common::{Test, KUB_ACCOUNT_NAME};

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
                .kubernetes_ca_cert(include_str!("../files/kubernetes/ca.crt")), // .issuer(&endpoint.jtw_issuer),
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
    // We use the same test vector than official vault client: <https://github.com/hashicorp/vault/blob/dbc2f06fbdb5f523dec18a7c52d06c5f68ce45d7/command/agentproxyshared/auth/kubernetes/kubernetes_test.go#L100>
    vaultrs::auth::kubernetes::login(client, &endpoint.path, &endpoint.role_name, "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJrdWJlcm5ldGVzL3NlcnZpY2VhY2NvdW50Iiwia3ViZXJuZXRlcy5pby9zZXJ2aWNlYWNjb3VudC9uYW1lc3BhY2UiOiJkZWZhdWx0Iiwia3ViZXJuZXRlcy5pby9zZXJ2aWNlYWNjb3VudC9zZWNyZXQubmFtZSI6InZhdWx0LWF1dGgtdG9rZW4tdDVwY24iLCJrdWJlcm5ldGVzLmlvL3NlcnZpY2VhY2NvdW50L3NlcnZpY2UtYWNjb3VudC5uYW1lIjoidmF1bHQtYXV0aCIsImt1YmVybmV0ZXMuaW8vc2VydmljZWFjY291bnQvc2VydmljZS1hY2NvdW50LnVpZCI6ImQ3N2Y4OWJjLTkwNTUtMTFlNy1hMDY4LTA4MDAyNzZkOTliZiIsInN1YiI6InN5c3RlbTpzZXJ2aWNlYWNjb3VudDpkZWZhdWx0OnZhdWx0LWF1dGgifQ.HKUcqgrvan5ZC_mnpaMEx4RW3KrhfyH_u8G_IA2vUfkLK8tH3T7fJuJaPr7W6K_BqCrbeM5y3owszOzb4NR0Lvw6GBt2cFcen2x1Ua4Wokr0bJjTT7xQOIOw7UvUDyVS17wAurlfUnmWMwMMMOebpqj5K1t6GnyqghH1wPdHYRGX-q5a6C323dBCgM5t6JY_zTTaBgM6EkFq0poBaifmSMiJRPrdUN_-IgyK8fgQRiFYYkgS6DMIU4k4nUOb_sUFf5xb8vMs3SMteKiuWFAIt4iszXTj5IyBUNqe0cXA3zSY3QiNCV6bJ2CWW0Qf9WDtniT79VAqcR4GYaTC_gxjNA")
        .await
        .unwrap();
}

mod role {
    use crate::common::{KUB_ACCOUNT_NAME, KUB_NAMESPACE};

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
                    .bound_service_account_namespaces(vec![KUB_NAMESPACE.into()])
                    .bound_service_account_names(vec![KUB_ACCOUNT_NAME.into()])
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
}

async fn setup(
    client: &impl Client,
    nginx_server_addr: &str,
) -> Result<KubernetesRoleEndpoint, ClientError> {
    debug!("setting up Kubernetes auth engine");
    let path = "kubernetes_test";

    // Mount the AppRole auth engine
    auth::enable(client, path, "kubernetes", None)
        .await
        .unwrap();

    Ok(KubernetesRoleEndpoint {
        path: path.into(),
        role_name: KUB_ACCOUNT_NAME.into(),
        kubernetes_host: nginx_server_addr.into(),
    })
}
