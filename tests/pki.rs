mod common;

use common::VaultServer;
use vaultrs::api::sys::requests::EnableEngineDataConfigBuilder;
use vaultrs::error::ClientError;

#[test]
fn test() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);
    let endpoint = setup(&server).unwrap();

    // Test roles
    crate::role::test_set(&server, &endpoint);
    crate::role::test_read(&server, &endpoint);
    crate::role::test_list(&server, &endpoint);
    crate::role::test_delete(&server, &endpoint);

    // Test CA
    crate::role::test_set(&server, &endpoint);
    crate::cert::ca::test_generate(&server, &endpoint);
    crate::cert::ca::test_sign(&server, &endpoint);
    crate::cert::ca::test_sign_intermediate(&server, &endpoint);
    crate::cert::ca::test_sign_self_issued(&server, &endpoint);
    crate::cert::ca::test_delete(&server, &endpoint);
    crate::cert::ca::test_submit(&server, &endpoint);
    crate::cert::ca::test_delete(&server, &endpoint);
    crate::cert::ca::test_generate(&server, &endpoint);

    // Test intermediate CA
    crate::cert::ca::int::test_generate(&server, &endpoint);
    crate::cert::ca::int::test_set_signed(&server, &endpoint);

    // Test certs
    crate::cert::test_generate(&server, &endpoint);
    crate::cert::test_read(&server, &endpoint);
    crate::cert::test_list(&server, &endpoint);
    crate::cert::test_revoke(&server, &endpoint);
    crate::cert::test_tidy(&server, &endpoint);

    // Test CRLs
    crate::cert::crl::test_set_config(&server, &endpoint);
    crate::cert::crl::test_read_config(&server, &endpoint);
    crate::cert::crl::test_rotate(&server, &endpoint);

    // Test URLs
    crate::cert::urls::test_set(&server, &endpoint);
    crate::cert::urls::test_read(&server, &endpoint);
}

mod cert {
    use vaultrs::api::pki::requests::GenerateCertificateRequest;
    use vaultrs::pki::cert;

    use crate::PKIEndpoint;

    use super::VaultServer;

    pub fn test_generate(server: &VaultServer, endpoint: &PKIEndpoint) {
        let resp = cert::generate(
            &server.client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
            Some(GenerateCertificateRequest::builder().common_name("test.com")),
        );
        assert!(resp.is_ok());
        assert!(!resp.unwrap().certificate.is_empty())
    }

    pub fn test_list(server: &VaultServer, endpoint: &PKIEndpoint) {
        let res = cert::list(&server.client, endpoint.path.as_str());
        assert!(res.is_ok());
        assert!(!res.unwrap().is_empty());
    }

    pub fn test_read(server: &VaultServer, endpoint: &PKIEndpoint) {
        let certs = cert::list(&server.client, endpoint.path.as_str()).unwrap();

        let resp = cert::read(&server.client, endpoint.path.as_str(), certs[0].as_str());
        assert!(resp.is_ok());
        assert!(!resp.unwrap().certificate.is_empty());
    }

    pub fn test_revoke(server: &VaultServer, endpoint: &PKIEndpoint) {
        let cert = cert::generate(
            &server.client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
            Some(GenerateCertificateRequest::builder().common_name("test.com")),
        )
        .unwrap();

        let resp = cert::revoke(
            &server.client,
            endpoint.path.as_str(),
            cert.serial_number.as_str(),
        );
        assert!(resp.is_ok());
        assert!(resp.unwrap().revocation_time > 0);
    }

    pub fn test_tidy(server: &VaultServer, endpoint: &PKIEndpoint) {
        let resp = cert::tidy(&server.client, endpoint.path.as_str());
        assert!(resp.is_ok());
    }

    pub mod ca {
        use std::fs;

        use crate::{common::VaultServer, PKIEndpoint};
        use vaultrs::{api::pki::requests::GenerateRootRequest, pki::cert::ca};

        pub fn test_delete(server: &VaultServer, endpoint: &PKIEndpoint) {
            let resp = ca::delete(&server.client, endpoint.path.as_str());
            assert!(resp.is_ok());
        }

        pub fn test_generate(server: &VaultServer, endpoint: &PKIEndpoint) {
            let resp = ca::generate(
                &server.client,
                endpoint.path.as_str(),
                "internal",
                Some(
                    GenerateRootRequest::builder()
                        .common_name("Test")
                        .ttl("87600h"),
                ),
            );

            assert!(resp.is_ok());
            assert!(resp.unwrap().is_some());
        }

        pub fn test_sign(server: &VaultServer, endpoint: &PKIEndpoint) {
            let csr = fs::read_to_string("tests/files/csr.pem").unwrap();

            let resp = ca::sign(
                &server.client,
                endpoint.path.as_str(),
                endpoint.role.as_str(),
                csr.as_str(),
                "test.com",
                None,
            );

            assert!(resp.is_ok());
            assert!(!resp.unwrap().certificate.is_empty());
        }

        pub fn test_sign_intermediate(server: &VaultServer, endpoint: &PKIEndpoint) {
            let csr = fs::read_to_string("tests/files/csr.pem").unwrap();

            let resp = ca::sign_intermediate(
                &server.client,
                endpoint.path.as_str(),
                csr.as_str(),
                "test.com",
                None,
            );

            assert!(resp.is_ok());
            assert!(!resp.unwrap().certificate.is_empty());
        }

        pub fn test_sign_self_issued(server: &VaultServer, endpoint: &PKIEndpoint) {
            let cert = fs::read_to_string("tests/files/root_ca.crt").unwrap();

            let resp = ca::sign_self_issued(&server.client, endpoint.path.as_str(), cert.as_str());

            assert!(resp.is_ok());
            assert!(!resp.unwrap().certificate.is_empty());
        }

        pub fn test_submit(server: &VaultServer, endpoint: &PKIEndpoint) {
            let bundle = fs::read_to_string("tests/files/ca.pem").unwrap();

            let resp = ca::delete(&server.client, endpoint.path.as_str());
            assert!(resp.is_ok());

            let resp = ca::submit(&server.client, endpoint.path.as_str(), bundle.as_str());
            assert!(resp.is_ok());
        }

        pub mod int {
            use crate::common::VaultServer;
            use crate::PKIEndpoint;
            use vaultrs::pki::cert::ca;
            use vaultrs::pki::cert::ca::int;

            pub fn test_generate(server: &VaultServer, _: &PKIEndpoint) {
                let resp = server.mount("pki_int", "pki");
                assert!(resp.is_ok());

                let resp =
                    int::generate(&server.client, "pki_int", "internal", "test-int.com", None);

                assert!(resp.is_ok());
                assert!(!resp.unwrap().csr.is_empty());
            }

            pub fn test_set_signed(server: &VaultServer, endpoint: &PKIEndpoint) {
                let resp =
                    int::generate(&server.client, "pki_int", "internal", "test-int.com", None);
                assert!(resp.is_ok());

                let resp = ca::sign_intermediate(
                    &server.client,
                    endpoint.path.as_str(),
                    resp.unwrap().csr.as_str(),
                    "test-int.com",
                    None,
                );
                assert!(resp.is_ok());

                let resp = int::set_signed(
                    &server.client,
                    "pki_int",
                    resp.unwrap().certificate.as_str(),
                );
                assert!(resp.is_ok());
            }
        }
    }

    pub mod crl {
        use crate::{common::VaultServer, PKIEndpoint};
        use vaultrs::{api::pki::requests::SetCRLConfigRequest, pki::cert::crl};

        pub fn test_rotate(server: &VaultServer, endpoint: &PKIEndpoint) {
            let res = crl::rotate(&server.client, endpoint.path.as_str());
            assert!(res.is_ok());
            assert!(res.unwrap().success);
        }

        pub fn test_read_config(server: &VaultServer, endpoint: &PKIEndpoint) {
            let res = crl::set_config(
                &server.client,
                endpoint.path.as_str(),
                Some(SetCRLConfigRequest::builder().expiry("72h").disable(false)),
            );
            assert!(res.is_ok());

            let res = crl::read_config(&server.client, endpoint.path.as_str());
            assert!(res.is_ok());
            assert!(!res.unwrap().disable);
        }

        pub fn test_set_config(server: &VaultServer, endpoint: &PKIEndpoint) {
            let res = crl::set_config(
                &server.client,
                endpoint.path.as_str(),
                Some(SetCRLConfigRequest::builder().expiry("72h").disable(false)),
            );
            assert!(res.is_ok());
        }
    }

    pub mod urls {
        use crate::{common::VaultServer, PKIEndpoint};
        use vaultrs::{api::pki::requests::SetURLsRequest, pki::cert::urls};

        pub fn test_read(server: &VaultServer, endpoint: &PKIEndpoint) {
            let res = urls::read(&server.client, endpoint.path.as_str());
            assert!(res.is_ok());
            assert!(!res.unwrap().issuing_certificates.is_empty())
        }

        pub fn test_set(server: &VaultServer, endpoint: &PKIEndpoint) {
            let issue = format!("{}/v1/{}/ca", server.address, endpoint.path);
            let dist = format!("{}/v1/{}/crl", server.address, endpoint.path);

            let res = urls::set(
                &server.client,
                endpoint.path.as_str(),
                Some(
                    SetURLsRequest::builder()
                        .issuing_certificates(vec![issue])
                        .crl_distribution_points(vec![dist]),
                ),
            );
            assert!(res.is_ok());
        }
    }
}

mod role {
    use crate::{common::VaultServer, PKIEndpoint};
    use vaultrs::{api::pki::requests::SetRoleRequest, pki::role};

    pub fn test_delete(server: &VaultServer, endpoint: &PKIEndpoint) {
        let res = role::delete(
            &server.client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
        );
        assert!(res.is_ok());
    }

    pub fn test_list(server: &VaultServer, endpoint: &PKIEndpoint) {
        let res = role::list(&server.client, endpoint.path.as_str());
        assert!(res.is_ok());
        assert!(!res.unwrap().keys.is_empty());
    }

    pub fn test_read(server: &VaultServer, endpoint: &PKIEndpoint) {
        let res = role::read(
            &server.client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
        );
        assert!(res.is_ok());
        assert!(res.unwrap().allow_any_name)
    }

    pub fn test_set(server: &VaultServer, endpoint: &PKIEndpoint) {
        let res = role::set(
            &server.client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
            Some(SetRoleRequest::builder().allow_any_name(true)),
        );
        assert!(res.is_ok());
    }
}

#[derive(Debug)]
pub struct PKIEndpoint {
    pub path: String,
    pub role: String,
}

fn setup(server: &VaultServer) -> Result<PKIEndpoint, ClientError> {
    let path = "pki_test";
    let role = "test";

    // Mount the PKI engine
    let config = EnableEngineDataConfigBuilder::default()
        .max_lease_ttl("87600h")
        .build()
        .unwrap();
    server.mount_with_config(path, "pki", config)?;

    Ok(PKIEndpoint {
        path: path.to_string(),
        role: role.to_string(),
    })
}
