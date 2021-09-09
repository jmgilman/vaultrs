# vaultrs-login

> Adds login support for Vault clients from [vaultrs][1].

## Installation

Add `vaultrs-test` as a developemnt depdendency to your cargo.toml:
```
[dev-dependencies]
vaultrs-test = "0.1.0"
```

## Usage

```rust
use vaultrs_test::docker::{Server, ServerConfig};
use vaultrs_test::{VaultServer, VaultServerConfig};

// Configures a container to run Vault server v1.8.2
let config = VaultServerConfig::default(Some("1.8.2"));

// Creates a test instance to run the container in
let instance = config.to_instance();

// Runs the test instance, passing in details about the container environment
instance.run(|ops| async move {
    // The code below only runs after the container is verified running

    // Creates an abstraction for interacting with the Vault container
    let server = VaultServer::new(&ops, &config);

    // Run test code against container
})

// Container is cleaned up at this point
```

## Testing

Run tests with cargo:

```
cargo test
```

## Contributing

Check out the [issues][2] for items neeeding attention or submit your own and 
then:

1. Fork the repo (https://github.com/jmgilman/vaultrs-test/fork)
2. Create your feature branch (git checkout -b feature/fooBar)
3. Commit your changes (git commit -am 'Add some fooBar')
4. Push to the branch (git push origin feature/fooBar)
5. Create a new Pull Request

[1]: https://github.com/jmgilman/vaultrs
[2]: https://github.com/jmgilman/vaultrs-test/issues