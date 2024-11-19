mod approle;
mod aws;
mod cert;
mod client;
mod common;
mod database;
mod identity;
mod kubernetes;
mod kv1;
mod kv2;
mod login;
mod oidc;
mod pki;
mod ssh;
mod sys;
mod token;
mod transit;
mod userpass;

// We use a single binary for integration tests because we want
// them to run in parallel
// https://users.rust-lang.org/t/how-to-execute-the-cargo-test-concurrently/92803/4
