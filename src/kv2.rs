pub mod config {
    use crate::{
        api::{
            self,
            kv2::{
                requests::{
                    ReadConfigurationRequest, SetConfigurationRequest,
                    SetConfigurationRequestBuilder,
                },
                responses::ReadConfigurationResponse,
            },
        },
        client::VaultClient,
        error::ClientError,
    };

    /// Read the configuration of the mounted KV engine
    ///
    /// See [ReadConfigurationResponse]
    pub fn read(
        client: &VaultClient,
        mount: &str,
    ) -> Result<ReadConfigurationResponse, ClientError> {
        let endpoint = ReadConfigurationRequest::builder()
            .mount(mount)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint)
    }

    /// Update the configuration of the mounted KV engine
    ///
    /// See [SetConfigurationRequest]
    pub fn set(
        client: &VaultClient,
        mount: &str,
        opts: Option<&mut SetConfigurationRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = SetConfigurationRequest::builder();
        let endpoint = opts.unwrap_or(&mut t).mount(mount).build().unwrap();
        api::exec_with_empty(client, endpoint)
    }
}
