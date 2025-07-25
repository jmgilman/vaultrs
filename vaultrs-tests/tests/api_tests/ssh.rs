use tracing::debug;
use vaultrs::api::ssh::requests::SetRoleRequest;
use vaultrs::client::Client;
use vaultrs::sys::mount;

use crate::common::Test;

#[tokio::test]
async fn test() {
    let test = Test::builder().await;
    let client = test.client();
    let endpoint = setup(client).await;

    // Test roles
    role::test_set(client, &endpoint).await;
    role::test_read(client, &endpoint).await;
    role::test_list(client, &endpoint).await;

    // Test zero addresses
    zero::test_set(client, &endpoint).await;
    zero::test_list(client, &endpoint).await;
    zero::test_delete(client, &endpoint).await;

    // Test CA
    ca::test_submit(client, &endpoint).await;
    ca::test_read(client, &endpoint).await;
    ca::test_delete(client, &endpoint).await;
    ca::test_generate(client, &endpoint).await;
    ca::test_sign(client, &endpoint).await;

    // Test generate
    let key = test_generate_otp(client, &endpoint).await;
    test_verify_otp(client, &endpoint, key).await;

    role::test_delete(client, &endpoint).await;
}

pub async fn test_generate_otp(client: &impl Client, endpoint: &SSHEndpoint) -> String {
    vaultrs::ssh::generate(
        client,
        endpoint.path.as_str(),
        endpoint.otp_role.as_str(),
        "192.168.1.1",
        Some("admin".to_string()),
    )
    .await
    .unwrap()
    .key
}

pub async fn test_verify_otp(client: &impl Client, endpoint: &SSHEndpoint, otp: String) {
    vaultrs::ssh::verify_otp(client, endpoint.path.as_str(), otp.as_str())
        .await
        .unwrap();
}

pub mod ca {
    use super::{Client, SSHEndpoint};
    use std::fs;
    use vaultrs::ssh::ca;

    pub async fn test_delete(client: &impl Client, endpoint: &SSHEndpoint) {
        ca::delete(client, endpoint.path.as_str()).await.unwrap();
    }

    pub async fn test_generate(client: &impl Client, endpoint: &SSHEndpoint) {
        ca::generate(client, endpoint.path.as_str()).await.unwrap();
    }

    pub async fn test_read(client: &impl Client, endpoint: &SSHEndpoint) {
        ca::read(client, endpoint.path.as_str()).await.unwrap();
    }

    pub async fn test_sign(client: &impl Client, endpoint: &SSHEndpoint) {
        let public_key = fs::read_to_string("tests/files/id_rsa.pub").unwrap();
        ca::sign(
            client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
            public_key.as_str(),
            None,
        )
        .await
        .unwrap();
    }

    pub async fn test_submit(client: &impl Client, endpoint: &SSHEndpoint) {
        let private_key = fs::read_to_string("tests/files/id_rsa").unwrap();
        let public_key = fs::read_to_string("tests/files/id_rsa.pub").unwrap();
        ca::set(
            client,
            endpoint.path.as_str(),
            private_key.as_str(),
            public_key.as_str(),
        )
        .await
        .unwrap();
    }
}

mod role {
    use super::{Client, SSHEndpoint};
    use vaultrs::{api::ssh::requests::SetRoleRequest, ssh::role};

    pub async fn test_delete(client: &impl Client, endpoint: &SSHEndpoint) {
        role::delete(client, endpoint.path.as_str(), endpoint.role.as_str())
            .await
            .unwrap();
    }

    pub async fn test_list(client: &impl Client, endpoint: &SSHEndpoint) {
        role::list(client, endpoint.path.as_str()).await.unwrap();
    }

    pub async fn test_read(client: &impl Client, endpoint: &SSHEndpoint) {
        role::read(client, endpoint.path.as_str(), endpoint.role.as_str())
            .await
            .unwrap();
    }

    pub async fn test_set(client: &impl Client, endpoint: &SSHEndpoint) {
        role::set(
            client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
            Some(
                &mut SetRoleRequest::builder()
                    .key_type("ca")
                    .allowed_users("*")
                    .allow_user_certificates(true),
            ),
        )
        .await
        .unwrap();
    }
}

pub mod zero {
    use super::{Client, SSHEndpoint};
    use vaultrs::ssh::zero;

    pub async fn test_set(client: &impl Client, endpoint: &SSHEndpoint) {
        zero::set(client, endpoint.path.as_str(), vec![endpoint.role.clone()])
            .await
            .unwrap();
    }

    pub async fn test_list(client: &impl Client, endpoint: &SSHEndpoint) {
        zero::list(client, endpoint.path.as_str()).await.unwrap();
    }

    pub async fn test_delete(client: &impl Client, endpoint: &SSHEndpoint) {
        zero::delete(client, endpoint.path.as_str()).await.unwrap();
    }
}

#[derive(Debug)]
pub struct SSHEndpoint {
    pub path: String,
    pub role: String,
    pub otp_role: String,
}

async fn setup(client: &impl Client) -> SSHEndpoint {
    debug!("setting up SSH auth engine");

    let path = "ssh_test";
    let role = "test";
    let otp_role = "test_otp";

    // Mount the SSH auth engine
    mount::enable(client, path, "ssh", None).await.unwrap();

    // Create OTP role
    vaultrs::ssh::role::set(
        client,
        path,
        otp_role,
        Some(
            &mut SetRoleRequest::builder()
                .key_type("otp")
                .default_user("admin")
                .cidr_list("192.168.0.0/16"),
        ),
    )
    .await
    .unwrap();

    SSHEndpoint {
        path: path.to_string(),
        role: role.to_string(),
        otp_role: otp_role.to_string(),
    }
}
