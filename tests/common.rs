use testcontainers::clients::Cli;
use testcontainers::images::generic::{GenericImage, WaitFor};
use testcontainers::{Container, Docker};
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};

const TOKEN: &str = "testtoken";

pub struct VaultServer<'a> {
    pub container: Container<'a, Cli, GenericImage>,
    token: String,
}

impl<'a> VaultServer<'a> {
    pub fn new(client: &'a Cli) -> Self {
        let im = GenericImage::new("vault")
            .with_env_var("VAULT_DEV_ROOT_TOKEN_ID", TOKEN)
            .with_wait_for(WaitFor::message_on_stdout(
                "Development mode should NOT be used in production installations!",
            ));
        let cont = client.run(im);
        VaultServer {
            token: TOKEN.to_string(),
            container: cont,
        }
    }

    pub fn addr(&self) -> String {
        let host_port = self.container.get_host_port(8200).unwrap();
        format!("http://localhost:{}", host_port)
    }

    pub fn client(&self) -> VaultClient {
        VaultClient::new(
            VaultClientSettingsBuilder::default()
                .address(self.addr())
                .token(self.token.as_str())
                .build()
                .unwrap(),
        )
        .unwrap()
    }
}
