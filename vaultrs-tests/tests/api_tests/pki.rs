use crate::common::Test;
use tracing::debug;
use vaultrs::api::sys::requests::{EnableEngineDataConfigBuilder, EnableEngineRequest};
use vaultrs::client::Client;
use vaultrs::error::ClientError;
use vaultrs::sys::mount;

#[tokio::test]
async fn test() {
    let test = Test::builder().await;
    let client = test.client();
    let endpoint = setup(client).await.unwrap();

    // Test roles
    role::test_set(client, &endpoint).await;
    role::test_read(client, &endpoint).await;
    role::test_list(client, &endpoint).await;
    role::test_delete(client, &endpoint).await;

    // Test CA
    role::test_set(client, &endpoint).await;
    cert::ca::test_generate(client, &endpoint).await;
    cert::ca::test_sign(client, &endpoint).await;
    cert::ca::test_sign_intermediate(client, &endpoint).await;

    cert::ca::test_sign_self_issued(client, &endpoint).await;
    cert::ca::test_delete(client, &endpoint).await;
    cert::ca::test_submit(client, &endpoint).await;
    cert::ca::test_delete(client, &endpoint).await;
    cert::ca::test_generate(client, &endpoint).await;

    // Test intermediate CA
    cert::ca::int::test_generate(client, &endpoint).await;
    cert::ca::int::test_set_signed(client, &endpoint).await;

    // Test certs
    cert::test_generate(client, &endpoint).await;
    cert::test_read(client, &endpoint).await;
    cert::test_list(client, &endpoint).await;
    cert::test_revoke(client, &endpoint).await;
    cert::test_tidy(client, &endpoint).await;

    // Test CRLs
    cert::crl::test_set_config(client, &endpoint).await;
    cert::crl::test_read_config(client, &endpoint).await;
    cert::crl::test_rotate(client, &endpoint).await;

    // Test URLs
    cert::urls::test_set(client, &endpoint).await;
    cert::urls::test_read(client, &endpoint).await;
}

mod cert {
    use vaultrs::api::pki::requests::GenerateCertificateRequest;
    use vaultrs::pki::cert;

    use super::{Client, PKIEndpoint};

    pub async fn test_generate(client: &impl Client, endpoint: &PKIEndpoint) {
        let resp = cert::generate(
            client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
            Some(GenerateCertificateRequest::builder().common_name("test.com")),
        )
        .await;
        assert!(resp.is_ok());
        assert!(!resp.unwrap().certificate.is_empty())
    }

    pub async fn test_list(client: &impl Client, endpoint: &PKIEndpoint) {
        let res = cert::list(client, endpoint.path.as_str()).await;
        assert!(res.is_ok());
        assert!(!res.unwrap().is_empty());
    }

    pub async fn test_read(client: &impl Client, endpoint: &PKIEndpoint) {
        let certs = cert::list(client, endpoint.path.as_str()).await.unwrap();

        let resp = cert::read(client, endpoint.path.as_str(), certs[0].as_str())
            .await
            .unwrap();
        assert!(!resp.certificate.is_empty());
    }

    pub async fn test_revoke(client: &impl Client, endpoint: &PKIEndpoint) {
        let cert = cert::generate(
            client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
            Some(GenerateCertificateRequest::builder().common_name("test.com")),
        )
        .await
        .unwrap();

        let resp = cert::revoke(client, endpoint.path.as_str(), cert.serial_number.as_str()).await;
        assert!(resp.is_ok());
        assert!(resp.unwrap().revocation_time > 0);
    }

    pub async fn test_tidy(client: &impl Client, endpoint: &PKIEndpoint) {
        let resp = cert::tidy(client, endpoint.path.as_str()).await;
        assert!(resp.is_ok());
    }

    pub mod ca {
        use std::fs;

        use super::{Client, PKIEndpoint};
        use vaultrs::{api::pki::requests::GenerateRootRequest, pki::cert::ca};

        pub async fn test_delete(client: &impl Client, endpoint: &PKIEndpoint) {
            let resp = ca::delete(client, endpoint.path.as_str()).await;
            assert!(resp.is_ok());
        }

        pub async fn test_generate(client: &impl Client, endpoint: &PKIEndpoint) {
            let resp = ca::generate(
                client,
                endpoint.path.as_str(),
                "internal",
                Some(
                    GenerateRootRequest::builder()
                        .common_name("Test")
                        .ttl("87600h"),
                ),
            )
            .await;

            assert!(resp.is_ok());
            assert!(resp.unwrap().is_some());
        }

        pub async fn test_sign(client: &impl Client, endpoint: &PKIEndpoint) {
            let csr = fs::read_to_string("tests/files/csr.pem").unwrap();

            let resp = ca::sign(
                client,
                endpoint.path.as_str(),
                endpoint.role.as_str(),
                csr.as_str(),
                "test.com",
                None,
            )
            .await;

            assert!(resp.is_ok());
            assert!(!resp.unwrap().certificate.is_empty());
        }

        pub async fn test_sign_intermediate(client: &impl Client, endpoint: &PKIEndpoint) {
            let csr = fs::read_to_string("tests/files/csr.pem").unwrap();

            let resp = ca::sign_intermediate(
                client,
                endpoint.path.as_str(),
                csr.as_str(),
                "test.com",
                None,
            )
            .await;

            assert!(resp.is_ok());
            assert!(!resp.unwrap().certificate.is_empty());
        }

        pub async fn test_sign_self_issued(client: &impl Client, endpoint: &PKIEndpoint) {
            let cert = fs::read_to_string("tests/files/root_ca.crt").unwrap();

            let resp = ca::sign_self_issued(client, endpoint.path.as_str(), cert.as_str()).await;

            assert!(resp.is_ok());
            assert!(!resp.unwrap().certificate.is_empty());
        }

        pub async fn test_submit(client: &impl Client, endpoint: &PKIEndpoint) {
            let bundle = fs::read_to_string("tests/files/ca.pem").unwrap();

            let resp = ca::delete(client, endpoint.path.as_str()).await;
            assert!(resp.is_ok());

            let resp = ca::submit(client, endpoint.path.as_str(), bundle.as_str()).await;
            assert!(resp.is_ok());
        }

        pub mod int {
            use super::{Client, PKIEndpoint};
            use vaultrs::pki::cert::ca;
            use vaultrs::pki::cert::ca::int;
            use vaultrs::sys::mount;

            pub async fn test_generate(client: &impl Client, _: &PKIEndpoint) {
                mount::enable(client, "pki_int", "pki", None).await.unwrap();

                let resp = int::generate(client, "pki_int", "internal", "test-int.com", None).await;

                assert!(resp.is_ok());
                assert!(!resp.unwrap().csr.is_empty());
            }

            pub async fn test_set_signed(client: &impl Client, endpoint: &PKIEndpoint) {
                let resp = int::generate(client, "pki_int", "internal", "test-int.com", None).await;
                assert!(resp.is_ok());

                let resp = ca::sign_intermediate(
                    client,
                    endpoint.path.as_str(),
                    resp.unwrap().csr.as_str(),
                    "test-int.com",
                    None,
                )
                .await;
                assert!(resp.is_ok());

                let resp =
                    int::set_signed(client, "pki_int", resp.unwrap().certificate.as_str()).await;
                assert!(resp.is_ok());
            }
        }
    }

    pub mod crl {
        use super::{Client, PKIEndpoint};
        use vaultrs::{api::pki::requests::SetCRLConfigRequest, pki::cert::crl};

        pub async fn test_rotate(client: &impl Client, endpoint: &PKIEndpoint) {
            let res = crl::rotate(client, endpoint.path.as_str()).await;
            assert!(res.is_ok());
            assert!(res.unwrap().success);
        }

        pub async fn test_read_config(client: &impl Client, endpoint: &PKIEndpoint) {
            let res = crl::set_config(
                client,
                endpoint.path.as_str(),
                Some(SetCRLConfigRequest::builder().expiry("72h").disable(false)),
            )
            .await;
            assert!(res.is_ok());

            let res = crl::read_config(client, endpoint.path.as_str()).await;
            assert!(res.is_ok());
            assert!(!res.unwrap().disable);
        }

        pub async fn test_set_config(client: &impl Client, endpoint: &PKIEndpoint) {
            let res = crl::set_config(
                client,
                endpoint.path.as_str(),
                Some(SetCRLConfigRequest::builder().expiry("72h").disable(false)),
            )
            .await;
            assert!(res.is_ok());
        }
    }

    pub mod urls {
        use super::{Client, PKIEndpoint};
        use vaultrs::{api::pki::requests::SetURLsRequest, pki::cert::urls};

        pub async fn test_read(client: &impl Client, endpoint: &PKIEndpoint) {
            let res = urls::read(client, endpoint.path.as_str()).await;
            assert!(res.is_ok());
            assert!(!res.unwrap().issuing_certificates.is_empty())
        }

        pub async fn test_set(client: &impl Client, endpoint: &PKIEndpoint) {
            let issue = format!("{}/v1/{}/ca", client.http().base, endpoint.path);
            let dist = format!("{}/v1/{}/crl", client.http().base, endpoint.path);

            let res = urls::set(
                client,
                endpoint.path.as_str(),
                Some(
                    SetURLsRequest::builder()
                        .issuing_certificates(vec![issue])
                        .crl_distribution_points(vec![dist]),
                ),
            )
            .await;
            assert!(res.is_ok());
        }
    }
}

mod role {
    use super::{Client, PKIEndpoint};
    use vaultrs::{api::pki::requests::SetRoleRequest, pki::role};

    pub async fn test_delete(client: &impl Client, endpoint: &PKIEndpoint) {
        let res = role::delete(client, endpoint.path.as_str(), endpoint.role.as_str()).await;
        assert!(res.is_ok());
    }

    pub async fn test_list(client: &impl Client, endpoint: &PKIEndpoint) {
        let res = role::list(client, endpoint.path.as_str()).await;
        assert!(res.is_ok());
        assert!(!res.unwrap().keys.is_empty());
    }

    pub async fn test_read(client: &impl Client, endpoint: &PKIEndpoint) {
        let res = role::read(client, endpoint.path.as_str(), endpoint.role.as_str()).await;
        assert!(res.is_ok());
        assert!(res.unwrap().allow_any_name)
    }

    pub async fn test_set(client: &impl Client, endpoint: &PKIEndpoint) {
        let res = role::set(
            client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
            Some(SetRoleRequest::builder().allow_any_name(true)),
        )
        .await;
        assert!(res.is_ok());
    }
}

#[derive(Debug)]
pub struct PKIEndpoint {
    pub path: String,
    pub role: String,
}

async fn setup(client: &impl Client) -> Result<PKIEndpoint, ClientError> {
    debug!("setting up PKI auth engine");

    let path = "pki_test";
    let role = "test";

    // Mount the PKI engine
    let config = EnableEngineDataConfigBuilder::default()
        .max_lease_ttl("87600h")
        .build()
        .unwrap();
    mount::enable(
        client,
        path,
        "pki",
        Some(EnableEngineRequest::builder().config(config)),
    )
    .await
    .unwrap();

    Ok(PKIEndpoint {
        path: path.to_string(),
        role: role.to_string(),
    })
}