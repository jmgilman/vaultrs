pub mod role {
    use crate::api;
    use crate::api::rabbitmq::{
        requests::GenerateCredentialsRequest, responses::GenerateCredentialsResponse,
    };
    use crate::client::Client;
    use crate::error::ClientError;

    /// Generates credentials from a role
    ///
    /// See [GenerateCredentialsRequest]
    pub async fn creds(
        client: &impl Client,
        mount: &str,
        name: &str,
    ) -> Result<GenerateCredentialsResponse, ClientError> {
        let endpoint = GenerateCredentialsRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_no_result(client, endpoint).await
    }
}
