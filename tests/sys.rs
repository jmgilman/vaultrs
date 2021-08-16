mod common;

use common::VaultServer;
use vaultrs::sys::mount;

#[test]
fn create_mount() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);

    let resp = mount::enable(&server.client, "pki_temp", "pki", None);
    assert!(resp.is_ok());

    let mounts = mount::list(&server.client);
    assert!(mounts.is_ok());
    assert!(mounts.unwrap().contains_key("pki_temp/"));
}

#[test]
fn list_mount() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);

    let resp = mount::list(&server.client);
    assert!(resp.is_ok());
    assert!(!resp.unwrap().is_empty());
}
