mod common;

use common::VaultServer;
use vaultrs::api::sys::requests::EnableEngineDataBuilder;
use vaultrs::sys::mount;

#[test]
fn create_mount() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);
    let client = server.client();
    let data = EnableEngineDataBuilder::default()
        .engine_type("pki")
        .build()
        .unwrap();
    let resp = mount::enable(&client, "pki_temp", data);
    dbg!(resp);
}
