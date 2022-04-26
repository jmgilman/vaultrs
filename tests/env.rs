use std::env;
use vaultrs::client::VaultClient;
use vaultrs::client::VaultClientSettingsBuilder;

const VAULT_SKIP_VERIFY: &str = "VAULT_SKIP_VERIFY";

fn build_client() -> VaultClient {
    VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address("https://127.0.0.1:8200")
            .build()
            .unwrap(),
    )
    .unwrap()
}

#[test]
#[serial_test::serial]
fn test_should_verify_tls() {
    let values = vec!["", "1", "t", "T", "true", "True", "TRUE"];
    for value in values {
        env::set_var(VAULT_SKIP_VERIFY, value);
        let client = build_client();
        assert_eq!(client.settings.verify, true);
    }
}

#[test]
#[serial_test::serial]
fn test_should_not_verify_tls() {
    let values = vec!["0", "f", "F", "false", "False", "FALSE"];
    for value in values {
        env::set_var(VAULT_SKIP_VERIFY, value);
        let client = build_client();
        assert_eq!(client.settings.verify, false);
    }
}

#[test]
#[serial_test::serial]
fn test_should_verify_tls_if_variable_is_not_set() {
    env::remove_var(VAULT_SKIP_VERIFY);
    let client = build_client();
    assert_eq!(client.settings.verify, true);
}
