use crate::{
    api::{
        self,
        auth::approle::requests::{LoginWithApproleRequest, TidyRequest},
        AuthInfo,
    },
    client::Client,
    error::ClientError,
};

// Fetch a token with policies in corresponding AppRole.
//
// See [LoginWithApproleRequest]
pub async fn login(
    client: &impl Client,
    mount: &str,
    role_id: &str,
    secret_id: &str,
) -> Result<AuthInfo, ClientError> {
    let endpoint = LoginWithApproleRequest::builder()
        .mount(mount)
        .role_id(role_id)
        .secret_id(secret_id)
        .build()
        .unwrap();
    api::auth(client, endpoint).await
}

/// Tidy's up the AppRole backend.
///
/// See [TidyRequest]
pub async fn tidy(client: &impl Client, mount: &str) -> Result<(), ClientError> {
    let endpoint = TidyRequest::builder().mount(mount).build().unwrap();
    api::exec_with_empty_result(client, endpoint).await
}

pub mod role {
    use crate::api;
    use crate::api::auth::approle::requests::UpdateRoleIDRequest;
    use crate::api::auth::approle::{
        requests::{
            DeleteAppRoleRequest, ListRolesRequest, ReadAppRoleRequest, ReadRoleIDRequest,
            SetAppRoleRequest, SetAppRoleRequestBuilder,
        },
        responses::{ListRolesResponse, ReadAppRoleResponse, ReadRoleIDResponse},
    };
    use crate::client::Client;
    use crate::error::ClientError;

    /// Lists all AppRoles.
    ///
    /// See [ListRolesRequest]
    pub async fn list(client: &impl Client, mount: &str) -> Result<ListRolesResponse, ClientError> {
        let endpoint = ListRolesRequest::builder().mount(mount).build().unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Reads properties of an AppRole.
    ///
    /// See [ReadAppRoleRequest]
    pub async fn read(
        client: &impl Client,
        mount: &str,
        role_name: &str,
    ) -> Result<ReadAppRoleResponse, ClientError> {
        let endpoint = ReadAppRoleRequest::builder()
            .mount(mount)
            .role_name(role_name)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Creates or updates an AppRole.
    ///
    /// See [SetAppRoleRequest]
    pub async fn set(
        client: &impl Client,
        mount: &str,
        role_name: &str,
        opts: Option<&mut SetAppRoleRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = SetAppRoleRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut t)
            .mount(mount)
            .role_name(role_name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Deletes an existing AppRole.
    ///
    /// See [DeleteAppRoleRequest]
    pub async fn delete(
        client: &impl Client,
        mount: &str,
        role_name: &str,
    ) -> Result<(), ClientError> {
        let endpoint = DeleteAppRoleRequest::builder()
            .mount(mount)
            .role_name(role_name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Reads the RoleID of an existing AppRole.
    ///
    /// See [ReadRoleIDRequest]
    pub async fn read_id(
        client: &impl Client,
        mount: &str,
        role_name: &str,
    ) -> Result<ReadRoleIDResponse, ClientError> {
        let endpoint = ReadRoleIDRequest::builder()
            .mount(mount)
            .role_name(role_name)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Updates the Role ID of an AppRole.
    ///
    /// See [UpdateRoleIDRequest]
    pub async fn update_id(
        client: &impl Client,
        mount: &str,
        role_name: &str,
        role_id: &str,
    ) -> Result<(), ClientError> {
        let endpoint = UpdateRoleIDRequest::builder()
            .mount(mount)
            .role_name(role_name)
            .role_id(role_id)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    pub mod secret {
        use crate::api;
        use crate::api::auth::approle::requests::{
            CreateCustomSecretIDRequest, CreateCustomSecretIDRequestBuilder,
            DeleteSecretIDAccessorRequest, DeleteSecretIDRequest, GenerateNewSecretIDRequest,
            GenerateNewSecretIDRequestBuilder, ListSecretIDRequest, ReadSecretIDAccessorRequest,
            ReadSecretIDRequest,
        };
        use crate::api::auth::approle::responses::{
            CreateCustomSecretIDResponse, GenerateNewSecretIDResponse, ListSecretIDResponse,
            ReadSecretIDResponse,
        };
        use crate::client::Client;
        use crate::error::ClientError;

        /// Creates a custom secret ID.
        ///
        /// See [CreateCustomSecretIDRequest]
        pub async fn custom(
            client: &impl Client,
            mount: &str,
            role_name: &str,
            secret_id: &str,
            opts: Option<&mut CreateCustomSecretIDRequestBuilder>,
        ) -> Result<CreateCustomSecretIDResponse, ClientError> {
            let mut t = CreateCustomSecretIDRequest::builder();
            let endpoint = opts
                .unwrap_or(&mut t)
                .mount(mount)
                .role_name(role_name)
                .secret_id(secret_id)
                .build()
                .unwrap();
            api::exec_with_result(client, endpoint).await
        }

        /// Deletes an AppRole secret ID.
        ///
        /// See [DeleteSecretIDRequest]
        pub async fn delete(
            client: &impl Client,
            mount: &str,
            role_name: &str,
            secret_id: &str,
        ) -> Result<(), ClientError> {
            let endpoint = DeleteSecretIDRequest::builder()
                .mount(mount)
                .role_name(role_name)
                .secret_id(secret_id)
                .build()
                .unwrap();
            api::exec_with_empty(client, endpoint).await
        }

        /// Deletes an AppRole secret ID by accessor.
        ///
        /// See [DeleteSecretIDAccessorRequest]
        pub async fn delete_accessor(
            client: &impl Client,
            mount: &str,
            role_name: &str,
            secret_id_accessor: &str,
        ) -> Result<(), ClientError> {
            let endpoint = DeleteSecretIDAccessorRequest::builder()
                .mount(mount)
                .role_name(role_name)
                .secret_id_accessor(secret_id_accessor)
                .build()
                .unwrap();
            api::exec_with_empty(client, endpoint).await
        }

        /// Generates and issues a new SecretID on an existing AppRole.
        ///
        /// See [GenerateNewSecretIDRequest]
        pub async fn generate(
            client: &impl Client,
            mount: &str,
            role_name: &str,
            opts: Option<&mut GenerateNewSecretIDRequestBuilder>,
        ) -> Result<GenerateNewSecretIDResponse, ClientError> {
            let mut t = GenerateNewSecretIDRequest::builder();
            let endpoint = opts
                .unwrap_or(&mut t)
                .mount(mount)
                .role_name(role_name)
                .build()
                .unwrap();
            api::exec_with_result(client, endpoint).await
        }

        /// Lists ApplRole secret IDs.
        ///
        /// See [ListSecretIDRequest]
        pub async fn list(
            client: &impl Client,
            mount: &str,
            role_name: &str,
        ) -> Result<ListSecretIDResponse, ClientError> {
            let endpoint = ListSecretIDRequest::builder()
                .mount(mount)
                .role_name(role_name)
                .build()
                .unwrap();
            api::exec_with_result(client, endpoint).await
        }

        /// Reads an AppleRole secret ID.
        ///
        /// See [ReadSecretIDRequest]
        pub async fn read(
            client: &impl Client,
            mount: &str,
            role_name: &str,
            secret_id: &str,
        ) -> Result<ReadSecretIDResponse, ClientError> {
            let endpoint = ReadSecretIDRequest::builder()
                .mount(mount)
                .role_name(role_name)
                .secret_id(secret_id)
                .build()
                .unwrap();
            api::exec_with_result(client, endpoint).await
        }

        /// Reads an AppleRole secret ID by accessor.
        ///
        /// See [ReadSecretIDAccessorRequest]
        pub async fn read_accessor(
            client: &impl Client,
            mount: &str,
            role_name: &str,
            secret_id_accessor: &str,
        ) -> Result<ReadSecretIDResponse, ClientError> {
            let endpoint = ReadSecretIDAccessorRequest::builder()
                .mount(mount)
                .role_name(role_name)
                .secret_id_accessor(secret_id_accessor)
                .build()
                .unwrap();
            api::exec_with_result(client, endpoint).await
        }
    }
}
