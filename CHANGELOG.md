# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[unreleased]: https://github.com/jmgilman/vaultrs/compare/v0.5.2...HEAD
[0.5.2]: https://github.com/jmgilman/vaultrs/compare/v0.5.2
[0.5.1]: https://github.com/jmgilman/vaultrs/compare/v0.5.1
[0.5.0]: https://github.com/jmgilman/vaultrs/compare/v0.5.0
[0.4.0]: https://github.com/jmgilman/vaultrs/compare/v0.4.0
[0.3.0]: https://github.com/jmgilman/vaultrs/compare/v0.3.0
[0.2.0]: https://github.com/jmgilman/vaultrs/compare/v0.2.0
[0.1.1]: https://github.com/jmgilman/vaultrs/compare/v0.1.1
[0.1.0]: https://github.com/jmgilman/vaultrs/releases/tag/v0.1.0