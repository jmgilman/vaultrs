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
    cert::ca::int::test_cross_sign(client, &endpoint).await;

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

    // Test issuers
    issuer::test_read(client, &endpoint).await;
    issuer::test_sign_intermediate(client, &endpoint).await;
    issuer::test_import(client, &endpoint).await;
    issuer::test_set_default(client, &endpoint).await;

    // Test intermediate issuers
    issuer::int::test_generate(client, &endpoint).await;
}

mod cert {
    use vaultrs::api::pki::requests::GenerateCertificateRequest;
    use vaultrs::pki::cert;

    use super::{Client, PKIEndpoint};

    pub async fn test_generate(client: &impl Client, endpoint: &PKIEndpoint) {
        assert!(!cert::generate(
            client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
            Some(GenerateCertificateRequest::builder().common_name("test.com")),
        )
        .await
        .unwrap()
        .certificate
        .is_empty());
    }

    pub async fn test_list(client: &impl Client, endpoint: &PKIEndpoint) {
        assert!(!cert::list(client, endpoint.path.as_str())
            .await
            .unwrap()
            .is_empty());
    }

    pub async fn test_read(client: &impl Client, endpoint: &PKIEndpoint) {
        let certs = cert::list(client, endpoint.path.as_str()).await.unwrap();

        assert!(
            !cert::read(client, endpoint.path.as_str(), certs[0].as_str())
                .await
                .unwrap()
                .certificate
                .is_empty()
        );
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

        assert!(
            cert::revoke(client, endpoint.path.as_str(), cert.serial_number.as_str())
                .await
                .unwrap()
                .revocation_time
                > 0
        );
    }

    pub async fn test_tidy(client: &impl Client, endpoint: &PKIEndpoint) {
        cert::tidy(client, endpoint.path.as_str()).await.unwrap();
    }

    pub mod ca {
        use std::fs;

        use super::{Client, PKIEndpoint};
        use vaultrs::{api::pki::requests::GenerateRootRequest, pki::cert::ca};

        pub async fn test_delete(client: &impl Client, endpoint: &PKIEndpoint) {
            ca::delete(client, endpoint.path.as_str()).await.unwrap();
        }

        pub async fn test_generate(client: &impl Client, endpoint: &PKIEndpoint) {
            assert!(ca::generate(
                client,
                endpoint.path.as_str(),
                "internal",
                Some(
                    GenerateRootRequest::builder()
                        .common_name("Test")
                        .ttl("87600h"),
                ),
            )
            .await
            .unwrap()
            .is_some());
        }

        pub async fn test_sign(client: &impl Client, endpoint: &PKIEndpoint) {
            let csr = fs::read_to_string("tests/files/csr.pem").unwrap();

            assert!(!ca::sign(
                client,
                endpoint.path.as_str(),
                endpoint.role.as_str(),
                csr.as_str(),
                "test.com",
                None,
            )
            .await
            .unwrap()
            .certificate
            .is_empty());
        }

        pub async fn test_sign_intermediate(client: &impl Client, endpoint: &PKIEndpoint) {
            let csr = fs::read_to_string("tests/files/csr.pem").unwrap();

            assert!(!ca::sign_intermediate(
                client,
                endpoint.path.as_str(),
                csr.as_str(),
                "test.com",
                None,
            )
            .await
            .unwrap()
            .certificate
            .is_empty());
        }

        pub async fn test_sign_self_issued(client: &impl Client, endpoint: &PKIEndpoint) {
            let cert = fs::read_to_string("tests/files/root_ca.crt").unwrap();

            assert!(
                !ca::sign_self_issued(client, endpoint.path.as_str(), cert.as_str())
                    .await
                    .unwrap()
                    .certificate
                    .is_empty()
            );
        }

        pub async fn test_submit(client: &impl Client, endpoint: &PKIEndpoint) {
            let bundle = fs::read_to_string("tests/files/ca.pem").unwrap();

            ca::delete(client, endpoint.path.as_str()).await.unwrap();

            ca::submit(client, endpoint.path.as_str(), bundle.as_str())
                .await
                .unwrap();
        }

        pub mod int {
            use super::{Client, PKIEndpoint};
            use vaultrs::pki::cert::ca;
            use vaultrs::pki::cert::ca::int;
            use vaultrs::sys::mount;

            pub async fn test_generate(client: &impl Client, _: &PKIEndpoint) {
                mount::enable(client, "pki_int", "pki", None).await.unwrap();

                assert!(
                    !int::generate(client, "pki_int", "internal", "test-int.com", None)
                        .await
                        .unwrap()
                        .csr
                        .is_empty()
                );
            }

            pub async fn test_set_signed(client: &impl Client, endpoint: &PKIEndpoint) {
                let csr = int::generate(client, "pki_int", "internal", "test-int.com", None)
                    .await
                    .unwrap()
                    .csr;

                let certificate = ca::sign_intermediate(
                    client,
                    endpoint.path.as_str(),
                    &csr,
                    "test-int.com",
                    None,
                )
                .await
                .unwrap()
                .certificate;

                int::set_signed(client, "pki_int", &certificate)
                    .await
                    .unwrap();
            }

            pub async fn test_cross_sign(client: &impl Client, endpoint: &PKIEndpoint) {
                assert!(!int::cross_sign(client, endpoint.path.as_str(), None)
                    .await
                    .unwrap()
                    .csr
                    .is_empty());
            }
        }
    }

    pub mod crl {
        use super::{Client, PKIEndpoint};
        use vaultrs::{api::pki::requests::SetCRLConfigRequest, pki::cert::crl};

        pub async fn test_rotate(client: &impl Client, endpoint: &PKIEndpoint) {
            assert!(
                crl::rotate(client, endpoint.path.as_str())
                    .await
                    .unwrap()
                    .success
            );
        }

        pub async fn test_read_config(client: &impl Client, endpoint: &PKIEndpoint) {
            crl::set_config(
                client,
                endpoint.path.as_str(),
                Some(SetCRLConfigRequest::builder().expiry("72h").disable(false)),
            )
            .await
            .unwrap();

            assert!(
                !crl::read_config(client, endpoint.path.as_str())
                    .await
                    .unwrap()
                    .disable
            );
        }

        pub async fn test_set_config(client: &impl Client, endpoint: &PKIEndpoint) {
            crl::set_config(
                client,
                endpoint.path.as_str(),
                Some(SetCRLConfigRequest::builder().expiry("72h").disable(false)),
            )
            .await
            .unwrap();
        }
    }

    pub mod urls {
        use super::{Client, PKIEndpoint};
        use vaultrs::{api::pki::requests::SetURLsRequest, pki::cert::urls};

        pub async fn test_read(client: &impl Client, endpoint: &PKIEndpoint) {
            assert!(!urls::read(client, endpoint.path.as_str())
                .await
                .unwrap()
                .issuing_certificates
                .is_empty());
        }

        pub async fn test_set(client: &impl Client, endpoint: &PKIEndpoint) {
            let issue = format!("{}/v1/{}/ca", client.http().base, endpoint.path);
            let dist = format!("{}/v1/{}/crl", client.http().base, endpoint.path);

            urls::set(
                client,
                endpoint.path.as_str(),
                Some(
                    SetURLsRequest::builder()
                        .issuing_certificates(vec![issue])
                        .crl_distribution_points(vec![dist]),
                ),
            )
            .await
            .unwrap();
        }
    }
}

mod issuer {
    use std::fs;

    use super::{Client, PKIEndpoint};
    use vaultrs::{
        api::pki::responses::ImportIssuerResponse,
        pki::{issuer, key},
    };

    pub async fn test_read(client: &impl Client, endpoint: &PKIEndpoint) {
        let resp = issuer::read(client, endpoint.path.as_str(), None)
            .await
            .unwrap();
        let certificate = resp.certificate;
        let ca_chain = resp.ca_chain;
        assert!(!certificate.is_empty());
        assert_eq!(ca_chain.len(), 1);
        assert_eq!(ca_chain[0], certificate);
    }

    pub async fn test_sign_intermediate(client: &impl Client, endpoint: &PKIEndpoint) {
        let csr = fs::read_to_string("tests/files/csr.pem").unwrap();

        assert!(!issuer::sign_intermediate(
            client,
            endpoint.path.as_str(),
            csr.as_str(),
            "test.com",
            None,
            None,
        )
        .await
        .unwrap()
        .certificate
        .is_empty());
    }

    pub async fn test_import(client: &impl Client, endpoint: &PKIEndpoint) {
        let bundle = fs::read_to_string("tests/files/ca.pem").unwrap();

        let ImportIssuerResponse {
            imported_issuers,
            imported_keys,
            existing_issuers,
            existing_keys,
            mapping,
        } = issuer::import(client, endpoint.path.as_str(), bundle.as_str())
            .await
            .unwrap();

        assert!(existing_issuers.is_none());
        assert!(existing_keys.is_none());
        assert!(mapping.is_some());
        assert_eq!(mapping.unwrap().len(), 1);

        assert_eq!(imported_issuers.as_ref().unwrap().len(), 1);
        let imported_issuer = &imported_issuers.unwrap()[0];
        assert_eq!(imported_keys.as_ref().unwrap().len(), 1);
        let imported_key = &imported_keys.unwrap()[0];

        // attempt to import the same CA twice should return existing issuer

        let ImportIssuerResponse {
            imported_issuers,
            imported_keys,
            existing_issuers,
            existing_keys,
            mapping,
        } = issuer::import(client, endpoint.path.as_str(), bundle.as_str())
            .await
            .unwrap();

        assert!(imported_issuers.is_none());
        assert!(imported_keys.is_none());
        assert_eq!(mapping.unwrap().len(), 1);

        assert_eq!(existing_issuers.as_ref().unwrap().len(), 1);
        assert_eq!(&existing_issuers.unwrap()[0], imported_issuer);
        assert_eq!(existing_keys.as_ref().unwrap().len(), 1);
        assert_eq!(&existing_keys.unwrap()[0], imported_key);

        // remove imported issuer
        issuer::delete(client, endpoint.path.as_str(), imported_issuer)
            .await
            .unwrap();
        key::delete(client, endpoint.path.as_str(), imported_key)
            .await
            .unwrap();
    }

    pub async fn test_set_default(client: &impl Client, endpoint: &PKIEndpoint) {
        let endpoint = &endpoint.path;

        // get existing CA
        let resp = issuer::read(client, endpoint, None).await.unwrap();
        let old_issuer_cert = resp.certificate;
        let old_issuer_id = resp.issuer_id;

        // import new CA
        let bundle = fs::read_to_string("tests/files/ca.pem").unwrap();
        let resp = issuer::import(client, endpoint, &bundle).await.unwrap();
        assert_eq!(resp.imported_issuers.as_ref().unwrap().len(), 1);
        let new_issuer_id = &resp.imported_issuers.unwrap()[0];

        // import new CA should not affect default issuer
        let resp = issuer::read(client, endpoint, None).await.unwrap();
        assert_eq!(resp.certificate, old_issuer_cert);
        assert_eq!(resp.issuer_id, old_issuer_id);

        // set imported CA as a default
        let resp = issuer::set_default(client, endpoint, new_issuer_id)
            .await
            .unwrap();
        assert_eq!(&resp.default, new_issuer_id);

        // imported CA is a default issuer
        let resp = issuer::read(client, endpoint, None).await.unwrap();
        assert_eq!(&resp.issuer_id, new_issuer_id);

        // restore default issuer
        let resp = issuer::set_default(client, endpoint, &old_issuer_id)
            .await
            .unwrap();
        assert_eq!(resp.default, old_issuer_id);

        let resp = issuer::read(client, endpoint, None).await.unwrap();
        assert_eq!(resp.certificate, old_issuer_cert);
        assert_eq!(resp.issuer_id, old_issuer_id);

        // remove imported issuer
        issuer::delete(client, endpoint, new_issuer_id)
            .await
            .unwrap();
    }

    pub mod int {
        use super::{Client, PKIEndpoint};
        use vaultrs::pki::issuer;

        pub async fn test_generate(client: &impl Client, endpoint: &PKIEndpoint) {
            for request_type in ["internal", "exported", "existing"] {
                let resp = issuer::int::generate(
                    client,
                    endpoint.path.as_str(),
                    request_type,
                    "test-int.com",
                    None,
                )
                .await;

                assert!(resp.is_ok());
                assert!(!resp.unwrap().csr.is_empty());
            }
        }
    }
}

mod role {
    use super::{Client, PKIEndpoint};
    use vaultrs::{api::pki::requests::SetRoleRequest, pki::role};

    pub async fn test_delete(client: &impl Client, endpoint: &PKIEndpoint) {
        role::delete(client, endpoint.path.as_str(), endpoint.role.as_str())
            .await
            .unwrap();
    }

    pub async fn test_list(client: &impl Client, endpoint: &PKIEndpoint) {
        assert!(!role::list(client, endpoint.path.as_str())
            .await
            .unwrap()
            .keys
            .is_empty());
    }

    pub async fn test_read(client: &impl Client, endpoint: &PKIEndpoint) {
        assert!(
            role::read(client, endpoint.path.as_str(), endpoint.role.as_str())
                .await
                .unwrap()
                .allow_any_name
        );
    }

    pub async fn test_set(client: &impl Client, endpoint: &PKIEndpoint) {
        role::set(
            client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
            Some(SetRoleRequest::builder().allow_any_name(true)),
        )
        .await
        .unwrap();
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
