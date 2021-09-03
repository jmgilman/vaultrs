use crate::{api::AuthInfo, client::VaultClient, error::ClientError, login::core::LoginMethod};
use async_trait::async_trait;

/// A login method which uses user/pass credentials for obtaining a new token.
#[derive(Debug)]
pub struct UserpassLogin {
    pub password: String,
    pub username: String,
}

impl UserpassLogin {
    pub fn new(username: &str, password: &str) -> Self {
        UserpassLogin {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

#[async_trait]
impl LoginMethod for UserpassLogin {
    async fn login(&self, client: &VaultClient, mount: &str) -> Result<AuthInfo, ClientError> {
        crate::auth::userpass::login(
            client,
            mount,
            self.username.as_str(),
            self.password.as_str(),
        )
        .await
    }
}
