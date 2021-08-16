mod common;

use common::VaultServer;
use vaultrs::sys::mount;

#[test]
fn create_mount() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);
    let req = mount::enable("pki_temp")
        .engine_type("pki")
        .build()
        .unwrap();
    let resp = server.client.execute(req);
    assert!(resp.is_ok());

    let req = mount::list().build().unwrap();
    let mounts = server.client.execute(req);
    assert!(mounts.is_ok());
    assert!(mounts.unwrap().unwrap().contains_key("pki_temp/"));
}

#[test]
fn list_mount() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);

    let req = mount::list().build().unwrap();
    let resp = server.client.execute(req);
    assert!(resp.is_ok());
    assert!(resp.unwrap().is_some());
}
