#[macro_use]
extern crate tracing;

mod common;
mod vault_bind_mounts_container;

use std::collections::HashMap;
use std::error::Error as _;
use std::fs;

use dockertest_server::Test;
use rcgen::{BasicConstraints, Certificate, CertificateParams, IsCa};
use tempfile::TempDir;
use test_log::test;
use vault_bind_mounts_container::{VaultServer, VaultServerConfig};
use vaultrs::api::auth::cert::requests::{
    ConfigureTlsCertificateMethodBuilder, CreateCaCertificateRoleRequestBuilder,
};
use vaultrs::auth::cert::{self};
use vaultrs::client::{Client, VaultClient, VaultClientSettingsBuilder};
use vaultrs::error::ClientError;
use vaultrs::sys::auth;

use crate::common::{PORT, VERSION};

#[test]
fn test() {
    let certs = generate_certs();
    let test = new_tls_test(&certs.serialized_cert_dir);
    test.run(|instance| async move {
        let server: VaultServer = instance.server();

        let ca_cert_path = certs
            .serialized_cert_dir
            .path()
            .to_path_buf()
            .join("ca_cert.pem")
            .to_str()
            .unwrap()
            .to_string();
        let client_cert_str = certs
            .client_cert
            .serialize_pem_with_signer(&certs.ca_cert)
            .unwrap();
        let mut data = client_cert_str.as_bytes().to_vec();
        let mut data2 = certs
            .client_cert
            .serialize_private_key_pem()
            .as_bytes()
            .to_vec();
        data.append(&mut data2);
        let identity = reqwest::Identity::from_pem(&data).unwrap();

        let client = match VaultClient::new(
            VaultClientSettingsBuilder::default()
                .address(format!("https://localhost:{PORT}"))
                .token(server.token.clone())
                .ca_certs(vec![ca_cert_path])
                .identity(Some(identity))
                .build()
                .unwrap(),
        ) {
            Ok(c) => c,
            Err(err) => {
                assert!(err
                    .source()
                    .unwrap()
                    .source()
                    .unwrap()
                    .to_string()
                    .eq("incompatible TLS identity type"));
                assert!(cfg!(feature = "native-tls").eq(&true));
                return;
            }
        };
        let endpoint = setup(&client).await.unwrap();

        // Test CA cert role
        ca_cert_role::test_set(&client, &endpoint, client_cert_str.clone()).await;
        ca_cert_role::test_read(&client, &endpoint).await;
        ca_cert_role::test_list(&client, &endpoint).await;

        // Test login
        test_login(&client, &endpoint).await;

        test_configure(&client, &endpoint).await;

        // Test delete
        ca_cert_role::test_delete(&client, &endpoint).await;
    });
}

pub async fn test_login(client: &impl Client, endpoint: &CertEndpoint) {
    let res = cert::login(client, endpoint.path.as_str(), endpoint.name.as_str()).await;
    assert!(res.is_ok());
}

pub async fn test_configure(client: &impl Client, endpoint: &CertEndpoint) {
    cert::configure_tls_certificate_method(
        client,
        endpoint.path.as_str(),
        Some(
            &mut ConfigureTlsCertificateMethodBuilder::default()
                .enable_identity_alias_metadata(true),
        ),
    )
    .await
    .unwrap();
    let login = cert::login(client, endpoint.path.as_str(), endpoint.name.as_str())
        .await
        .unwrap();
    let entity = vaultrs::identity::entity::read_by_id(client, &login.entity_id)
        .await
        .unwrap();
    // FIXME: When we will bump the tested vault to a newer version, we will need to update this assert.
    assert!(entity.metadata.is_none());
}

pub mod ca_cert_role {
    use vaultrs::{auth::cert::ca_cert_role, client::Client};

    use crate::CertEndpoint;

    pub async fn test_delete(client: &impl Client, endpoint: &CertEndpoint) {
        let res =
            ca_cert_role::delete(client, endpoint.path.as_str(), endpoint.name.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_list(client: &impl Client, endpoint: &CertEndpoint) {
        let res = ca_cert_role::list(client, endpoint.path.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_read(client: &impl Client, endpoint: &CertEndpoint) {
        let res = ca_cert_role::read(client, endpoint.path.as_str(), endpoint.name.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_set(client: &impl Client, endpoint: &CertEndpoint, certificate: String) {
        let res = ca_cert_role::set(
            client,
            endpoint.path.as_str(),
            endpoint.name.as_str(),
            certificate.as_str(),
            None,
        )
        .await;
        assert!(res.is_ok());
    }
}

#[derive(Debug)]
pub struct CertEndpoint {
    pub path: String,
    pub name: String,
}

async fn setup(client: &impl Client) -> Result<CertEndpoint, ClientError> {
    debug!("setting up cert auth engine");

    let path = "cert_test";
    let name = "test";

    // Mount the cert auth engine
    auth::enable(client, path, "cert", None).await?;

    Ok(CertEndpoint {
        path: path.to_string(),
        name: name.to_string(),
    })
}

struct Certificates {
    ca_cert: Certificate,
    client_cert: Certificate,
    serialized_cert_dir: TempDir,
}

fn generate_certs() -> Certificates {
    let mut ca_cert_params = CertificateParams::new([]);
    ca_cert_params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    let ca_cert = Certificate::from_params(ca_cert_params).unwrap();

    let client_cert_params = CertificateParams::new([]);
    let client_cert = Certificate::from_params(client_cert_params).unwrap();

    let server_cert_params = CertificateParams::new(["localhost".to_string()]);
    let server_cert = Certificate::from_params(server_cert_params).unwrap();

    // We need to serialize the ca and server certs so that we can mount them within the vault container
    let serialized_cert_dir = tempfile::tempdir().unwrap();

    let ca_cert_path = serialized_cert_dir.path().to_path_buf().join("ca_cert.pem");
    fs::write(ca_cert_path, ca_cert.serialize_pem().unwrap()).unwrap();

    let server_cert_path = serialized_cert_dir
        .path()
        .to_path_buf()
        .join("server_cert.pem");
    fs::write(
        server_cert_path,
        server_cert.serialize_pem_with_signer(&ca_cert).unwrap(),
    )
    .unwrap();

    let server_key_path = serialized_cert_dir
        .path()
        .to_path_buf()
        .join("server_key.pem");
    fs::write(server_key_path, server_cert.serialize_private_key_pem()).unwrap();

    Certificates {
        ca_cert,
        client_cert,
        serialized_cert_dir,
    }
}

fn new_tls_test(server_certs_dir: &TempDir) -> Test {
    let mut test = Test::default();
    let certs_mount_dir = "/etc/vault/certs".to_string();
    let ca_cert_mount_path = format!("{certs_mount_dir}/ca_cert.pem");
    let server_cert_mount_path = format!("{certs_mount_dir}/server_cert.pem");
    let server_key_mount_path = format!("{certs_mount_dir}/server_key.pem");
    let vault_config = HashMap::from([(
        "listener",
        vec![HashMap::from([(
            "tcp",
            HashMap::from([
                ("address", "0.0.0.0:8200".to_string()), // 8200 is hardcoded as internal port in VaultServerConfig::into_composition
                ("tls_cert_file", server_cert_mount_path),
                ("tls_key_file", server_key_mount_path),
                ("tls_client_ca_file", ca_cert_mount_path),
                ("tls_min_version", "tls12".to_string()),
            ]),
        )])],
    )]);

    let env = HashMap::from([
        (
            "VAULT_DEV_LISTEN_ADDRESS".to_string(),
            "0.0.0.0:9999".to_string(), // Setting 9999 to leave 8200 available for the listener configured in VAULT_LOCAL_CONFIG
        ),
        (
            "VAULT_LOCAL_CONFIG".to_string(),
            serde_json::to_string(&vault_config).unwrap(),
        ),
    ]);

    let config = VaultServerConfig::builder()
        .port(PORT)
        .version(VERSION.into())
        .env(env)
        .bind_mounts(HashMap::from([(
            certs_mount_dir,
            server_certs_dir.path().to_str().unwrap().to_string(),
        )]))
        .build()
        .unwrap();
    test.register(config);
    test
}
