# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.4] - 2025-02-21

### Added

- Add disable sys auth
- Add sys remount endpoint
- Add missing parameters to generate intermediate request

### Changed

- The internal testsuite use now testcontainers instead of dockertest
- Improve the testing speed by regrouping integration tests
- Reduce log verbosity on some trace

### Fixed

- Fix the readme example compilation

## [0.7.3] - 2024-11-08

### Added

- Add sys tool random
- Add cert auth configuration
- Add disable mount endpoint
- Add support for reading asymmetric keys from the /transit/keys route

### Changed

- bump rewest to 0.12 (which also bump rustls to 0.23)
- re-export identity to allow different version of reqwest
- improve logging
- update read certificate to vault 1.11.0

### Fixed

- do not log unseal key
- fix formatting
- Fixed links in changelog

## [0.7.2] - 2024-03-20

### Added

- Support for initial vault setup

### Fixed

- Bug to fail by compiling with disabled default features and native-tls


## [0.7.1] - 2024-03-16

### Added

- The `X-Vault-Request` Header to each request.
- Support for Client Certificate.
- Support custom metadata in KV2 secret engine.
- Support `expiration` field to `GenerateCertificateResponse`.
- Support for `AWS` secret engine.
- Partial support for `identity` secret engine (`entity`, `entity_alias`, `group` and `group_alias`).
### Fixed

- Issue with the `native-tls` feature where it doesn't compile.
- Issue where a URL was encoded twice what leads to wrong paths.
- Wrong name for `derived` filed in `CreateKeyRequest`.
- RUSTSEC-2023-0052 by bumping aws modules to the latest version.

## [0.7.0] - 2023-03-25

### Added

- AWS auth method
- Support for transit secret engine
- Implements Key Value v1 APIs

### Changed

- Allows choosing between rustls and native-tls
- Adds missing revocation_time when reading certificates
- Adds missing namespace header
- Removes printing of AWS login request

## [0.6.2] - 2022-05-01

### Added

- feat: allow timeout setting on client builder
- feat: add support for cas option for kv2 backend

## [0.6.1] - 2022-05-13

### Changed

- fix: makes rustls-tls usage consistent across dependencies
- fix: change login method from GET to POST
- fix: allow building VaultClientSettings without address
- fix: updates TLS verification according to VAULT_SKIP_VERIFY
- chore: upgrades dependencies
- chore: bumps supported Vault version to v1.10.3

## [0.6.0] - 2022-03-15

### Added

- Support for kubernetes authentication engine
- Support for sys/unseal

### Fixed

- Makes version field optional in secrets

## [0.5.4] - 2021-09-21

### Added

- Support for database secrets engine
- Tracing using the `tracing` crate

### Changed

- Internal structure of tests

## [0.5.3] - 2021-09-11

### Removed

- Dependency on openssl

## [0.5.2] - 2021-09-11

### Changed

- Bumps rustify to v0.4.4

## [0.5.1] - 2021-09-10

### Fixed

- Fixes bug where server status returned incorrectly with network errors

## [0.5.0] - 2021-09-09

### Added

- Support for working with policies

### Changed

- Pins tests to Vault v1.8.2
- Adds `Client` trait to allow making mock clients
- Uses `dockertest` over `testcontainers` for running tests
- Refactors test code to separate `vaultrs-test` crate
- Refactors login code to separate `vaultrs-login` crate

### Removed

- Removes token file methods from client

## [0.4.0] - 2021-09-02

### Added

- Support for AppRole auth method
- Support for Userpass auth method
- Support for sealing server
- Support for getting server status
- Support for using CA certificates in requests
- Generic login method for clients to easily obtain tokens
- Methods for isting available auth methods on a server
- Additional support for automating OIDC logins

## [0.3.0] - 2021-08-30

### Added

- Support for interacting with token backend
- Support for OIDC auth method
- Support for SSH secrets engine

## [0.2.0] - 2021-08-25

### Changed

- Applied changes from latest version of `rustify`
- Converted all functions to be async rather than synchronous

## [0.1.1] - 2021-08-22

### Fixed

- Incorrect doc comments in library file

## [0.1.0] - 2021-08-22

### Added

- Initial release

[unreleased]: https://github.com/jmgilman/vaultrs/compare/v0.7.4...HEAD
[0.7.4]: https://github.com/jmgilman/vaultrs/compare/v0.7.3...v0.7.4
[0.7.3]: https://github.com/jmgilman/vaultrs/compare/v0.7.2...v0.7.3
[0.7.2]: https://github.com/jmgilman/vaultrs/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/jmgilman/vaultrs/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/jmgilman/vaultrs/compare/v0.6.2...v0.7.0
[0.6.2]: https://github.com/jmgilman/vaultrs/compare/v0.6.1...v0.6.2
[0.6.1]: https://github.com/jmgilman/vaultrs/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/jmgilman/vaultrs/compare/v0.5.4...v0.6.0
[0.5.4]: https://github.com/jmgilman/vaultrs/compare/v0.5.3...v0.5.4
[0.5.3]: https://github.com/jmgilman/vaultrs/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/jmgilman/vaultrs/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/jmgilman/vaultrs/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/jmgilman/vaultrs/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/jmgilman/vaultrs/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/jmgilman/vaultrs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/jmgilman/vaultrs/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/jmgilman/vaultrs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/jmgilman/vaultrs/releases/tag/v0.1.0
