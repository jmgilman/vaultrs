use std::collections::HashMap;

use crate::{api::AuthInfo, client::VaultClient, error::ClientError};
use async_trait::async_trait;

/// Contains the login methods currently supported by this crate
pub const SUPPORTED_METHODS: [Method; 3] = [Method::APPROLE, Method::TOKEN, Method::USERPASS];

/// Represents all login methods.
#[derive(Debug, PartialEq, Eq)]
pub enum Method {
    ALICLOUD,
    APPROLE,
    AWS,
    AZURE,
    CERT,
    CF,
    GCP,
    GITHUB,
    KERBEROS,
    KUBERNETES,
    LDAP,
    OCI,
    OIDC,
    OKTA,
    RADIUS,
    TOKEN,
    UNKNOWN,
    USERPASS,
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Method::ALICLOUD => write!(f, "AliCloud"),
            Method::APPROLE => write!(f, "AppRole"),
            Method::AWS => write!(f, "AWS"),
            Method::AZURE => write!(f, "Azure"),
            Method::CERT => write!(f, "TLS Certificates"),
            Method::CF => write!(f, "Cloud Foundry"),
            Method::GCP => write!(f, "GCP"),
            Method::GITHUB => write!(f, "Github"),
            Method::KERBEROS => write!(f, "Kerberos"),
            Method::KUBERNETES => write!(f, "Kubernetes"),
            Method::LDAP => write!(f, "LDAP"),
            Method::OCI => write!(f, "Oracle Cloud Infrastructure"),
            Method::OIDC => write!(f, "OpenID Connect"),
            Method::OKTA => write!(f, "Okta"),
            Method::RADIUS => write!(f, "RADIUS"),
            Method::TOKEN => write!(f, "Token"),
            Method::UNKNOWN => write!(f, "Unknown"),
            Method::USERPASS => write!(f, "Username/Password"),
        }
    }
}

/// Returns a list of login methods available on the Vault server
pub async fn list(client: &VaultClient) -> Result<HashMap<String, Method>, ClientError> {
    let mounts = crate::sys::auth::list(client).await?;
    let mut result = HashMap::new();
    for (path, info) in mounts {
        let method = match info.mount_type.as_str() {
            "alicloud" => Method::ALICLOUD,
            "approle" => Method::APPROLE,
            "aws" => Method::AWS,
            "azure" => Method::AZURE,
            "cert" => Method::CERT,
            "cf" => Method::CF,
            "gcp" => Method::GCP,
            "github" => Method::GITHUB,
            "kerberos" => Method::KERBEROS,
            "kubernetes" => Method::KUBERNETES,
            "ldap" => Method::LDAP,
            "oci" => Method::OCI,
            "oidc" => Method::OIDC,
            "okta" => Method::OKTA,
            "radius" => Method::RADIUS,
            "token" => Method::TOKEN,
            "userpass" => Method::USERPASS,
            _ => Method::UNKNOWN,
        };

        result.insert(path, method);
    }
    Ok(result)
}

pub async fn list_supported(client: &VaultClient) -> Result<HashMap<String, Method>, ClientError> {
    let mut mounts = list(client).await?;
    mounts.retain(|_, v| SUPPORTED_METHODS.contains(v));
    Ok(mounts)
}

/// Represents a method for logging into Vault which returns a new token.
#[async_trait]
pub trait LoginMethod {
    async fn login(&self, client: &VaultClient, mount: &str) -> Result<AuthInfo, ClientError>;
}

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
