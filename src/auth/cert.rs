use crate::{
    api::{self, auth::cert::requests::LoginRequest, AuthInfo},
    client::Client,
    error::ClientError,
};

// Fetch a token with policies corresponding to the certificate.
//
// See [LoginRequest]
#[instrument(skip(client), err)]
pub async fn login(
    client: &impl Client,
    mount: &str,
    cert_name: &str,
) -> Result<AuthInfo, ClientError> {
    let endpoint = LoginRequest::builder()
        .mount(mount)
        .cert_name(cert_name)
        .build()
        .unwrap();
    api::auth(client, endpoint).await
}
