use crate::{api::AuthInfo, client::VaultClient, error::ClientError};
use async_trait::async_trait;

#[async_trait]
pub trait LoginMethod {
    async fn login(&self, client: &VaultClient, mount: &str) -> Result<AuthInfo, ClientError>;
}

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
    async fn login(&self, client: &VaultClient, mount: &str) -> Result<AuthInfo, ClientError> {
        crate::auth::approle::login(
            client,
            mount,
            self.role_id.as_str(),
            self.secret_id.as_str(),
        )
        .await
    }
}

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
