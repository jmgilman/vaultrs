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
fn build_without_address() {
    let settings = VaultClientSettingsBuilder::default()
        .token("TOKEN")
        .build()
        .unwrap();

    assert_eq!("http://127.0.0.1:8200", settings.address);
}
