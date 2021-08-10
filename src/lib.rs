#[macro_use]
extern crate derive_builder;

mod api;
mod client;
mod error;
mod pki;
mod sys;

#[cfg(test)]
mod tests {
    use crate::api::sys::requests::EnableEngineDataBuilder;
    use crate::client::{VaultClient, VaultClientSettingsBuilder};
    use testcontainers::images::generic::{GenericImage, WaitFor};
    use testcontainers::Docker;

    #[test]
    fn create_mount() {
        let cl = testcontainers::clients::Cli::default();
        let im = GenericImage::new("vault")
            .with_env_var("VAULT_DEV_ROOT_TOKEN_ID", "mytoken")
            .with_wait_for(WaitFor::message_on_stdout(
                "Development mode should NOT be used in production installations!",
            ));

        let server = cl.run(im);
        let host_port = server.get_host_port(8200).unwrap();
        let url = format!("http://localhost:{}", host_port);
        println!("{}", url);

        let vcl = VaultClient::new(
            VaultClientSettingsBuilder::default()
                .address(url)
                .token("mytoken")
                .verify(false)
                .build()
                .unwrap(),
        )
        .unwrap();
        let data = EnableEngineDataBuilder::default()
            .engine_type("pki")
            .build()
            .unwrap();
        let resp = crate::sys::mount::enable(&vcl, "pki_temp", data);
        assert!(resp.is_ok());
    }
}
