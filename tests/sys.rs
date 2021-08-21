mod common;

use common::VaultServer;
use vaultrs::{api::sys::requests::ListMountsRequest, sys::mount};

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

#[test]
fn test_wrap() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);

    let endpoint = ListMountsRequest::builder().build().unwrap();
    let resp = vaultrs::api::wrap(&server.client, endpoint).unwrap();
    let unwrap_resp = mount::unwrap::<ListMountsRequest>(&server.client, resp.token.as_str());
    assert!(unwrap_resp.is_ok());
}
