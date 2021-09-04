use crate::{api::AuthInfo, client::Client, error::ClientError, login::core::LoginMethod};
use async_trait::async_trait;

/// A login method which uses AppRole credentials for obtaining a new token.
#[derive(Debug)]
pub struct AppRoleLogin {
    pub role_id: String,
    pub secret_id: String,
}

impl AppRoleLogin {
    pub fn new(role_id: &str, secret_id: &str) -> Self {
        AppRoleLogin {
            role_id: role_id.to_string(),
            secret_id: secret_id.to_string(),
        }
    }
}

#[async_trait]
impl LoginMethod for AppRoleLogin {
    async fn login(&self, client: &impl Client, mount: &str) -> Result<AuthInfo, ClientError> {
        crate::auth::approle::login(
            client,
            mount,
            self.role_id.as_str(),
            self.secret_id.as_str(),
        )
        .await
    }
}
