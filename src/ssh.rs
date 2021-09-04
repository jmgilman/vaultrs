use crate::api;
use crate::api::ssh::requests::{GenerateSSHCredsRequest, VerifySSHOTPRequest};
use crate::api::ssh::responses::{GenerateSSHCredsResponse, VerifySSHOTPResponse};
use crate::client::Client;
use crate::error::ClientError;

/// Generates SSH credentials for the given role
///
/// See [GenerateSSHCredsRequest]
pub async fn generate(
    client: &impl Client,
    mount: &str,
    name: &str,
    ip: &str,
    username: Option<String>,
) -> Result<GenerateSSHCredsResponse, ClientError> {
    let mut endpoint = GenerateSSHCredsRequest::builder();
    if let Some(u) = username {
        endpoint.username(u);
    }
    api::exec_with_result(
        client,
        endpoint.mount(mount).name(name).ip(ip).build().unwrap(),
    )
    .await
}

/// Verify SSH OTP details
///
/// See [VerifySSHOTPRequest]
pub async fn verify_otp(
    client: &impl Client,
    mount: &str,
    otp: &str,
) -> Result<VerifySSHOTPResponse, ClientError> {
    let endpoint = VerifySSHOTPRequest::builder()
        .mount(mount)
        .otp(otp)
        .build()
        .unwrap();
    api::exec_with_result(client, endpoint).await
}

pub mod ca {
    use crate::api;
    use crate::api::ssh::requests::{
        DeleteCAInfoRequest, ReadPublicKeyRequest, SignSSHKeyRequest, SignSSHKeyRequestBuilder,
        SubmitCAInfoRequest,
    };
    use crate::api::ssh::responses::{
        ReadPublicKeyResponse, SignSSHKeyResponse, SubmitCAInfoResponse,
    };
    use crate::client::Client;
    use crate::error::ClientError;

    /// Deletes the stored keys for the CA.
    ///
    /// See [DeleteCAInfoRequest]
    pub async fn delete(client: &impl Client, mount: &str) -> Result<(), ClientError> {
        let endpoint = DeleteCAInfoRequest::builder().mount(mount).build().unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Generates CA certificate internally and returns the public key.
    ///
    /// See [SubmitCAInfoRequest]
    pub async fn generate(
        client: &impl Client,
        mount: &str,
    ) -> Result<SubmitCAInfoResponse, ClientError> {
        let endpoint = SubmitCAInfoRequest::builder()
            .mount(mount)
            .generate_signing_key(true)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Reads the public key of the CA.
    ///
    /// See [ReadPublicKeyRequest]
    pub async fn read(
        client: &impl Client,
        mount: &str,
    ) -> Result<ReadPublicKeyResponse, ClientError> {
        let endpoint = ReadPublicKeyRequest::builder()
            .mount(mount)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Signs a public key using the CA certificate.
    ///
    /// See [SignSSHKeyRequest]
    pub async fn sign(
        client: &impl Client,
        mount: &str,
        name: &str,
        public_key: &str,
        opts: Option<&mut SignSSHKeyRequestBuilder>,
    ) -> Result<SignSSHKeyResponse, ClientError> {
        let mut t = SignSSHKeyRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut t)
            .mount(mount)
            .name(name)
            .public_key(public_key)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Sets the private and public key for the CA.
    ///
    /// See [SubmitCAInfoRequest]
    pub async fn set(
        client: &impl Client,
        mount: &str,
        private_key: &str,
        public_key: &str,
    ) -> Result<(), ClientError> {
        let endpoint = SubmitCAInfoRequest::builder()
            .mount(mount)
            .private_key(private_key)
            .public_key(public_key)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }
}

pub mod key {
    use crate::api;
    use crate::api::ssh::requests::{DeleteKeyRequest, SetKeyRequest};
    use crate::client::Client;
    use crate::error::ClientError;

    /// Creates or updates a SSH key
    ///
    /// See [SetKeyRequest]
    pub async fn set(
        client: &impl Client,
        mount: &str,
        name: &str,
        key: &str,
    ) -> Result<(), ClientError> {
        let endpoint = SetKeyRequest::builder()
            .mount(mount)
            .name(name)
            .key(key)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Deletes a SSH key
    ///
    /// See [DeleteKeyRequest]
    pub async fn delete(client: &impl Client, mount: &str, name: &str) -> Result<(), ClientError> {
        let endpoint = DeleteKeyRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }
}

pub mod role {
    use crate::api;
    use crate::api::ssh::requests::ListRolesByIPRequest;
    use crate::api::ssh::responses::ListRolesByIPResponse;
    use crate::api::ssh::{
        requests::{
            DeleteRoleRequest, ListRolesRequest, ReadRoleRequest, SetRoleRequest,
            SetRoleRequestBuilder,
        },
        responses::{ListRolesResponse, ReadRoleResponse},
    };
    use crate::client::Client;
    use crate::error::ClientError;

    /// Deletes a role
    ///
    /// See [DeleteRoleRequest]
    pub async fn delete(client: &impl Client, mount: &str, name: &str) -> Result<(), ClientError> {
        let endpoint = DeleteRoleRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Lists all roles
    ///
    /// See [ListRolesRequest]
    pub async fn list(client: &impl Client, mount: &str) -> Result<ListRolesResponse, ClientError> {
        let endpoint = ListRolesRequest::builder().mount(mount).build().unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Lists all roles by IP
    ///
    /// See [ListRolesByIPRequest]
    pub async fn list_by_ip(
        client: &impl Client,
        mount: &str,
        ip: &str,
    ) -> Result<ListRolesByIPResponse, ClientError> {
        let endpoint = ListRolesByIPRequest::builder()
            .mount(mount)
            .ip(ip)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Reads a role
    ///
    /// See [ReadRoleRequest]
    pub async fn read(
        client: &impl Client,
        mount: &str,
        name: &str,
    ) -> Result<ReadRoleResponse, ClientError> {
        let endpoint = ReadRoleRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Creates or updates a role
    ///
    /// See [SetRoleRequest]
    pub async fn set(
        client: &impl Client,
        mount: &str,
        name: &str,
        opts: Option<&mut SetRoleRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = SetRoleRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut t)
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }
}

pub mod zero {
    use crate::api;
    use crate::api::ssh::requests::{
        ConfigureZeroAddressRolesRequest, DeleteZeroAddressRolesRequest,
        ListZeroAddressRolesRequest,
    };
    use crate::api::ssh::responses::ListZeroAddressRolesResponse;
    use crate::client::Client;
    use crate::error::ClientError;

    /// Deletes all zero-address roles
    ///
    /// See [DeleteZeroAddressRolesRequest]
    pub async fn delete(client: &impl Client, mount: &str) -> Result<(), ClientError> {
        let endpoint = DeleteZeroAddressRolesRequest::builder()
            .mount(mount)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Lists all zero-address roles
    ///
    /// See [ListZeroAddressRolesRequest]
    pub async fn list(
        client: &impl Client,
        mount: &str,
    ) -> Result<ListZeroAddressRolesResponse, ClientError> {
        let endpoint = ListZeroAddressRolesRequest::builder()
            .mount(mount)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Sets zero-address roles
    ///
    /// See [ConfigureZeroAddressRolesRequest]
    pub async fn set(
        client: &impl Client,
        mount: &str,
        roles: Vec<String>,
    ) -> Result<(), ClientError> {
        let endpoint = ConfigureZeroAddressRolesRequest::builder()
            .mount(mount)
            .roles(roles)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }
}
