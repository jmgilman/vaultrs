mod common;

use common::VaultServer;
use vaultrs::api::pki::requests::GenerateRootRequest;
use vaultrs::api::sys::requests::EnableEngineDataConfigBuilder;
use vaultrs::error::ClientError;

mod cert {
    use test_env_log::test;
    use vaultrs::api::pki::requests::GenerateCertificateRequest;
    use vaultrs::pki::cert;

    use super::setup;
    use super::VaultServer;

    #[test]
    fn test_generate() {
        let docker = testcontainers::clients::Cli::default();
        let server = VaultServer::new(&docker);
        let endpoint = setup(&server).unwrap();

        let resp = cert::generate(
            &server.client,
            endpoint.path.as_str(),
            endpoint.role.as_str(),
            Some(GenerateCertificateRequest::builder().common_name("test.com")),
        );
        assert!(resp.is_ok());
        assert!(!resp.unwrap().certificate.is_empty())
    }

    #[test]
    fn test_list() {
        let docker = testcontainers::clients::Cli::default();
        let server = VaultServer::new(&docker);
        let endpoint = setup(&server).unwrap();

        let res = cert::list(&server.client, endpoint.path.as_str());
        assert!(res.is_ok());
        assert!(!res.unwrap().is_empty());
    }

    #[test]
    fn test_read() {
        let docker = testcontainers::clients::Cli::default();
        let server = VaultServer::new(&docker);
        let endpoint = setup(&server).unwrap();
        let certs = cert::list(&server.client, endpoint.path.as_str()).unwrap();

        let resp = cert::read(&server.client, endpoint.path.as_str(), certs[0].as_str());
        assert!(resp.is_ok());
        assert!(!resp.unwrap().certificate.is_empty());
    }

    #[test]
    fn test_revoke() {
        let docker = testcontainers::clients::Cli::default();
        let server = VaultServer::new(&docker);
        let endpoint = setup(&server).unwrap();

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

    #[test]
    fn test_tidy() {
        let docker = testcontainers::clients::Cli::default();
        let server = VaultServer::new(&docker);
        let endpoint = setup(&server).unwrap();

        let resp = cert::tidy(&server.client, endpoint.path.as_str());
        assert!(resp.is_ok());
    }

    mod ca {
        use std::fs;

        use crate::{cert::setup, common::VaultServer};
        use test_env_log::test;
        use vaultrs::{api::pki::requests::GenerateRootRequest, pki::cert::ca};

        #[test]
        fn test_delete() {
            let docker = testcontainers::clients::Cli::default();
            let server = VaultServer::new(&docker);
            let endpoint = setup(&server).unwrap();

            let resp = ca::delete(&server.client, endpoint.path.as_str());
            assert!(resp.is_ok());
        }

        #[test]
        fn test_generate() {
            let docker = testcontainers::clients::Cli::default();
            let server = VaultServer::new(&docker);
            let endpoint = setup(&server).unwrap();

            let resp = ca::delete(&server.client, endpoint.path.as_str());
            assert!(resp.is_ok());

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

        #[test]
        fn test_sign() {
            let docker = testcontainers::clients::Cli::default();
            let server = VaultServer::new(&docker);
            let endpoint = setup(&server).unwrap();
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

        #[test]
        fn test_sign_intermediate() {
            let docker = testcontainers::clients::Cli::default();
            let server = VaultServer::new(&docker);
            let endpoint = setup(&server).unwrap();
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

        #[test]
        fn test_sign_self_issued() {
            let docker = testcontainers::clients::Cli::default();
            let server = VaultServer::new(&docker);
            let endpoint = setup(&server).unwrap();
            let cert = fs::read_to_string("tests/files/root_ca.crt").unwrap();

            let resp = ca::sign_self_issued(&server.client, endpoint.path.as_str(), cert.as_str());

            assert!(resp.is_ok());
            assert!(!resp.unwrap().certificate.is_empty());
        }

        #[test]
        fn test_submit() {
            let docker = testcontainers::clients::Cli::default();
            let server = VaultServer::new(&docker);
            let endpoint = setup(&server).unwrap();
            let bundle = fs::read_to_string("tests/files/ca.pem").unwrap();

            let resp = ca::delete(&server.client, endpoint.path.as_str());
            assert!(resp.is_ok());

            let resp = ca::submit(&server.client, endpoint.path.as_str(), bundle.as_str());
            assert!(resp.is_ok());
        }

        mod int {
            use crate::{common::VaultServer, setup};
            use vaultrs::pki::cert::ca;
            use vaultrs::pki::cert::ca::int;

            #[test]
            fn test_generate() {
                let docker = testcontainers::clients::Cli::default();
                let server = VaultServer::new(&docker);

                let resp = server.mount("pki_int", "pki");
                assert!(resp.is_ok());

                let resp =
                    int::generate(&server.client, "pki_int", "internal", "test-int.com", None);

                assert!(resp.is_ok());
                assert!(!resp.unwrap().csr.is_empty());
            }

            #[test]
            fn test_set_signed() {
                let docker = testcontainers::clients::Cli::default();
                let server = VaultServer::new(&docker);
                let endpoint = setup(&server).unwrap();

                let resp = server.mount("pki_int", "pki");
                assert!(resp.is_ok());

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

    mod crl {
        use crate::{common::VaultServer, setup};
        use vaultrs::{api::pki::requests::SetCRLConfigRequest, pki::cert::crl};

        #[test]
        fn test_rotate() {
            let docker = testcontainers::clients::Cli::default();
            let server = VaultServer::new(&docker);
            let endpoint = setup(&server).unwrap();

            let res = crl::rotate(&server.client, endpoint.path.as_str());
            assert!(res.is_ok());
            assert!(res.unwrap().success);
        }

        #[test]
        fn test_read_config() {
            let docker = testcontainers::clients::Cli::default();
            let server = VaultServer::new(&docker);
            let endpoint = setup(&server).unwrap();

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

        #[test]
        fn test_set_config() {
            let docker = testcontainers::clients::Cli::default();
            let server = VaultServer::new(&docker);
            let endpoint = setup(&server).unwrap();

            let res = crl::set_config(
                &server.client,
                endpoint.path.as_str(),
                Some(SetCRLConfigRequest::builder().expiry("72h").disable(false)),
            );
            assert!(res.is_ok());
        }
    }
}

#[derive(Debug)]
struct PKIEndpoint {
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

    // Generate the root CA
    vaultrs::pki::cert::ca::generate(
        &server.client,
        path,
        "internal",
        Some(
            GenerateRootRequest::builder()
                .common_name("Test")
                .ttl("87600h"),
        ),
    )?;

    // Configure CRL
    let issue = format!("{}/v1/{}/ca", server.address, path);
    let dist = format!("{}/v1/{}/crl", server.address, path);
    let req = vaultrs::pki::cert::urls::set_urls(path)
        .issuing_certificates(vec![issue])
        .crl_distribution_points(vec![dist])
        .build()
        .unwrap();
    server.client.execute(req)?;

    // Setup a test role
    let req = vaultrs::pki::role::set(path, role)
        .allow_any_name(true)
        .build()
        .unwrap();
    server.client.execute(req)?;

    Ok(PKIEndpoint {
        path: path.to_string(),
        role: role.to_string(),
    })
}
