mod common;

use common::VaultServer;
use vaultrs::api::sys::requests::EnableEngineDataConfigBuilder;
use vaultrs::error::ClientError;

mod cert {
    use test_env_log::test;
    use vaultrs::pki::cert;

    use super::setup;
    use super::VaultServer;

    #[test]
    fn test_generate() {
        let docker = testcontainers::clients::Cli::default();
        let server = VaultServer::new(&docker);
        let endpoint = setup(&server).unwrap();
        let domain = "test.com";

        let req = cert::generate(endpoint.path.as_str(), endpoint.role.as_str())
            .common_name(domain)
            .build()
            .unwrap();
        let resp = server.client.execute(req);
        assert!(resp.is_ok());
        assert!(resp.unwrap().is_some());
    }

    #[test]
    fn test_list() {
        let docker = testcontainers::clients::Cli::default();
        let server = VaultServer::new(&docker);
        let endpoint = setup(&server).unwrap();

        let req = cert::list(endpoint.path.as_str()).build().unwrap();
        let res = server.client.execute(req);
        assert!(res.is_ok());
        assert!(res.unwrap().is_some());
    }

    #[test]
    fn test_read() {
        let docker = testcontainers::clients::Cli::default();
        let server = VaultServer::new(&docker);
        let endpoint = setup(&server).unwrap();

        let req = cert::list(endpoint.path.as_str()).build().unwrap();
        let certs = server.client.execute(req).unwrap().unwrap();

        let req = cert::read(endpoint.path.as_str(), certs.keys[0].as_str())
            .build()
            .unwrap();
        let resp = server.client.execute(req);
        assert!(resp.is_ok());
        assert!(resp.unwrap().is_some());
    }

    #[test]
    fn test_revoke() {
        let docker = testcontainers::clients::Cli::default();
        let server = VaultServer::new(&docker);
        let endpoint = setup(&server).unwrap();
        let domain = "test.com";

        let req = cert::generate(endpoint.path.as_str(), endpoint.role.as_str())
            .common_name(domain)
            .build()
            .unwrap();
        let cert = server.client.execute(req).unwrap().unwrap();

        let req = cert::revoke(endpoint.path.as_str(), cert.serial_number.as_str())
            .build()
            .unwrap();
        let resp = server.client.execute(req);
        assert!(resp.is_ok());
        assert!(resp.unwrap().is_some());
    }

    #[test]
    fn test_tidy() {
        let docker = testcontainers::clients::Cli::default();
        let server = VaultServer::new(&docker);
        let endpoint = setup(&server).unwrap();

        let req = cert::tidy(endpoint.path.as_str()).build().unwrap();
        let resp = server.client.execute(req);
        assert!(resp.is_ok());
        assert!(resp.unwrap().is_some());
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
    let req = vaultrs::pki::cert::ca::generate(path, "internal")
        .common_name("Test")
        .ttl("87600h")
        .build()
        .unwrap();
    server.client.execute(req)?;

    // Configure CRL
    let issue = format!("{}/v1/{}/ca", server.address, path);
    let dist = format!("{}/v1/{}/crl", server.address, path);
    let req = vaultrs::pki::cert::urls::set_urls(path)
        .issuing_certificates(vec![issue])
        .crl_distribution_points(vec![dist])
        .build()
        .unwrap();
    let r = server.client.execute(req)?;
    dbg!(r);

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
