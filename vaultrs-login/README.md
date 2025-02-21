# vaultrs-login

> Adds login support for Vault clients from [vaultrs][1].

## Installation

Add `vaultrs-login` as a dependency to your cargo.toml:

```toml
[dependencies]
vaultrs-login = "0.2.3"
```

## Usage

```rust
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs_login::LoginClient;
use vaultrs_login::engines::approle::AppRoleLogin;

// Create a client
let mut client = VaultClient::new(
    VaultClientSettingsBuilder::default()
        .address("https://127.0.0.1:8200")
        .build()
        .unwrap()
).unwrap();

// Use one of the login flows to obtain a token for the client
let role_id = String::from("my-role-id");
let secret_id = String::from("secret");
let login = AppRoleLogin { role_id, secret_id };

client.login("approle", &login).await; // Token is automatically set to client
```

## Testing

Run tests with cargo:

```bash
cargo test
```
