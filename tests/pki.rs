mod common;

use common::VaultServer;
use vaultrs::api::sys::requests::EnableEngineDataConfigBuilder;
use vaultrs::error::ClientError;

mod cert {
    use vaultrs::pki::cert;

    use super::setup;
    use super::VaultServer;

    #[test]
    fn test_generate() {
        let docker = testcontainers::clients::Cli::default();
        let server = VaultServer::new(&docker);
        let endpoint = setup(&server).unwrap();
        let domain = "test.com";

        let r = cert::generate(endpoint.path.as_str(), endpoint.role.as_str())
            .common_name(domain)
            .execute(&server.client.http);
        assert!(r.is_ok());
        assert!(r.unwrap().is_some());
    }

    #[test]
    fn test_list() {
        let docker = testcontainers::clients::Cli::default();
        let server = VaultServer::new(&docker);
        let endpoint = setup(&server).unwrap();

        let r = cert::list(endpoint.path.as_str()).execute(&server.client.http);
        assert!(r.is_ok());
        assert!(r.unwrap().is_some());
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
    vaultrs::pki::cert::ca::generate(path, "internal")
        .common_name("Test")
        .ttl("87600h")
        .execute(&server.client.http)
        .map_err(ClientError::from)?;

    // Setup a test role
    vaultrs::pki::role::set(path, role)
        .allow_any_name(true)
        .execute(&server.client.http)
        .map_err(ClientError::from)?;

    Ok(PKIEndpoint {
        path: path.to_string(),
        role: role.to_string(),
    })
}
