pub mod config {

    use crate::{
        api::{
            self,
            aws::{
                requests::{
                    ConfigureLeaseRequest, GetConfigurationRequest, ReadLeaseRequest,
                    RotateRootCredentialsRequest, SetConfigurationRequest,
                    SetConfigurationRequestBuilder,
                },
                responses::{
                    GetConfigurationResponse, ReadLeaseResponse, RotateRootCredentialsResponse,
                },
            },
        },
        client::Client,
        error::ClientError,
    };

    /// Configures the root IAM credentials to communicate with AWS.
    ///
    /// See [SetConfigurationRequest]
    pub async fn set(
        client: &impl Client,
        mount: &str,
        access_key: &str,
        secret_key: &str,
        opts: Option<&mut SetConfigurationRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = SetConfigurationRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut t)
            .mount(mount)
            .access_key(access_key)
            .secret_key(secret_key)
            .build()
            .unwrap();

        api::exec_with_empty(client, endpoint).await
    }

    pub async fn get(
        client: &impl Client,
        mount: &str,
    ) -> Result<GetConfigurationResponse, ClientError> {
        let e = GetConfigurationRequest::builder()
            .mount(mount)
            .build()
            .unwrap();

        api::exec_with_result(client, e).await
    }

    pub async fn rotate(
        client: &impl Client,
        mount: &str,
    ) -> Result<RotateRootCredentialsResponse, ClientError> {
        let endpoint = RotateRootCredentialsRequest::builder()
            .mount(mount)
            .build()
            .unwrap();

        api::exec_with_result(client, endpoint).await
    }

    /// Configures the root IAM credentials to communicate with AWS.
    ///
    /// See [SetConfigurationRequest]
    pub async fn set_lease(
        client: &impl Client,
        mount: &str,
        lease: &str,
        lease_max: &str,
    ) -> Result<(), ClientError> {
        let endpoint = ConfigureLeaseRequest::builder()
            .mount(mount)
            .lease(lease)
            .lease_max(lease_max)
            .build()
            .unwrap();

        api::exec_with_empty(client, endpoint).await
    }

    pub async fn read_lease(
        client: &impl Client,
        mount: &str,
    ) -> Result<ReadLeaseResponse, ClientError> {
        let endpoint = ReadLeaseRequest::builder().mount(mount).build().unwrap();

        api::exec_with_result(client, endpoint).await
    }
}

pub mod roles {

    use crate::{
        api::{
            self,
            aws::{
                requests::{
                    CreateUpdateRoleRequest, CreateUpdateRoleRequestBuilder, DeleteRoleRequest,
                    GenerateCredentialsRequest, GenerateCredentialsRequestBuilder,
                    GenerateCredentialsStsRequest, GenerateCredentialsStsRequestBuilder,
                    ListRolesRequest, ReadRoleRequest,
                },
                responses::{GenerateCredentialsResponse, ListRolesResponse, ReadRoleResponse},
            },
        },
        client::Client,
        error::ClientError,
    };

    pub async fn create_update(
        client: &impl Client,
        mount: &str,
        name: &str,
        credential_type: &str,
        opts: Option<&mut CreateUpdateRoleRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = CreateUpdateRoleRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut t)
            .mount(mount)
            .name(name)
            .credential_type(credential_type)
            .build()
            .unwrap();

        api::exec_with_empty(client, endpoint).await
    }

    pub async fn read(
        client: &impl Client,
        mount: &str,
        name: &str,
    ) -> Result<ReadRoleResponse, ClientError> {
        let e = ReadRoleRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();

        api::exec_with_result(client, e).await
    }

    pub async fn delete(client: &impl Client, mount: &str, name: &str) -> Result<(), ClientError> {
        let e = DeleteRoleRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();

        api::exec_with_empty(client, e).await
    }

    pub async fn list(client: &impl Client, mount: &str) -> Result<ListRolesResponse, ClientError> {
        let e = ListRolesRequest::builder().mount(mount).build().unwrap();

        api::exec_with_result(client, e).await
    }

    /// Generate credentials using /aws/creds endpoint
    pub async fn credentials(
        client: &impl Client,
        mount: &str,
        name: &str,
        opts: Option<&mut GenerateCredentialsRequestBuilder>,
    ) -> Result<GenerateCredentialsResponse, ClientError> {
        let mut t = GenerateCredentialsRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut t)
            .mount(mount)
            .name(name)
            .build()
            .unwrap();

        api::exec_with_result(client, endpoint).await
    }

    /// Generate credentials using /aws/sts endpoint
    pub async fn credentials_sts(
        client: &impl Client,
        mount: &str,
        name: &str,
        opts: Option<&mut GenerateCredentialsStsRequestBuilder>,
    ) -> Result<GenerateCredentialsResponse, ClientError> {
        let mut t = GenerateCredentialsStsRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut t)
            .mount(mount)
            .name(name)
            .build()
            .unwrap();

        api::exec_with_result(client, endpoint).await
    }
}
