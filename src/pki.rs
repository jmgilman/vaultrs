pub mod cert {
    use crate::api;
    use crate::api::pki::responses::{
        GenerateCertificateResponse, ReadCertificateResponse, RevokeCertificateResponse,
    };
    use crate::error::ClientError;
    use crate::{
        api::pki::requests::{
            GenerateCertificateRequest, GenerateCertificateRequestBuilder, ListCertificatesRequest,
            ReadCertificateRequest, RevokeCertificateRequest, TidyRequest,
        },
        client::VaultClient,
    };

    /// Generates a certificate using the given role and options
    ///
    /// See [GenerateCertificateRequest]
    pub fn generate(
        client: &VaultClient,
        mount: &str,
        role: &str,
        opts: Option<&mut GenerateCertificateRequestBuilder>,
    ) -> Result<GenerateCertificateResponse, ClientError> {
        let mut t = GenerateCertificateRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut t)
            .mount(mount)
            .role(role)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint)
    }

    /// Lists all certificates
    ///
    /// See [ListCertificatesRequest]
    pub fn list(client: &VaultClient, mount: &str) -> Result<Vec<String>, ClientError> {
        let endpoint = ListCertificatesRequest::builder()
            .mount(mount)
            .build()
            .unwrap();
        Ok(api::exec_with_result(client, endpoint)?.keys)
    }

    /// Read a certificate using its serial
    ///
    /// See [ReadCertificateRequest]
    pub fn read(
        client: &VaultClient,
        mount: &str,
        serial: &str,
    ) -> Result<ReadCertificateResponse, ClientError> {
        let endpoint = ReadCertificateRequest::builder()
            .mount(mount)
            .serial(serial)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint)
    }

    /// Revokes a certificate using its serial
    ///
    /// See [RevokeCertificateRequest]
    pub fn revoke(
        client: &VaultClient,
        mount: &str,
        serial: &str,
    ) -> Result<RevokeCertificateResponse, ClientError> {
        let endpoint = RevokeCertificateRequest::builder()
            .mount(mount)
            .serial_number(serial)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint)
    }

    /// Tidy's up the certificate backend
    ///
    /// See [TidyRequest]
    pub fn tidy(client: &VaultClient, mount: &str) -> Result<(), ClientError> {
        let endpoint = TidyRequest::builder().mount(mount).build().unwrap();
        api::exec_with_empty(client, endpoint)
    }

    pub mod ca {
        use crate::api;
        use crate::api::pki::responses::SignSelfIssuedResponse;
        use crate::{
            api::pki::{
                requests::{
                    DeleteRootRequest, GenerateRootRequest, GenerateRootRequestBuilder,
                    SignCertificateRequest, SignCertificateRequestBuilder, SignIntermediateRequest,
                    SignIntermediateRequestBuilder, SignSelfIssuedRequest, SubmitCARequest,
                },
                responses::{
                    GenerateRootResponse, SignCertificateResponse, SignIntermediateResponse,
                },
            },
            client::VaultClient,
            error::ClientError,
        };

        /// Delete's the root CA
        ///
        /// See [DeleteRootRequest]
        pub fn delete(client: &VaultClient, mount: &str) -> Result<(), ClientError> {
            let endpoint = DeleteRootRequest::builder().mount(mount).build().unwrap();
            api::exec_with_empty(client, endpoint)
        }

        /// Generates a new root CA
        ///
        /// See [GenerateRootRequest]
        pub fn generate(
            client: &VaultClient,
            mount: &str,
            cert_type: &str,
            opts: Option<&mut GenerateRootRequestBuilder>,
        ) -> Result<Option<GenerateRootResponse>, ClientError> {
            let mut t = GenerateRootRequest::builder();
            let endpoint = opts
                .unwrap_or(&mut t)
                .mount(mount)
                .cert_type(cert_type)
                .build()
                .unwrap();
            api::exec_with_result(client, endpoint)
        }

        /// Signs a certificate using the root CA
        ///
        /// See [SignCertificateRequest]
        pub fn sign(
            client: &VaultClient,
            mount: &str,
            role: &str,
            csr: &str,
            common_name: &str,
            opts: Option<&mut SignCertificateRequestBuilder>,
        ) -> Result<SignCertificateResponse, ClientError> {
            let mut t = SignCertificateRequest::builder();
            let endpoint = opts
                .unwrap_or(&mut t)
                .mount(mount)
                .role(role)
                .csr(csr)
                .common_name(common_name)
                .build()
                .unwrap();
            api::exec_with_result(client, endpoint)
        }

        /// Signs an intermediate CA using the root CA
        ///
        /// See [SignIntermediateRequest]
        pub fn sign_intermediate(
            client: &VaultClient,
            mount: &str,
            csr: &str,
            common_name: &str,
            opts: Option<&mut SignIntermediateRequestBuilder>,
        ) -> Result<SignIntermediateResponse, ClientError> {
            let mut t = SignIntermediateRequest::builder();
            let endpoint = opts
                .unwrap_or(&mut t)
                .mount(mount)
                .csr(csr)
                .common_name(common_name)
                .build()
                .unwrap();
            api::exec_with_result(client, endpoint)
        }

        /// Signs a self issued certificate using the root CA
        ///
        /// See [SignSelfIssuedRequest]
        pub fn sign_self_issued(
            client: &VaultClient,
            mount: &str,
            certificate: &str,
        ) -> Result<SignSelfIssuedResponse, ClientError> {
            let endpoint = SignSelfIssuedRequest::builder()
                .mount(mount)
                .certificate(certificate)
                .build()
                .unwrap();
            api::exec_with_result(client, endpoint)
        }

        /// Configures the root CA
        ///
        /// See [SubmitCARequest]
        pub fn submit(
            client: &VaultClient,
            mount: &str,
            pem_bundle: &str,
        ) -> Result<(), ClientError> {
            let endpoint = SubmitCARequest::builder()
                .mount(mount)
                .pem_bundle(pem_bundle)
                .build()
                .unwrap();
            api::exec_with_empty(client, endpoint)
        }

        pub mod int {
            use crate::api;
            use crate::{
                api::pki::{
                    requests::{
                        GenerateIntermediateRequest, GenerateIntermediateRequestBuilder,
                        SetSignedIntermediateRequest,
                    },
                    responses::GenerateIntermediateResponse,
                },
                client::VaultClient,
                error::ClientError,
            };

            /// Generates an intermediate CA
            ///
            /// See [GenerateIntermediateRequest]
            pub fn generate(
                client: &VaultClient,
                mount: &str,
                cert_type: &str,
                common_name: &str,
                opts: Option<&mut GenerateIntermediateRequestBuilder>,
            ) -> Result<GenerateIntermediateResponse, ClientError> {
                let mut t = GenerateIntermediateRequest::builder();
                let endpoint = opts
                    .unwrap_or(&mut t)
                    .mount(mount)
                    .cert_type(cert_type)
                    .common_name(common_name)
                    .build()
                    .unwrap();
                api::exec_with_result(client, endpoint)
            }

            /// Sets the signed CA certificate
            ///
            /// See [SetSignedIntermediateRequest]
            pub fn set_signed(
                client: &VaultClient,
                mount: &str,
                certificate: &str,
            ) -> Result<(), ClientError> {
                let endpoint = SetSignedIntermediateRequest::builder()
                    .mount(mount)
                    .certificate(certificate)
                    .build()
                    .unwrap();
                api::exec_with_empty(client, endpoint)
            }
        }
    }

    pub mod crl {
        use crate::api::pki::{
            requests::{
                ReadCRLConfigRequest, RotateCRLsRequest, SetCRLConfigRequest,
                SetCRLConfigRequestBuilder,
            },
            responses::{ReadCRLConfigResponse, RotateCRLsResponse},
        };
        use crate::api::{self, exec_with_empty};
        use crate::client::VaultClient;
        use crate::error::ClientError;

        /// Rotates the CRL
        ///
        /// See [RotateCRLsRequest]
        pub fn rotate(
            client: &VaultClient,
            mount: &str,
        ) -> Result<RotateCRLsResponse, ClientError> {
            let endpoint = RotateCRLsRequest::builder().mount(mount).build().unwrap();
            api::exec_with_result(client, endpoint)
        }

        /// Reads the CRL configuration
        ///
        /// See [ReadCRLConfigRequest]
        pub fn read_config(
            client: &VaultClient,
            mount: &str,
        ) -> Result<ReadCRLConfigResponse, ClientError> {
            let endpoint = ReadCRLConfigRequest::builder()
                .mount(mount)
                .build()
                .unwrap();
            api::exec_with_result(client, endpoint)
        }

        /// Sets the CRL configuration
        ///
        /// See [SetCRLConfigRequest]
        pub fn set_config(
            client: &VaultClient,
            mount: &str,
            opts: Option<&mut SetCRLConfigRequestBuilder>,
        ) -> Result<(), ClientError> {
            let mut t = SetCRLConfigRequest::builder();
            let endpoint = opts.unwrap_or(&mut t).mount(mount).build().unwrap();
            exec_with_empty(client, endpoint)
        }
    }

    pub mod urls {
        use crate::api;
        use crate::api::pki::{
            requests::{ReadURLsRequest, SetURLsRequest, SetURLsRequestBuilder},
            responses::ReadURLsResponse,
        };
        use crate::client::VaultClient;
        use crate::error::ClientError;

        /// Reads the configured certificate URLs
        ///
        /// See [ReadURLsRequest]
        pub fn read(client: &VaultClient, mount: &str) -> Result<ReadURLsResponse, ClientError> {
            let endpoint = ReadURLsRequest::builder().mount(mount).build().unwrap();
            api::exec_with_result(client, endpoint)
        }

        /// Sets the configured certificate URLs
        ///
        /// See [SetURLsRequest]
        pub fn set(
            client: &VaultClient,
            mount: &str,
            opts: Option<&mut SetURLsRequestBuilder>,
        ) -> Result<(), ClientError> {
            let mut t = SetURLsRequest::builder();
            let endpoint = opts.unwrap_or(&mut t).mount(mount).build().unwrap();
            api::exec_with_empty(client, endpoint)
        }
    }
}

pub mod role {
    use crate::api;
    use crate::api::pki::{
        requests::{
            DeleteRoleRequest, ListRolesRequest, ReadRoleRequest, SetRoleRequest,
            SetRoleRequestBuilder,
        },
        responses::{ListRolesResponse, ReadRoleResponse},
    };
    use crate::client::VaultClient;
    use crate::error::ClientError;

    /// Deletes a role
    ///
    /// See [DeleteRoleRequest]
    pub fn delete(client: &VaultClient, mount: &str, name: &str) -> Result<(), ClientError> {
        let endpoint = DeleteRoleRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint)
    }

    /// Lists all roles
    ///
    /// See [ListRolesRequest]
    pub fn list(client: &VaultClient, mount: &str) -> Result<ListRolesResponse, ClientError> {
        let endpoint = ListRolesRequest::builder().mount(mount).build().unwrap();
        api::exec_with_result(client, endpoint)
    }

    /// Reads a role
    ///
    /// See [ReadRoleRequest]
    pub fn read(
        client: &VaultClient,
        mount: &str,
        name: &str,
    ) -> Result<ReadRoleResponse, ClientError> {
        let endpoint = ReadRoleRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint)
    }

    /// Creates or updates a role
    ///
    /// See [SetRoleRequest]
    pub fn set(
        client: &VaultClient,
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
        api::exec_with_empty(client, endpoint)
    }
}
