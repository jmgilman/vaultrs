use std::env;
use std::sync::Mutex;

use reqwest::Url;
use vaultrs::client::VaultClient;
use vaultrs::client::VaultClientSettingsBuilder;

#[test]
fn build_without_token() {
    let settings = VaultClientSettingsBuilder::default()
        .address("https://127.0.0.1:9999")
        .build()
        .unwrap();

    assert_eq!("", settings.token);
}

#[test]
#[should_panic]
fn build_with_invalid_address_panics() {
    let _ = VaultClientSettingsBuilder::default().address("invalid_url");
}

#[test]
fn build_without_address() {
    let expected_address = "https://example.com:1234";
    env::set_var("VAULT_ADDR", expected_address);

    let settings = VaultClientSettingsBuilder::default().build().unwrap();
    assert_eq!(Url::parse(expected_address).unwrap(), settings.address);

    // What follows should, ideally, be a separate test case.
    // However, since we're using environment variables here
    // and those are a shared resource for the whole process,
    // (and tests are executed in parallel, in multiple threads),
    // this can lead to race conditions.
    // Since both cases test related behaviour, it's probably the simplest
    // solution to just test them this way.
    env::remove_var("VAULT_ADDR");

    let settings = VaultClientSettingsBuilder::default().build().unwrap();

    assert_eq!(
        Url::parse("http://127.0.0.1:8200").unwrap(),
        settings.address
    );
}

#[test]
fn build_with_proxy() {
    let expected_proxy = "https://example.com:1234";

    let settings = VaultClientSettingsBuilder::default()
        .address("http://127.0.0.1:8200")
        .proxy(expected_proxy)
        .build()
        .unwrap();
    assert_eq!(Url::parse(expected_proxy).unwrap(), settings.proxy.unwrap());
    assert_eq!(
        Url::parse("http://127.0.0.1:8200").unwrap(),
        settings.address
    );
}

#[test]
#[should_panic]
fn build_with_invalid_proxy_panics() {
    let _ = VaultClientSettingsBuilder::default()
        .proxy("invalid_url")
        .build();
}

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
fn test_should_skip_tls_verification() {
    serialized(|| {
        for value in ["", "1", "t", "T", "true", "True", "TRUE"] {
            env::set_var(VAULT_SKIP_VERIFY, value);
            let client = build_client();
            assert!(!client.settings.verify);
        }
    });
}

#[test]
fn test_should_not_skip_tls_verification() {
    serialized(|| {
        for value in ["0", "f", "F", "false", "False", "FALSE"] {
            env::set_var(VAULT_SKIP_VERIFY, value);
            let client = build_client();
            assert!(client.settings.verify);
        }
    });
}

#[test]
fn test_should_verify_tls_if_variable_is_not_set() {
    serialized(|| {
        env::remove_var(VAULT_SKIP_VERIFY);
        let client = build_client();
        assert!(client.settings.verify);
    });
}

/// Approximates `#[serial]` from the `serial_test` crate.
///
/// No attempt is made to recover from a poisoned mutex, which will
/// happen when `f` panics. In other words, all the tests that use
/// `serialized` will start failing after one test panics.
// Taken from here <https://github.com/rustls/rustls/blob/257e511ce879e8f785484528cc78fdf2d83ec182/rustls-test/tests/key_log_file_env.rs#L34>
#[allow(dead_code)]
fn serialized(f: impl FnOnce()) {
    // Ensure every test is run serialized
    static MUTEX: Mutex<()> = const { Mutex::new(()) };

    let _guard = MUTEX.lock().unwrap();

    f()
}
