use crate::{
    api::{
        self,
        auth::aws::requests::{Ec2LoginRequest, IamLoginRequest},
        AuthInfo,
    },
    client::Client,
    error::ClientError,
};

// See [IamLoginRequest]
pub async fn iam_login(
    client: &impl Client,
    mount: &str,
    iam_http_request_method: &str,
    iam_request_url: &str,
    iam_request_headers: &str,
    iam_request_body: &str,
    role: Option<&str>,
) -> Result<AuthInfo, ClientError> {
    let mut endpoint = IamLoginRequest::builder();

    endpoint
        .iam_http_request_method(iam_http_request_method)
        .iam_request_url(iam_request_url)
        .iam_request_headers(iam_request_headers)
        .iam_request_body(iam_request_body);

    if let Some(role) = role {
        endpoint.role(role);
    }

    api::auth(client, endpoint.mount(mount).build().unwrap()).await
}

// See [Ec2LoginRequest]
pub async fn ec2_login(
    client: &impl Client,
    mount: &str,
    pkcs7: &str,
    nonce: Option<&str>,
    role: Option<&str>,
) -> Result<AuthInfo, ClientError> {
    let mut endpoint = Ec2LoginRequest::builder();

    if let Some(nonce) = nonce {
        endpoint.nonce(nonce);
    }
    if let Some(role) = role {
        endpoint.role(role);
    }
    api::auth(client, endpoint.mount(mount).pkcs7(pkcs7).build().unwrap()).await
}

// modules structure depends on URI,
// e.g /auth/aws/config/client -> `mod config { mod client { fn set, fn read, fn delete }}`
pub mod config {
    pub mod client {
        use crate::{
            api::{
                self,
                auth::aws::{
                    requests::{
                        ConfigureClientRequest, ConfigureClientRequestBuilder,
                        DeleteClientConfigurationRequest, ReadClientConfigurationRequest,
                        RotateRootCredentialsRequest,
                    },
                    responses::{ReadClientConfigurationResponse, RotateRootCredentialsResponse},
                },
            },
            client::Client,
            error::ClientError,
        };

        /// Configures the credentials required to perform API calls to AWS as well as custom endpoints to talk to AWS APIs.
        ///
        /// See [ConfigureClientRequest]
        pub async fn set(
            client: &impl Client,
            mount: &str,
            opts: Option<&mut ConfigureClientRequestBuilder>,
        ) -> Result<(), ClientError> {
            let mut t = ConfigureClientRequest::builder();
            let endpoint = opts.unwrap_or(&mut t).mount(mount).build().unwrap();
            api::exec_with_empty(client, endpoint).await
        }

        /// Returns the previously configured AWS access credentials.
        ///
        /// See [ReadClientConfigurationResponse]
        pub async fn read(
            client: &impl Client,
            mount: &str,
        ) -> Result<ReadClientConfigurationResponse, ClientError> {
            let endpoint = ReadClientConfigurationRequest::builder()
                .mount(mount)
                .build()
                .unwrap();
            api::exec_with_result(client, endpoint).await
        }

        /// Deletes the previously configured AWS access credentials.
        ///
        /// See [DeleteClientConfigurationRequest]
        pub async fn delete(client: &impl Client, mount: &str) -> Result<(), ClientError> {
            let endpoint = DeleteClientConfigurationRequest::builder()
                .mount(mount)
                .build()
                .unwrap();
            api::exec_with_empty(client, endpoint).await
        }

        /// When you have configured Vault with static credentials, you can use this function to have Vault rotate the access key it used.
        ///
        /// See [RotateRootCredentialsRequest]
        pub async fn rotate_root_credentials(
            client: &impl Client,
            mount: &str,
        ) -> Result<RotateRootCredentialsResponse, ClientError> {
            let endpoint = RotateRootCredentialsRequest::builder()
                .mount(mount)
                .build()
                .unwrap();
            api::exec_with_result(client, endpoint).await
        }
    }

    pub mod identity {
        use crate::{
            api::{
                self,
                auth::aws::{
                    requests::{
                        ConfigureIdentityRequest, ConfigureIdentityRequestBuilder,
                        ReadIdentityConfigurationRequest,
                    },
                    responses::ReadIdentityConfigurationResponse,
                },
            },
            client::Client,
            error::ClientError,
        };

        /// This configures the way that Vault interacts with the Identity store.
        ///
        /// See [ConfigureIdentityRequest]
        pub async fn set(
            client: &impl Client,
            mount: &str,
            opts: Option<&mut ConfigureIdentityRequestBuilder>,
        ) -> Result<(), ClientError> {
            let mut t = ConfigureIdentityRequest::builder();
            let endpoint = opts.unwrap_or(&mut t).mount(mount).build().unwrap();
            api::exec_with_empty(client, endpoint).await
        }

        /// Returns the previously configured Identity integration configuration
        ///
        /// See [ReadIdentityConfigurationResponse]
        pub async fn read(
            client: &impl Client,
            mount: &str,
        ) -> Result<ReadIdentityConfigurationResponse, ClientError> {
            let endpoint = ReadIdentityConfigurationRequest::builder()
                .mount(mount)
                .build()
                .unwrap();
            api::exec_with_result(client, endpoint).await
        }
    }

    pub mod certificate {
        use crate::{
            api::{
                self,
                auth::aws::{
                    requests::{
                        CreateCertificateConfigurationRequest,
                        CreateCertificateConfigurationRequestBuilder,
                        DeleteCertificateConfigurationRequest,
                        ListCertificateConfigurationsRequest, ReadCertificateConfigurationRequest,
                    },
                    responses::{
                        ListCertificateConfigurationsResponse, ReadCertificateConfigurationResponse,
                    },
                },
            },
            client::Client,
            error::ClientError,
        };

        /// Registers an AWS public key to be used to verify the instance identity documents.
        ///
        /// See [CreateCertificateConfigurationRequest]
        pub async fn create(
            client: &impl Client,
            mount: &str,
            cert_name: &str,
            aws_public_cert: &str,
            opts: Option<&mut CreateCertificateConfigurationRequestBuilder>,
        ) -> Result<(), ClientError> {
            let mut t = CreateCertificateConfigurationRequest::builder();
            let endpoint = opts
                .unwrap_or(&mut t)
                .mount(mount)
                .cert_name(cert_name)
                .aws_public_cert(aws_public_cert)
                .build()
                .unwrap();
            api::exec_with_empty(client, endpoint).await
        }

        /// Returns the previously configured AWS public key.
        ///
        /// See [ReadCertificateConfigurationResponse]
        pub async fn read(
            client: &impl Client,
            mount: &str,
            cert_name: &str,
        ) -> Result<ReadCertificateConfigurationResponse, ClientError> {
            let endpoint = ReadCertificateConfigurationRequest::builder()
                .mount(mount)
                .cert_name(cert_name)
                .build()
                .unwrap();
            api::exec_with_result(client, endpoint).await
        }

        /// Removes the previously configured AWS public key.
        ///
        /// See [DeleteCertificateConfigurationRequest]
        pub async fn delete(
            client: &impl Client,
            mount: &str,
            cert_name: &str,
        ) -> Result<(), ClientError> {
            let endpoint = DeleteCertificateConfigurationRequest::builder()
                .mount(mount)
                .cert_name(cert_name)
                .build()
                .unwrap();
            api::exec_with_empty(client, endpoint).await
        }

        /// Lists all the AWS public certificates that are registered with the method.
        ///
        /// See [ListCertificateConfigurationsResponse]
        pub async fn list(
            client: &impl Client,
            mount: &str,
        ) -> Result<ListCertificateConfigurationsResponse, ClientError> {
            let endpoint = ListCertificateConfigurationsRequest::builder()
                .mount(mount)
                .build()
                .unwrap();
            api::exec_with_result(client, endpoint).await
        }
    }

    pub mod sts {
        use crate::{
            api::{
                self,
                auth::aws::{
                    requests::{
                        CreateStsRoleRequest, DeleteStsRoleRequest, ListStsRolesRequest,
                        ReadStsRoleRequest,
                    },
                    responses::{ListStsRolesResponse, ReadStsRoleResponse},
                },
            },
            client::Client,
            error::ClientError,
        };

        /// Allows the explicit association of STS roles to satellite AWS accounts.
        ///
        /// See [CreateStsRoleRequest]
        pub async fn create(
            client: &impl Client,
            mount: &str,
            account_id: &str,
            sts_role: &str,
        ) -> Result<(), ClientError> {
            let endpoint = CreateStsRoleRequest::builder()
                .mount(mount)
                .account_id(account_id)
                .sts_role(sts_role)
                .build()
                .unwrap();
            api::exec_with_empty(client, endpoint).await
        }

        /// Returns the previously configured STS role.
        ///
        /// See [ReadStsRoleResponse]
        pub async fn read(
            client: &impl Client,
            mount: &str,
            account_id: &str,
        ) -> Result<ReadStsRoleResponse, ClientError> {
            let endpoint = ReadStsRoleRequest::builder()
                .mount(mount)
                .account_id(account_id)
                .build()
                .unwrap();
            api::exec_with_result(client, endpoint).await
        }

        /// Lists all the AWS Account IDs for which an STS role is registered.
        ///
        /// See [ListStsRolesResponse]
        pub async fn list(
            client: &impl Client,
            mount: &str,
        ) -> Result<ListStsRolesResponse, ClientError> {
            let endpoint = ListStsRolesRequest::builder().mount(mount).build().unwrap();
            api::exec_with_result(client, endpoint).await
        }

        /// Deletes a previously configured AWS account/STS role association.
        ///
        /// See [DeleteStsRoleRequest]
        pub async fn delete(
            client: &impl Client,
            mount: &str,
            account_id: &str,
        ) -> Result<(), ClientError> {
            let endpoint = DeleteStsRoleRequest::builder()
                .mount(mount)
                .account_id(account_id)
                .build()
                .unwrap();
            api::exec_with_empty(client, endpoint).await
        }
    }

    pub mod tidy {
        pub mod identity_access_list {
            use crate::{
                api::{
                    self,
                    auth::aws::{
                        requests::{
                            ConfigureIdentityAccessListTidyOperationRequest,
                            ConfigureIdentityAccessListTidyOperationRequestBuilder,
                            DeleteIdentityAccessListTidySettingsRequest,
                            ReadIdentityAccessListTidySettingsRequest,
                        },
                        responses::ReadIdentityAccessListTidySettingsResponse,
                    },
                },
                client::Client,
                error::ClientError,
            };

            /// Configures the periodic tidying operation of the access listed identity entries.
            ///
            /// See [ConfigureIdentityAccessListTidyOperationRequest]
            pub async fn set(
                client: &impl Client,
                mount: &str,
                opts: Option<&mut ConfigureIdentityAccessListTidyOperationRequestBuilder>,
            ) -> Result<(), ClientError> {
                let mut t = ConfigureIdentityAccessListTidyOperationRequest::builder();
                let endpoint = opts.unwrap_or(&mut t).mount(mount).build().unwrap();
                api::exec_with_empty(client, endpoint).await
            }

            /// Returns the previously configured periodic access list tidying settings.
            ///
            /// See [ReadIdentityAccessListTidySettingsResponse]
            pub async fn read(
                client: &impl Client,
                mount: &str,
            ) -> Result<ReadIdentityAccessListTidySettingsResponse, ClientError> {
                let endpoint = ReadIdentityAccessListTidySettingsRequest::builder()
                    .mount(mount)
                    .build()
                    .unwrap();
                api::exec_with_result(client, endpoint).await
            }

            /// Deletes the previously configured periodic access list tidying settings.
            ///
            /// See [DeleteIdentityAccessListTidySettingsRequest]
            pub async fn delete(client: &impl Client, mount: &str) -> Result<(), ClientError> {
                let endpoint = DeleteIdentityAccessListTidySettingsRequest::builder()
                    .mount(mount)
                    .build()
                    .unwrap();
                api::exec_with_empty(client, endpoint).await
            }
        }

        pub mod role_tag_deny_list {
            use crate::{
                api::{
                    self,
                    auth::aws::{
                        requests::{
                            ConfigureRoleTagDenyListTidyOperationRequest,
                            ConfigureRoleTagDenyListTidyOperationRequestBuilder,
                            DeleteRoleTagDenyListTidySettingsRequest,
                            ReadRoleTagDenyListTidySettingsRequest,
                        },
                        responses::ReadRoleTagDenyListTidySettingsResponse,
                    },
                },
                client::Client,
                error::ClientError,
            };

            /// Configures the periodic tidying operation of the deny listed role tag entries.
            ///
            /// See [ConfigureRoleTagDenyListTidyOperationRequest]
            pub async fn set(
                client: &impl Client,
                mount: &str,
                opts: Option<&mut ConfigureRoleTagDenyListTidyOperationRequestBuilder>,
            ) -> Result<(), ClientError> {
                let mut t = ConfigureRoleTagDenyListTidyOperationRequest::builder();
                let endpoint = opts.unwrap_or(&mut t).mount(mount).build().unwrap();
                api::exec_with_empty(client, endpoint).await
            }

            /// Returns the previously configured periodic deny list tidying settings.
            ///
            /// See [ReadRoleTagDenyListTidySettingsResponse]
            pub async fn read(
                client: &impl Client,
                mount: &str,
            ) -> Result<ReadRoleTagDenyListTidySettingsResponse, ClientError> {
                let endpoint = ReadRoleTagDenyListTidySettingsRequest::builder()
                    .mount(mount)
                    .build()
                    .unwrap();
                api::exec_with_result(client, endpoint).await
            }

            /// Deletes the previously configured periodic access list tidying settings.
            ///
            /// See [DeleteRoleTagDenyListTidySettingsRequest]
            pub async fn delete(client: &impl Client, mount: &str) -> Result<(), ClientError> {
                let endpoint = DeleteRoleTagDenyListTidySettingsRequest::builder()
                    .mount(mount)
                    .build()
                    .unwrap();
                api::exec_with_empty(client, endpoint).await
            }
        }
    }
}

pub mod role {
    use crate::{
        api::{
            self,
            auth::aws::{
                requests::{
                    CreateRoleRequest, CreateRoleRequestBuilder, CreateRoleTagRequest,
                    CreateRoleTagRequestBuilder, DeleteRoleRequest, ListRolesRequest,
                    ReadRoleRequest,
                },
                responses::{CreateRoleTagResponse, ListRolesResponse, ReadRoleResponse},
            },
        },
        client::Client,
        error::ClientError,
    };

    /// Registers a role in the method
    ///
    /// See [CreateRoleRequest]
    pub async fn create(
        client: &impl Client,
        mount: &str,
        role: &str,
        opts: Option<&mut CreateRoleRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = CreateRoleRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut t)
            .mount(mount)
            .role(role)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Returns the previously registered role configuration
    ///
    /// See [ReadRoleResponse]
    pub async fn read(
        client: &impl Client,
        mount: &str,
        role: &str,
    ) -> Result<ReadRoleResponse, ClientError> {
        let endpoint = ReadRoleRequest::builder()
            .mount(mount)
            .role(role)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Lists all the roles that are registered with the method
    ///
    /// See [ListRolesResponse]
    pub async fn list(client: &impl Client, mount: &str) -> Result<ListRolesResponse, ClientError> {
        let endpoint = ListRolesRequest::builder().mount(mount).build().unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Deletes the previously registered role
    ///
    /// See [DeleteRoleRequest]
    pub async fn delete(client: &impl Client, mount: &str, role: &str) -> Result<(), ClientError> {
        let endpoint = DeleteRoleRequest::builder()
            .mount(mount)
            .role(role)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Creates a role tag on the role
    ///
    /// See [CreateRoleTagRequest]
    pub async fn create_tag(
        client: &impl Client,
        mount: &str,
        role: &str,
        opts: Option<&mut CreateRoleTagRequestBuilder>,
    ) -> Result<CreateRoleTagResponse, ClientError> {
        let mut t = CreateRoleTagRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut t)
            .mount(mount)
            .role(role)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }
}

pub mod role_tag_deny_list {
    use crate::{
        api::{
            self,
            auth::aws::{
                requests::{
                    DeleteDenyListTagsRequest, ListDenyListTagsRequest,
                    PlaceRoleTagsInDenyListRequest, ReadRoleTagDenyListRequest,
                    TidyDenyListTagsRequest, TidyDenyListTagsRequestBuilder,
                },
                responses::{ListDenyListTagsResponse, ReadRoleTagDenyListResponse},
            },
        },
        client::Client,
        error::ClientError,
    };

    /// Places a valid role tag in a deny list
    ///
    /// See [PlaceRoleTagsInDenyListRequest]
    pub async fn create(
        client: &impl Client,
        mount: &str,
        tag_value: &str,
    ) -> Result<(), ClientError> {
        let endpoint = PlaceRoleTagsInDenyListRequest::builder()
            .mount(mount)
            .tag_value(tag_value)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Returns the deny list entry of a previously deny listed role tag.
    ///
    /// See [ReadRoleTagDenyListResponse]
    pub async fn read(
        client: &impl Client,
        mount: &str,
        tag_value: &str,
    ) -> Result<ReadRoleTagDenyListResponse, ClientError> {
        let endpoint = ReadRoleTagDenyListRequest::builder()
            .mount(mount)
            .tag_value(tag_value)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Lists all the role tags that are deny listed
    ///
    /// See [ListDenyListTagsResponse]
    pub async fn list(
        client: &impl Client,
        mount: &str,
    ) -> Result<ListDenyListTagsResponse, ClientError> {
        let endpoint = ListDenyListTagsRequest::builder()
            .mount(mount)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Deletes a deny listed role tag
    ///
    /// See [DeleteDenyListTagsRequest]
    pub async fn delete(
        client: &impl Client,
        mount: &str,
        tag_value: &str,
    ) -> Result<(), ClientError> {
        let endpoint = DeleteDenyListTagsRequest::builder()
            .mount(mount)
            .tag_value(tag_value)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Cleans up the entries in the deny listed based on expiration time on the entry and safety_buffer.
    ///
    /// See [TidyDenyListTagsRequest]
    pub async fn tidy(
        client: &impl Client,
        mount: &str,
        opts: Option<&mut TidyDenyListTagsRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = TidyDenyListTagsRequest::builder();
        let endpoint = opts.unwrap_or(&mut t).mount(mount).build().unwrap();
        api::exec_with_empty(client, endpoint).await
    }
}

pub mod identity_access_list {
    use crate::{
        api::{
            self,
            auth::aws::{
                requests::{
                    DeleteIdentityAccessListEntriesRequest, ListIdentityAccessListEntriesRequest,
                    ReadIdentityAccessListInformationRequest, TidyIdentityAccessListEntriesRequest,
                    TidyIdentityAccessListEntriesRequestBuilder,
                },
                responses::{
                    ListIdentityAccessListEntriesResponse,
                    ReadIdentityAccessListInformationResponse,
                },
            },
        },
        client::Client,
        error::ClientError,
    };

    /// Returns an entry in the identity access list.
    ///
    /// See [ReadIdentityAccessListInformationResponse]
    pub async fn read(
        client: &impl Client,
        mount: &str,
        instance_id: &str,
    ) -> Result<ReadIdentityAccessListInformationResponse, ClientError> {
        let endpoint = ReadIdentityAccessListInformationRequest::builder()
            .mount(mount)
            .instance_id(instance_id)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Deletes a cache of the successful login from an instance
    ///
    /// See [DeleteIdentityAccessListEntriesRequest]
    pub async fn delete(
        client: &impl Client,
        mount: &str,
        instance_id: &str,
    ) -> Result<(), ClientError> {
        let endpoint = DeleteIdentityAccessListEntriesRequest::builder()
            .mount(mount)
            .instance_id(instance_id)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    /// Lists all the instance IDs that are in the access list of successful logins
    ///
    /// See [ListIdentityAccessListEntriesResponse]
    pub async fn list(
        client: &impl Client,
        mount: &str,
    ) -> Result<ListIdentityAccessListEntriesResponse, ClientError> {
        let endpoint = ListIdentityAccessListEntriesRequest::builder()
            .mount(mount)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Cleans up the entries in the access list based on expiration time andsafety_buffer
    ///
    /// See [TidyIdentityAccessListEntriesRequest]
    pub async fn tidy(
        client: &impl Client,
        mount: &str,
        opts: Option<&mut TidyIdentityAccessListEntriesRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = TidyIdentityAccessListEntriesRequest::builder();
        let endpoint = opts.unwrap_or(&mut t).mount(mount).build().unwrap();
        api::exec_with_empty(client, endpoint).await
    }
}
