pub mod mount {
    use crate::{
        api::sys::requests::{EnableEngineData, EnableEngineRequest},
        client::VaultClient,
        error::ClientError,
    };
    use rustify::endpoint::Endpoint;

    pub fn enable(
        client: &VaultClient,
        path: &str,
        data: EnableEngineData,
    ) -> Result<(), ClientError> {
        let req = EnableEngineRequest {
            path: path.to_string(),
            data,
        };
        req.execute(&client.http)?;
        Ok(())
    }
}
