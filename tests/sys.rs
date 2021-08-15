mod common;

use common::VaultServer;
use vaultrs::sys::mount;

#[test]
fn create_mount() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);
    let r = mount::enable("pki_temp")
        .engine_type("pki")
        .execute(&server.client.http);
    assert!(r.is_ok());

    let mounts = mount::list().execute(&server.client.http);
    assert!(mounts.is_ok());
    assert!(mounts.unwrap().unwrap().contains_key("pki_temp/"));
}

#[test]
fn list_mount() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);
    let r = mount::list().execute(&server.client.http);
    assert!(r.is_ok());
    assert!(r.unwrap().is_some());
}
