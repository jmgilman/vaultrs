#[macro_use]
extern crate tracing;

mod common;

use common::VaultServerHelper;
use vaultrs::client::Client;
use vaultrs::{api::ssh::requests::SetRoleRequest, error::ClientError};
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
        let endpoint = setup(&server, &client).await.unwrap();

        // Test roles
        crate::role::test_set(&client, &endpoint).await;
        crate::role::test_read(&client, &endpoint).await;
        crate::role::test_list(&client, &endpoint).await;

        // Test keys
        crate::key::test_set(&client, &endpoint).await;
        crate::key::test_delete(&client, &endpoint).await;

        // Test zero addresses
        crate::zero::test_set(&client, &endpoint).await;
        crate::zero::test_list(&client, &endpoint).await;
        crate::zero::test_delete(&client, &endpoint).await;

        // Test CA
        crate::ca::test_submit(&client, &endpoint).await;
        crate::ca::test_read(&client, &endpoint).await;
        crate::ca::test_delete(&client, &endpoint).await;
        crate::ca::test_generate(&client, &endpoint).await;
        crate::ca::test_sign(&client, &endpoint).await;

        // Test generate
        test_generate_dyn(&client, &endpoint).await;
        let key = test_generate_otp(&client, &endpoint).await;
        test_verify_otp(&client, &endpoint, key).await;

        crate::role::test_delete(&client, &endpoint).await;
    });
}

#[instrument(skip(client))]
pub async fn test_generate_dyn(client: &impl Client, endpoint: &SSHEndpoint) {
    let res = vaultrs::ssh::generate(
        client,
        endpoint.path.as_str(),
        endpoint.dyn_role.as_str(),
        "192.168.1.1",
        Some("admin".to_string()),
    )
    .await;

    // This will fail since we don't have a valid SSH server at the configured IP
    assert!(res.is_err());
    if let ClientError::APIError { code, errors: _ } = res.unwrap_err() {
        assert_eq!(code, 500);
    }
}

#[instrument(skip(client))]
pub async fn test_generate_otp(client: &impl Client, endpoint: &SSHEndpoint) -> String {
    let res = vaultrs::ssh::generate(
        client,
        endpoint.path.as_str(),
        endpoint.otp_role.as_str(),
        "192.168.1.1",
        Some("admin".to_string()),
    )
    .await;

    assert!(res.is_ok());
    res.unwrap().key
}

#[instrument(skip(client))]
pub async fn test_verify_otp(client: &impl Client, endpoint: &SSHEndpoint, otp: String) {
    let res = vaultrs::ssh::verify_otp(client, endpoint.path.as_str(), otp.as_str()).await;
    assert!(res.is_ok());
}

pub mod ca {
    use super::{Client, SSHEndpoint};
    use std::fs;
    use vaultrs::ssh::ca;

    #[instrument(skip(client))]
    pub async fn test_delete(client: &impl Client, endpoint: &SSHEndpoint) {
        let res = ca::delete(client, endpoint.path.as_str()).await;
        assert!(res.is_ok());
    }

    #[instrument(skip(client))]
    pub async fn test_generate(client: &impl Client, endpoint: &SSHEndpoint) {
        let res = ca::generate(client, endpoint.path.as_str()).await;
        assert!(res.is_ok());
    }

    #[instrument(skip(client))]
    pub async fn test_read(client: &impl Client, endpoint: &SSHEndpoint) {
        let res = ca::read(client, endpoint.path.as_str()).await;
        assert!(res.is_ok());
    }

    #[instrument(skip(client))]
    pub async fn test_sign(client: &impl Client, endpoint: &SSHEndpoint) {
        let public_key = fs::read_to_string("tests/files/id_rsa.pub").unwrap();
        let res = ca::sign(
            client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
            public_key.as_str(),
            None,
        )
        .await;
        assert!(res.is_ok());
    }

    #[instrument(skip(client))]
    pub async fn test_submit(client: &impl Client, endpoint: &SSHEndpoint) {
        let private_key = fs::read_to_string("tests/files/id_rsa").unwrap();
        let public_key = fs::read_to_string("tests/files/id_rsa.pub").unwrap();
        let res = ca::set(
            client,
            endpoint.path.as_str(),
            private_key.as_str(),
            public_key.as_str(),
        )
        .await;
        assert!(res.is_ok());
    }
}

pub mod key {
    use super::{Client, SSHEndpoint};
    use std::fs;
    use vaultrs::ssh::key;

    #[instrument(skip(client))]
    pub async fn test_set(client: &impl Client, endpoint: &SSHEndpoint) {
        let key = fs::read_to_string("tests/files/id_rsa").unwrap();
        let res = key::set(
            client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
            key.as_str(),
        )
        .await;
        assert!(res.is_ok());
    }

    #[instrument(skip(client))]
    pub async fn test_delete(client: &impl Client, endpoint: &SSHEndpoint) {
        let res = key::delete(client, endpoint.path.as_str(), endpoint.role.as_str()).await;
        assert!(res.is_ok());
    }
}

mod role {
    use super::{Client, SSHEndpoint};
    use vaultrs::{api::ssh::requests::SetRoleRequest, ssh::role};

    #[instrument(skip(client))]
    pub async fn test_delete(client: &impl Client, endpoint: &SSHEndpoint) {
        let res = role::delete(client, endpoint.path.as_str(), endpoint.role.as_str()).await;
        assert!(res.is_ok());
    }

    #[instrument(skip(client))]
    pub async fn test_list(client: &impl Client, endpoint: &SSHEndpoint) {
        let res = role::list(client, endpoint.path.as_str()).await;
        assert!(res.is_ok());
    }

    #[instrument(skip(client))]
    pub async fn test_read(client: &impl Client, endpoint: &SSHEndpoint) {
        let res = role::read(client, endpoint.path.as_str(), endpoint.role.as_str()).await;
        assert!(res.is_ok());
    }

    #[instrument(skip(client))]
    pub async fn test_set(client: &impl Client, endpoint: &SSHEndpoint) {
        let res = role::set(
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
        .await;
        assert!(res.is_ok());
    }
}

pub mod zero {
    use super::{Client, SSHEndpoint};
    use vaultrs::ssh::zero;

    #[instrument(skip(client))]
    pub async fn test_set(client: &impl Client, endpoint: &SSHEndpoint) {
        let res = zero::set(client, endpoint.path.as_str(), vec![endpoint.role.clone()]).await;
        assert!(res.is_ok());
    }

    #[instrument(skip(client))]
    pub async fn test_list(client: &impl Client, endpoint: &SSHEndpoint) {
        let res = zero::list(client, endpoint.path.as_str()).await;
        assert!(res.is_ok());
    }

    #[instrument(skip(client))]
    pub async fn test_delete(client: &impl Client, endpoint: &SSHEndpoint) {
        let res = zero::delete(client, endpoint.path.as_str()).await;
        assert!(res.is_ok());
    }
}

#[derive(Debug)]
pub struct SSHEndpoint {
    pub path: String,
    pub role: String,
    pub dyn_role: String,
    pub otp_role: String,
}

async fn setup(server: &VaultServer, client: &impl Client) -> Result<SSHEndpoint, ClientError> {
    debug!("setting up SSH auth engine");

    let path = "ssh_test";
    let role = "test";
    let dyn_role = "test_dyn";
    let otp_role = "test_otp";

    // Mount the SSH auth engine
    server.mount_secret(client, path, "ssh").await?;

    // Create key
    let key = std::fs::read_to_string("tests/files/id_rsa").unwrap();
    vaultrs::ssh::key::set(client, path, role, key.as_str()).await?;

    // Create dynamic role
    vaultrs::ssh::role::set(
        client,
        path,
        dyn_role,
        Some(
            &mut SetRoleRequest::builder()
                .key_type("dynamic")
                .key(role)
                .admin_user("admin")
                .default_user("admin")
                .cidr_list("192.168.0.0/16"),
        ),
    )
    .await?;

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
    .await?;

    Ok(SSHEndpoint {
        path: path.to_string(),
        role: role.to_string(),
        dyn_role: dyn_role.to_string(),
        otp_role: otp_role.to_string(),
    })
}
