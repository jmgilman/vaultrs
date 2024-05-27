use crate::{
    api::{
        self,
        auth::cert::requests::{
            ConfigureTlsCertificateMethod, ConfigureTlsCertificateMethodBuilder, LoginRequest,
        },
        AuthInfo,
    },
    client::Client,
    error::ClientError,
};

// Fetch a token with policies corresponding to the certificate.
//
// See [LoginRequest]
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

/// ConfigureTlsCertificateMethod
///
/// See [ConfigureTlsCertificateMethod]
pub async fn configure_tls_certificate_method(
    client: &impl Client,
    mount: &str,
    opts: Option<&mut ConfigureTlsCertificateMethodBuilder>,
) -> Result<(), ClientError> {
    let mut t = ConfigureTlsCertificateMethod::builder();
    let endpoint = opts.unwrap_or(&mut t).mount(mount).build().unwrap();
    api::exec_with_empty(client, endpoint).await
}

pub mod ca_cert_role {
    use crate::{
        api::{
            self,
            auth::cert::{
                requests::{
                    CreateCaCertificateRoleRequest, CreateCaCertificateRoleRequestBuilder,
                    DeleteCaCertificateRoleRequest, ListCaCertificateRoleRequest,
                    ReadCaCertificateRoleRequest,
                },
                responses::{ListCaCertificateRoleResponse, ReadCaCertificateRoleResponse},
            },
        },
        client::Client,
        error::ClientError,
    };

    /// Deletes a CA certificate role.
    ///
    /// See [DeleteCaCertificateRoleRequest]
    pub async fn delete(client: &impl Client, mount: &str, name: &str) -> Result<(), ClientError> {
        let endpoint = DeleteCaCertificateRoleRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Lists CA certificate roles.
    ///
    /// See [ListCaCertificateRoleRequest]
    pub async fn list(
        client: &impl Client,
        mount: &str,
    ) -> Result<ListCaCertificateRoleResponse, ClientError> {
        let endpoint = ListCaCertificateRoleRequest::builder()
            .mount(mount)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Reads information about a CA certificate role.
    ///
    /// See [ReadCaCertificateRoleRequest]
    pub async fn read(
        client: &impl Client,
        mount: &str,
        username: &str,
    ) -> Result<ReadCaCertificateRoleResponse, ClientError> {
        let endpoint = ReadCaCertificateRoleRequest::builder()
            .mount(mount)
            .name(username)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Creates a new CA certificate role
    ///
    /// See [CreateCaCertificateRoleRequest]
    pub async fn set(
        client: &impl Client,
        mount: &str,
        name: &str,
        certificate: &str,
        opts: Option<&mut CreateCaCertificateRoleRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = CreateCaCertificateRoleRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut t)
            .mount(mount)
            .name(name)
            .certificate(certificate)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }
}
