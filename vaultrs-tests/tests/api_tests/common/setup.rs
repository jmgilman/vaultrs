use super::images::{Nginx, Oidc, ProdVault, TlsVault, Vault};
use rcgen::{CertificateParams, KeyPair};
use std::{
    future::{Future, IntoFuture},
    path::{Path, PathBuf},
    pin::Pin,
};
use testcontainers::{runners::AsyncRunner, ContainerAsync, Image, ImageExt};
use testcontainers_modules::{localstack::LocalStack, postgres::Postgres};
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};

pub struct Test<I>
where
    I: Image,
{
    client: VaultClient,
    /// Vault handle, remove the container on drop.
    _vault: ContainerAsync<I>,
    /// If Vault use TLS, the CA that signed its certificate.
    ca_cert: Option<PathBuf>,
    localstack: Option<RunningLocalStack>,
    postgres: Option<RunningPostgres>,
    nginx: Option<RunningNginx>,
    oidc: Option<RunningOidc>,
}

impl<T> Test<T>
where
    T: Image,
{
    pub fn client(&self) -> &VaultClient {
        &self.client
    }

    pub fn client_mut(&mut self) -> &mut VaultClient {
        &mut self.client
    }

    pub fn localstack_url(&self) -> Option<&str> {
        self.localstack
            .as_ref()
            .map(|localstack| localstack.url.as_str())
    }

    pub fn postgres_url(&self) -> Option<&str> {
        self.postgres.as_ref().map(|postgres| postgres.url.as_str())
    }

    pub fn nginx_url(&self) -> Option<&str> {
        self.nginx.as_ref().map(|nginx| nginx.url.as_str())
    }

    pub fn oidc_url(&self) -> Option<&str> {
        self.oidc.as_ref().map(|oidc| oidc.url.as_str())
    }

    pub fn ca_cert(&self) -> Option<&Path> {
        self.ca_cert.as_deref()
    }
}

#[derive(Default)]
pub struct TestBuilder {
    localstack: Option<String>,
    nginx: bool,
    postgres: bool,
    oidc: bool,
}

impl TestBuilder {
    pub fn with_postgres(mut self) -> Self {
        self.postgres = true;
        self
    }

    pub fn with_nginx(mut self) -> Self {
        self.nginx = true;
        self
    }

    pub fn with_oidc(mut self) -> Self {
        self.oidc = true;
        self
    }

    pub fn with_localstack(
        mut self,
        services: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        // TODO: when Iterator::intersperse is stable use it.
        // https://docs.localstack.cloud/references/configuration//
        let mut services_env = String::new();
        for service in services {
            let service: String = service.into();
            services_env.push_str(&service);
            services_env.push(',');
        }
        let services_env = services_env.strip_suffix(',').unwrap();
        self.localstack = Some(services_env.to_string());
        self
    }

    // Don't use this directly, just use `.await` the `TestBuilder`.
    async fn build(self) -> Test<Vault> {
        let _ = tracing_subscriber::FmtSubscriber::builder()
            .with_test_writer()
            .try_init();

        let nginx = if self.nginx {
            let nginx = Nginx::new().start().await.unwrap();
            let bridge_ip = nginx.get_bridge_ip_address().await.unwrap();
            let url = format!("http://{bridge_ip}:80");
            Some(RunningNginx { _nginx: nginx, url })
        } else {
            None
        };

        let postgres = if self.postgres {
            let postgres = Postgres::default()
                .with_user(POSTGRES_USER)
                .with_password(POSTGRES_PASSWORD)
                .start()
                .await
                .unwrap();
            let bridge_ip = postgres.get_bridge_ip_address().await.unwrap();
            let url = format!("{bridge_ip}:5432");
            Some(RunningPostgres {
                _postgres: postgres,
                url,
            })
        } else {
            None
        };

        let localstack = if let Some(services) = self.localstack {
            let localstack = LocalStack::default()
                .with_env_var("SERVICES", services)
                .start()
                .await
                .unwrap();
            let bridge_ip = localstack.get_bridge_ip_address().await.unwrap();
            let url = format!("http://{bridge_ip}:4566");
            Some(RunningLocalStack {
                _localstack: localstack,
                url,
            })
        } else {
            None
        };

        let oidc = if self.oidc {
            let oidc = Oidc.start().await.unwrap();
            let host = oidc.get_bridge_ip_address().await.unwrap();
            let url = format!("http://{host}:8080");
            Some(RunningOidc { _oidc: oidc, url })
        } else {
            None
        };

        let vault = Vault::default().start().await.unwrap();
        let host_port = vault.get_host_port_ipv4(8200).await.unwrap();
        let addr = format!("http://localhost:{host_port}");

        let client = VaultClient::new(
            VaultClientSettingsBuilder::default()
                .address(addr)
                .token("root")
                .build()
                .unwrap(),
        )
        .unwrap();

        Test {
            localstack,
            nginx,
            postgres,
            client,
            oidc,
            _vault: vault,
            ca_cert: None,
        }
    }
}

impl IntoFuture for TestBuilder {
    type Output = Test<Vault>;

    // TODO: update when impl_trait_in_assoc_type is stabilized.
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + 'static>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.build())
    }
}

impl Test<Vault> {
    pub fn builder() -> TestBuilder {
        TestBuilder::default()
    }
}

impl Test<TlsVault> {
    pub async fn new_tls() -> Self {
        let _ = tracing_subscriber::FmtSubscriber::builder()
            .with_test_writer()
            .try_init();
        let TlsSetup {
            vault_key,
            vault_cert,
            client_bundle,
            ca_cert,
        } = generate_certs();
        let vault = TlsVault::new(&vault_key, &vault_cert, &ca_cert)
            .start()
            .await
            .unwrap();
        let ca_cert = vault.image().ca_cert();
        let host_port = vault.get_host_port_ipv4(8200).await.unwrap();
        let addr = format!("https://localhost:{host_port}");

        let identity = reqwest::Identity::from_pem(&client_bundle).unwrap();

        let client = VaultClient::new(
            VaultClientSettingsBuilder::default()
                .address(addr)
                .token("root")
                .identity(Some(identity))
                .ca_certs(vec![ca_cert.to_str().unwrap().to_string()])
                .build()
                .unwrap(),
        )
        .unwrap();

        Self {
            client,
            _vault: vault,
            localstack: None,
            postgres: None,
            nginx: None,
            oidc: None,
            ca_cert: Some(ca_cert),
        }
    }
}

impl Test<ProdVault> {
    pub async fn new_prod() -> Self {
        let (client, vault) = Self::new_vault_prod().await;

        Self {
            client,
            _vault: vault,
            localstack: None,
            postgres: None,
            nginx: None,
            oidc: None,
            ca_cert: None,
        }
    }

    async fn new_vault_prod() -> (VaultClient, ContainerAsync<ProdVault>) {
        let _ = tracing_subscriber::FmtSubscriber::builder()
            .with_test_writer()
            .try_init();

        let vault = ProdVault::default().start().await.unwrap();
        let host_port = vault.get_host_port_ipv4(8200).await.unwrap();
        let addr = format!("http://localhost:{host_port}");

        let client = VaultClient::new(
            VaultClientSettingsBuilder::default()
                .address(addr)
                .build()
                .unwrap(),
        )
        .unwrap();
        (client, vault)
    }
}

pub const POSTGRES_USER: &str = "postgres";
pub const POSTGRES_PASSWORD: &str = "postgres";

struct RunningOidc {
    /// OIDC handle, remove the container on drop.
    _oidc: ContainerAsync<Oidc>,
    url: String,
}

struct RunningLocalStack {
    /// Localstack handle, remove the container on drop.
    _localstack: ContainerAsync<LocalStack>,
    url: String,
}

struct RunningPostgres {
    /// Postgres handle, remove the container on drop.
    _postgres: ContainerAsync<Postgres>,
    url: String,
}

struct RunningNginx {
    /// Nginx handle, remove the container on drop.
    _nginx: ContainerAsync<Nginx>,
    url: String,
}

fn generate_certs() -> TlsSetup {
    let mut params = CertificateParams::new([]).unwrap();
    params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
    let ca_key_pair = KeyPair::generate().unwrap();
    let ca_cert = params.self_signed(&ca_key_pair).unwrap();

    let vault_key_pair = KeyPair::generate().unwrap();
    let params = CertificateParams::new(["localhost".to_string()]).unwrap();
    let vault_cert = params
        .signed_by(&vault_key_pair, &ca_cert, &ca_key_pair)
        .unwrap();

    let client_key_pair = KeyPair::generate().unwrap();
    let client_cert = CertificateParams::new([])
        .unwrap()
        .signed_by(&client_key_pair, &ca_cert, &ca_key_pair)
        .unwrap();

    let mut client_bundle = client_cert.pem().into_bytes();
    let mut client_key = client_key_pair.serialize_pem().into_bytes();
    client_bundle.append(&mut client_key);

    TlsSetup {
        vault_key: vault_key_pair.serialize_pem(),
        vault_cert: vault_cert.pem(),
        client_bundle,
        ca_cert: ca_cert.pem(),
    }
}

struct TlsSetup {
    ca_cert: String,
    vault_key: String,
    vault_cert: String,
    client_bundle: Vec<u8>,
}
