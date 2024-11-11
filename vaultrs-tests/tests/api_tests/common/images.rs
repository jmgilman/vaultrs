use reqwest::StatusCode;
use std::{borrow::Cow, collections::HashMap, fs, io::Write, path::PathBuf};
use testcontainers::{
    core::{wait::HttpWaitStrategy, ContainerPort, Mount, WaitFor},
    Image,
};

pub struct Vault {
    env_vars: HashMap<String, String>,
}

impl Default for Vault {
    fn default() -> Self {
        Self {
            env_vars: HashMap::from([("VAULT_DEV_ROOT_TOKEN_ID".to_owned(), "root".to_owned())]),
        }
    }
}

impl Image for Vault {
    fn name(&self) -> &str {
        VAULT_NAME
    }

    fn tag(&self) -> &str {
        VAULT_TAG
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::http(
            HttpWaitStrategy::new("/v1/sys/health").with_expected_status_code(StatusCode::OK),
        )]
    }

    fn expose_ports(&self) -> &[ContainerPort] {
        &[ContainerPort::Tcp(8200)]
    }

    fn env_vars(
        &self,
    ) -> impl IntoIterator<Item = (impl Into<Cow<'_, str>>, impl Into<Cow<'_, str>>)> {
        Box::new(self.env_vars.iter())
    }
}

pub struct TlsVault {
    env_vars: HashMap<String, String>,
    _binded_dir: tempfile::TempDir,
    volumes: Vec<Mount>,
}

impl TlsVault {
    pub fn new(vault_key: &str, vault_cert: &str, ca_cert: &str) -> Self {
        let binded_dir = tempfile::tempdir().unwrap();
        fs::write(binded_dir.path().join("ca_cert.crt"), ca_cert).unwrap();
        fs::write(binded_dir.path().join("vault_server.crt"), vault_cert).unwrap();
        fs::write(binded_dir.path().join("vault_server.key"), vault_key).unwrap();
        Self {
            env_vars: HashMap::from([
                (
                    "VAULT_LOCAL_CONFIG".to_owned(),
                    serde_json::json!({
                        "listener": [
                            {
                                "tcp": {
                                    "address": "0.0.0.0:8200",
                                    "tls_cert_file" : "/vault/config/vault_server.crt",
                                    "tls_key_file" : "/vault/config/vault_server.key",
                                    "tls_client_ca_file" : "/vault/config/ca_cert.crt",
                                    "tls_min_version" : "tls13",
                                }
                            }
                        ],
                        "storage": [
                            {
                                "inmem": {}
                            }
                        ],
                        "disable_mlock": true,
                        "log_level": "trace"
                    })
                    .to_string(),
                ),
                ("VAULT_DEV_ROOT_TOKEN_ID".to_owned(), "root".to_owned()),
                // Setting 9999 to leave 8200 available for the listener configured config.hcl
                (
                    "VAULT_DEV_LISTEN_ADDRESS".to_owned(),
                    "0.0.0.0:9999".to_owned(),
                ),
            ]),
            volumes: vec![Mount::bind_mount(
                binded_dir.path().to_str().unwrap(),
                "/vault/config",
            )],
            _binded_dir: binded_dir,
        }
    }

    pub fn ca_cert(&self) -> PathBuf {
        self._binded_dir.path().join("ca_cert.crt")
    }
}

impl Image for TlsVault {
    fn name(&self) -> &str {
        VAULT_NAME
    }

    fn tag(&self) -> &str {
        VAULT_TAG
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::http(
            HttpWaitStrategy::new("/v1/sys/health")
                .with_expected_status_code(StatusCode::OK)
                .with_client(
                    reqwest::ClientBuilder::new()
                        .danger_accept_invalid_certs(true)
                        .build()
                        .unwrap(),
                )
                .with_tls(),
        )]
    }

    fn expose_ports(&self) -> &[ContainerPort] {
        &[ContainerPort::Tcp(8200)]
    }

    fn env_vars(
        &self,
    ) -> impl IntoIterator<Item = (impl Into<Cow<'_, str>>, impl Into<Cow<'_, str>>)> {
        Box::new(self.env_vars.iter())
    }

    fn mounts(&self) -> impl IntoIterator<Item = &Mount> {
        Box::new(self.volumes.iter())
    }
}

/// A vault that is not in a dev mod.
/// Can be useful to test unseal and initialization workflows.
pub struct ProdVault {
    env_vars: HashMap<String, String>,
}

impl Default for ProdVault {
    fn default() -> Self {
        Self {
            env_vars: HashMap::from([(
                "VAULT_LOCAL_CONFIG".to_owned(),
                serde_json::json!({
                    "listener": [
                        {
                            "tcp": {
                                "address": "0.0.0.0:8200",
                                "tls_disable": "true"
                            }
                        }
                    ],
                    "storage": [
                        {
                            "inmem": {}
                        }
                    ],
                    "disable_mlock": true
                })
                .to_string(),
            )]),
        }
    }
}

impl Image for ProdVault {
    fn name(&self) -> &str {
        VAULT_NAME
    }

    fn tag(&self) -> &str {
        VAULT_TAG
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::http(
            HttpWaitStrategy::new("/v1/sys/health")
                .with_expected_status_code(StatusCode::NOT_IMPLEMENTED),
        )]
    }

    fn expose_ports(&self) -> &[ContainerPort] {
        &[ContainerPort::Tcp(8200)]
    }

    fn env_vars(
        &self,
    ) -> impl IntoIterator<Item = (impl Into<Cow<'_, str>>, impl Into<Cow<'_, str>>)> {
        Box::new(self.env_vars.iter())
    }

    fn cmd(&self) -> impl IntoIterator<Item = impl Into<Cow<'_, str>>> {
        // By default the Vault server will read the config file inside `/vault/config`
        vec!["server"].into_iter()
    }
}

/// Nginx is used as web server to mock kubernetes API
pub(super) struct Nginx {
    volumes: Vec<Mount>,
    _binded_dirs: Vec<tempfile::TempDir>,
}

impl Nginx {
    pub(super) fn new() -> Self {
        // Default conf, with a tweak to allow all http methods on static resources
        let nginx_conf = r#"
server {
    listen       80;
    server_name  localhost;

    location / {
        root   /usr/share/nginx/html;
        index  index.html index.htm;
    }
    # hack to allow all http methods on static resources
    error_page  405     =200 $uri;
}
"#
        .to_string();
        let binded_conf = tempfile::tempdir().unwrap();
        let mut conf = fs::File::create_new(binded_conf.path().join("nginx.conf")).unwrap();
        conf.write_all(nginx_conf.as_bytes()).unwrap();

        let binded_base = tempfile::tempdir().unwrap();
        let request_dir = binded_base
            .path()
            .join("apis")
            .join("authentication.k8s.io")
            .join("v1");
        fs::create_dir_all(&request_dir).unwrap();
        let mut index_html = fs::File::create_new(binded_base.path().join("index.html")).unwrap();
        index_html
            .write_all(b"<html><body>Hello World!</body></html>")
            .unwrap();
        let mut api_endpoint = fs::File::create_new(request_dir.join("tokenreviews")).unwrap();
        api_endpoint
            .write_all(
                serde_json::json!({
                  "apiVersion": "authentication.k8s.io/v1",
                  "kind": "TokenReview",
                  "status": {
                    "authenticated": true,
                    "user": {
                      "uid": "testuid",
                      "username": "system:serviceaccount:testns:test",
                    },
                    "audiences": ["vaultrs-test"]
                  }
                })
                .to_string()
                .as_bytes(),
            )
            .unwrap();

        Self {
            volumes: vec![
                Mount::bind_mount(
                    binded_base.path().to_str().unwrap(),
                    "/usr/share/nginx/html",
                ),
                Mount::bind_mount(binded_conf.path().to_str().unwrap(), "/etc/nginx/conf.d"),
            ],
            _binded_dirs: vec![binded_base, binded_conf],
        }
    }
}

impl Image for Nginx {
    fn name(&self) -> &str {
        NGINX_NAME
    }

    fn tag(&self) -> &str {
        NGINX_VERSION
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::http(
            HttpWaitStrategy::new("/").with_expected_status_code(StatusCode::OK),
        )]
    }

    fn expose_ports(&self) -> &[ContainerPort] {
        &[ContainerPort::Tcp(80)]
    }

    fn mounts(&self) -> impl IntoIterator<Item = &Mount> {
        Box::new(self.volumes.iter())
    }
}

/// Mock an identity provider, something that implements OpenID Connect.
pub(super) struct Oidc;

impl Image for Oidc {
    fn name(&self) -> &str {
        OIDC_NAME
    }

    fn tag(&self) -> &str {
        OIDC_VERSION
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::message_on_stdout(b"started server on address")]
    }
}

const VAULT_NAME: &str = "hashicorp/vault";
const VAULT_TAG: &str = "1.10.3";
const NGINX_NAME: &str = "nginx";
const NGINX_VERSION: &str = "1.21";
const OIDC_NAME: &str = "ghcr.io/navikt/mock-oauth2-server";
const OIDC_VERSION: &str = "0.3.5";
