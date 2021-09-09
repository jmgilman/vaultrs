pub const VERSION: &str = "1.8.2";

use vaultrs::{api::ssh::requests::SetRoleRequest, error::ClientError};
use vaultrs_test::docker::{Server, ServerConfig};
use vaultrs_test::{VaultServer, VaultServerConfig};

#[test]
fn test() {
    let config = VaultServerConfig::default(Some(VERSION));
    let instance = config.to_instance();

    instance.run(|ops| async move {
        let server = VaultServer::new(&ops, &config);
        let endpoint = setup(&server).await.unwrap();

        // Test roles
        crate::role::test_set(&server, &endpoint).await;
        crate::role::test_read(&server, &endpoint).await;
        crate::role::test_list(&server, &endpoint).await;

        // Test keys
        crate::key::test_set(&server, &endpoint).await;
        crate::key::test_delete(&server, &endpoint).await;

        // Test zero addresses
        crate::zero::test_set(&server, &endpoint).await;
        crate::zero::test_list(&server, &endpoint).await;
        crate::zero::test_delete(&server, &endpoint).await;

        // Test CA
        crate::ca::test_submit(&server, &endpoint).await;
        crate::ca::test_read(&server, &endpoint).await;
        crate::ca::test_delete(&server, &endpoint).await;
        crate::ca::test_generate(&server, &endpoint).await;
        crate::ca::test_sign(&server, &endpoint).await;

        // Test generate
        test_generate_dyn(&server, &endpoint).await;
        let key = test_generate_otp(&server, &endpoint).await;
        test_verify_otp(&server, &endpoint, key).await;

        crate::role::test_delete(&server, &endpoint).await;
    });
}

pub async fn test_generate_dyn(server: &VaultServer, endpoint: &SSHEndpoint) {
    let res = vaultrs::ssh::generate(
        &server.client,
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

pub async fn test_generate_otp(server: &VaultServer, endpoint: &SSHEndpoint) -> String {
    let res = vaultrs::ssh::generate(
        &server.client,
        endpoint.path.as_str(),
        endpoint.otp_role.as_str(),
        "192.168.1.1",
        Some("admin".to_string()),
    )
    .await;

    assert!(res.is_ok());
    res.unwrap().key
}

pub async fn test_verify_otp(server: &VaultServer, endpoint: &SSHEndpoint, otp: String) {
    let res = vaultrs::ssh::verify_otp(&server.client, endpoint.path.as_str(), otp.as_str()).await;
    assert!(res.is_ok());
}

pub mod ca {
    use super::{SSHEndpoint, VaultServer};
    use std::fs;
    use vaultrs::ssh::ca;

    pub async fn test_delete(server: &VaultServer, endpoint: &SSHEndpoint) {
        let res = ca::delete(&server.client, endpoint.path.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_generate(server: &VaultServer, endpoint: &SSHEndpoint) {
        let res = ca::generate(&server.client, endpoint.path.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_read(server: &VaultServer, endpoint: &SSHEndpoint) {
        let res = ca::read(&server.client, endpoint.path.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_sign(server: &VaultServer, endpoint: &SSHEndpoint) {
        let public_key = fs::read_to_string("tests/files/id_rsa.pub").unwrap();
        let res = ca::sign(
            &server.client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
            public_key.as_str(),
            None,
        )
        .await;
        assert!(res.is_ok());
    }

    pub async fn test_submit(server: &VaultServer, endpoint: &SSHEndpoint) {
        let private_key = fs::read_to_string("tests/files/id_rsa").unwrap();
        let public_key = fs::read_to_string("tests/files/id_rsa.pub").unwrap();
        let res = ca::set(
            &server.client,
            endpoint.path.as_str(),
            private_key.as_str(),
            public_key.as_str(),
        )
        .await;
        assert!(res.is_ok());
    }
}

pub mod key {
    use super::{SSHEndpoint, VaultServer};
    use std::fs;
    use vaultrs::ssh::key;

    pub async fn test_set(server: &VaultServer, endpoint: &SSHEndpoint) {
        let key = fs::read_to_string("tests/files/id_rsa").unwrap();
        let res = key::set(
            &server.client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
            key.as_str(),
        )
        .await;
        assert!(res.is_ok());
    }

    pub async fn test_delete(server: &VaultServer, endpoint: &SSHEndpoint) {
        let res = key::delete(
            &server.client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
        )
        .await;
        assert!(res.is_ok());
    }
}

mod role {
    use super::{SSHEndpoint, VaultServer};
    use vaultrs::{api::ssh::requests::SetRoleRequest, ssh::role};

    pub async fn test_delete(server: &VaultServer, endpoint: &SSHEndpoint) {
        let res = role::delete(
            &server.client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
        )
        .await;
        assert!(res.is_ok());
    }

    pub async fn test_list(server: &VaultServer, endpoint: &SSHEndpoint) {
        let res = role::list(&server.client, endpoint.path.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_read(server: &VaultServer, endpoint: &SSHEndpoint) {
        let res = role::read(
            &server.client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
        )
        .await;
        assert!(res.is_ok());
    }

    pub async fn test_set(server: &VaultServer, endpoint: &SSHEndpoint) {
        let res = role::set(
            &server.client,
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
    use super::{SSHEndpoint, VaultServer};
    use vaultrs::ssh::zero;

    pub async fn test_set(server: &VaultServer, endpoint: &SSHEndpoint) {
        let res = zero::set(
            &server.client,
            endpoint.path.as_str(),
            vec![endpoint.role.clone()],
        )
        .await;
        assert!(res.is_ok());
    }

    pub async fn test_list(server: &VaultServer, endpoint: &SSHEndpoint) {
        let res = zero::list(&server.client, endpoint.path.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_delete(server: &VaultServer, endpoint: &SSHEndpoint) {
        let res = zero::delete(&server.client, endpoint.path.as_str()).await;
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

async fn setup(server: &VaultServer) -> Result<SSHEndpoint, ClientError> {
    let path = "ssh_test";
    let role = "test";
    let dyn_role = "test_dyn";
    let otp_role = "test_otp";

    // Mount the OIDC auth engine
    server.mount_secret(path, "ssh").await?;

    // Create key
    let key = std::fs::read_to_string("tests/files/id_rsa").unwrap();
    vaultrs::ssh::key::set(&server.client, path, role, key.as_str()).await?;

    // Create dynamic role
    vaultrs::ssh::role::set(
        &server.client,
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
        &server.client,
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
