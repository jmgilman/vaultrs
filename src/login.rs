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

/// Returns a list of login methods currently supported by this crate
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

/// Represents a method for logging into Vault which returns a new token but
/// requires two separate steps to complete.
#[async_trait]
pub trait MultiLoginMethod {
    type Callback: MultiLoginCallback;

    async fn login<C: MultiLoginCallback>(
        &self,
        client: &VaultClient,
        mount: &str,
    ) -> Result<Self::Callback, ClientError>;
}

/// Represents the second step of a multi-step login method that returns the
/// authentication info.
#[async_trait]
pub trait MultiLoginCallback {
    async fn callback(self, client: &VaultClient, mount: &str) -> Result<AuthInfo, ClientError>;
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

#[cfg(feature = "oidc")]
use {
    tiny_http::{Response, Server},
    tokio::task::JoinHandle,
};

#[cfg(feature = "oidc")]
/// A login method which uses OIDC credentials for obtaining a new token.
#[derive(Debug)]
pub struct OIDCLogin {
    pub port: Option<u16>,    // Defaults to 8250
    pub role: Option<String>, // Defaults to what's configured in the backend
}

#[cfg(feature = "oidc")]
/// The callback for the OIDC login method.
#[derive(Debug)]
pub struct OIDCCallback {
    pub handle: JoinHandle<OIDCCallbackParams>,
    pub url: String,
}

#[cfg(feature = "oidc")]
// The parameters returned by the OAuth authorization server after successful
// authentication.
#[derive(Debug, Default)]
pub struct OIDCCallbackParams {
    pub code: String,
    pub nonce: String,
    pub state: String,
}

#[cfg(feature = "oidc")]
#[async_trait]
impl MultiLoginMethod for OIDCLogin {
    type Callback = OIDCCallback;

    /// Runs a standalone HTTP server which listens for the OIDC callback.
    ///
    /// This method performs several things. It firsts constructs a redirect URL
    /// which points back to the HTTP address of the web server it starts. It
    /// then asks Vault for an authroization URL using the constructed redirect.
    /// Finally, it starts a small HTTP server that listens for the redirect
    /// from the OAuth authorization server, capturing the various parameters
    /// and returning them as a [OIDCCallbackParams].
    ///
    /// The function returns an [OIDCCallback] which contains the authorization
    /// URL generated by Vault which an end-user must visit to complete the
    /// authorization flow. It also returns a handle to the task running the
    /// HTTP server. The `callback` method can be awaited on and will only
    /// return once the redirect has been received.
    async fn login<C: MultiLoginCallback>(
        &self,
        client: &VaultClient,
        mount: &str,
    ) -> Result<Self::Callback, ClientError> {
        // The Vault CLI uses http://localhost:8250/oidc/callback by default, so
        // we match that here to try and remain consistent
        let port = self.port.unwrap_or(8250);
        let ip = "127.0.0.1";
        let hostname = "localhost";

        let base = url::Url::parse(format!("http://{}:{}", hostname, port).as_str()).unwrap();
        let redirect = base.join("oidc/callback").unwrap().to_string();
        let response =
            crate::auth::oidc::auth(client, mount, redirect.as_str(), self.role.clone()).await?;
        let server = Server::http(format!("{}:{}", ip, port)).unwrap();

        let handle = tokio::task::spawn_blocking(move || {
            let mut result = OIDCCallbackParams::default();
            for request in server.incoming_requests() {
                let url = base.join(request.url()).unwrap();
                let query: HashMap<_, _> = url.query_pairs().into_owned().collect();

                result.code = query
                    .get("code")
                    .cloned()
                    .or_else(|| Some("".to_string()))
                    .unwrap();
                result.nonce = query
                    .get("nonce")
                    .cloned()
                    .or_else(|| Some("".to_string()))
                    .unwrap();
                result.state = query
                    .get("state")
                    .cloned()
                    .or_else(|| Some("".to_string()))
                    .unwrap();

                request
                    .respond(Response::from_string("Success!"))
                    .expect("Error responding!");
                server.unblock();
            }
            result
        });

        Ok(OIDCCallback {
            handle,
            url: response.auth_url,
        })
    }
}

#[cfg(feature = "oidc")]
#[async_trait]
impl MultiLoginCallback for OIDCCallback {
    /// Exchanges OIDC callback parameters for a Vault token.
    ///
    /// This method will block until the underlying HTTP server recieves a
    /// request from the OAuth authorization server at the redirect URL. It uses
    /// the resulting state, code, and nonce to retrieve a token from Vault.
    async fn callback(self, client: &VaultClient, mount: &str) -> Result<AuthInfo, ClientError> {
        let result = self.handle.await.unwrap();
        crate::auth::oidc::callback(
            client,
            mount,
            result.state.as_str(),
            result.nonce.as_str(),
            result.code.as_str(),
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
