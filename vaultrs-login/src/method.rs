use std::{collections::HashMap, convert::TryFrom, str::FromStr};

use serde::Deserialize;
use vaultrs::{client::Client, error::ClientError};

/// Contains the login methods currently supported by this crate
pub const SUPPORTED_METHODS: [Method; 4] =
    [Method::APPROLE, Method::OIDC, Method::USERPASS, Method::AWS];

/// Represents all login methods.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
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
            Method::USERPASS => write!(f, "Username/Password"),
        }
    }
}

impl FromStr for Method {
    type Err = ClientError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
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
            _ => return Err(ClientError::InvalidLoginMethodError),
        })
    }
}

impl TryFrom<String> for Method {
    type Error = ClientError;

    fn try_from(source: String) -> Result<Self, Self::Error> {
        source.parse()
    }
}

impl<'a> TryFrom<&'a str> for Method {
    type Error = ClientError;

    fn try_from(source: &'a str) -> Result<Self, Self::Error> {
        source.parse()
    }
}

#[allow(clippy::from_over_into)]
impl<'a> Into<&'a str> for Method {
    fn into(self) -> &'a str {
        match self {
            Method::ALICLOUD => "alicloud",
            Method::APPROLE => "approle",
            Method::AWS => "aws",
            Method::AZURE => "azure",
            Method::CERT => "cert",
            Method::CF => "cf",
            Method::GCP => "gcp",
            Method::GITHUB => "github",
            Method::KERBEROS => "kerberos",
            Method::KUBERNETES => "kubernetes",
            Method::LDAP => "ldap",
            Method::OCI => "oci",
            Method::OIDC => "oidc",
            Method::OKTA => "okta",
            Method::RADIUS => "radius",
            Method::TOKEN => "token",
            Method::USERPASS => "userpass",
        }
    }
}

impl From<Method> for String {
    fn from(m: Method) -> Self {
        let s: &str = m.into();
        s.to_string()
    }
}

/// Returns the default mount point for the given auth method
pub fn default_mount(method: &Method) -> String {
    method.clone().into()
}

/// Returns a list of login methods available on the Vault server
pub async fn list(client: &impl Client) -> Result<HashMap<String, Method>, ClientError> {
    let mounts = vaultrs::sys::auth::list(client).await?;
    let mut result = HashMap::new();
    for (path, info) in mounts {
        result.insert(path, info.mount_type.parse::<Method>()?);
    }
    Ok(result)
}

/// Returns a list of login methods currently supported by this crate
pub async fn list_supported(client: &impl Client) -> Result<HashMap<String, Method>, ClientError> {
    let mut mounts = list(client).await?;
    mounts.retain(|_, v| SUPPORTED_METHODS.contains(v));
    Ok(mounts)
}
