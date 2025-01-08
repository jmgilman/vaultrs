use vaultrs::client::VaultClient;

use crate::{common::Test, ssh::key};

#[tokio::test]
async fn test() {
    let test = Test::builder().await;
    let client = test.client();
    let endpoint = TotpEndpoint::setup(client).await.unwrap();

    key::test_create(&endpoint).await;
    key::test_read(&endpoint).await;
    key::test_list(&endpoint).await;
    key::test_delete(&endpoint).await;

    code::test_generate(&endpoint).await;
    code::test_validate(&endpoint).await;
}

mod key {
    use vaultrs::totp::key;

    pub async fn test_create(endpoint: &TotpEndpoint<'_>) {
        let resp = key::create(endpoint.client, &endpoint.path, &endpoint.name, None).await;
        assert!(resp.is_ok());

        let resp = key::create(endpoint.client, &endpoint.path, &endpoint.name, None).await;
        assert!(resp.is_ok());
    }

    pub async fn test_read(endpoint: &TotpEndpoint<'_>) {
        let resp = key::read(endpoint.client, &endpoint.path, &endpoint.name)
            .await
            .unwrap();
        assert_eq!(&resp.name, &endpoint.name);
    }

    pub async fn test_list(endpoint: &TotpEndpoint<'_>) {
        let resp = key::list(endpoint.client, &endpoint.path).await.unwrap();
        assert!(&resp.keys.contains(&endpoint.name));
    }

    pub async fn test_delete(endpoint: &TotpEndpoint<'_>) {
        let resp = key::delete(endpoint.client, &endpoint.path, &endpoint.name).await;
        assert!(resp.is_ok());
    }
}

pub struct TotpEndpoint<'a> {
    pub client: &'a VaultClient,
    pub path: String,
    pub name: String,
}

impl<'a> TotpEndpoint<'a> {
    async fn setup(client: &'a VaultClient) -> Result<Self, ClientError> {
        let endpoint = TotpEndpoint {
            client,
            path: "totp-test".into(),
            name: "test-key".into(),
        };

        mount::enable(endpoint.client, &endpoint.auth, "totp", None)
            .await
            .unwrap();

        Ok(endpoint)
    }
}
