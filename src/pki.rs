pub mod cert {
    use crate::api;
    use crate::api::pki::requests::{
        GenerateCertificateRequest, GenerateCertificateRequestBuilder, ListCertificatesRequest,
        ReadCertificateRequest, RevokeCertificateRequest, TidyRequest,
    };
    use crate::api::pki::responses::{
        GenerateCertificateResponse, ReadCertificateResponse, RevokeCertificateResponse,
    };
    use crate::client::Client;
    use crate::error::ClientError;

    /// Generates a certificate using the given role and options
    ///
    /// See [GenerateCertificateRequest]
    #[instrument(skip(client, opts), err)]
    pub async fn generate(
        client: &impl Client,
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
        api::exec_with_result(client, endpoint).await
    }

    /// Lists all certificates
    ///
    /// See [ListCertificatesRequest]
    #[instrument(skip(client), err)]
    pub async fn list(client: &impl Client, mount: &str) -> Result<Vec<String>, ClientError> {
        let endpoint = ListCertificatesRequest::builder()
            .mount(mount)
            .build()
            .unwrap();
        Ok(api::exec_with_result(client, endpoint).await?.keys)
    }

    /// Read a certificate using its serial
    ///
    /// See [ReadCertificateRequest]
    #[instrument(skip(client), err)]
    pub async fn read(
        client: &impl Client,
        mount: &str,
        serial: &str,
    ) -> Result<ReadCertificateResponse, ClientError> {
        let endpoint = ReadCertificateRequest::builder()
            .mount(mount)
            .serial(serial)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Revokes a certificate using its serial
    ///
    /// See [RevokeCertificateRequest]
    #[instrument(skip(client), err)]
    pub async fn revoke(
        client: &impl Client,
        mount: &str,
        serial: &str,
    ) -> Result<RevokeCertificateResponse, ClientError> {
        let endpoint = RevokeCertificateRequest::builder()
            .mount(mount)
            .serial_number(serial)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Tidy's up the certificate backend
    ///
    /// See [TidyRequest]
    #[instrument(skip(client), err)]
    pub async fn tidy(client: &impl Client, mount: &str) -> Result<(), ClientError> {
        let endpoint = TidyRequest::builder().mount(mount).build().unwrap();
        api::exec_with_empty_result(client, endpoint).await
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
            client::Client,
            error::ClientError,
        };

        /// Deletes the root CA
        ///
        /// See [DeleteRootRequest]
        #[instrument(skip(client), err)]
        pub async fn delete(client: &impl Client, mount: &str) -> Result<(), ClientError> {
            let endpoint = DeleteRootRequest::builder().mount(mount).build().unwrap();
            api::exec_with_empty(client, endpoint).await
        }

        /// Generates a new root CA
        ///
        /// See [GenerateRootRequest]
        #[instrument(skip(client, opts), err)]
        pub async fn generate(
            client: &impl Client,
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
            api::exec_with_result(client, endpoint).await
        }

        /// Signs a certificate using the root CA
        ///
        /// See [SignCertificateRequest]
        #[instrument(skip(client, opts), err)]
        pub async fn sign(
            client: &impl Client,
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
            api::exec_with_result(client, endpoint).await
        }

        /// Signs an intermediate CA using the root CA
        ///
        /// See [SignIntermediateRequest]
        #[instrument(skip(client, opts), err)]
        pub async fn sign_intermediate(
            client: &impl Client,
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
            api::exec_with_result(client, endpoint).await
        }

        /// Signs a self issued certificate using the root CA
        ///
        /// See [SignSelfIssuedRequest]
        #[instrument(skip(client, certificate), err)]
        pub async fn sign_self_issued(
            client: &impl Client,
            mount: &str,
            certificate: &str,
        ) -> Result<SignSelfIssuedResponse, ClientError> {
            let endpoint = SignSelfIssuedRequest::builder()
                .mount(mount)
                .certificate(certificate)
                .build()
                .unwrap();
            api::exec_with_result(client, endpoint).await
        }

        /// Configures the root CA
        ///
        /// See [SubmitCARequest]
        #[instrument(skip(client), err)]
        pub async fn submit(
            client: &impl Client,
            mount: &str,
            pem_bundle: &str,
        ) -> Result<(), ClientError> {
            let endpoint = SubmitCARequest::builder()
                .mount(mount)
                .pem_bundle(pem_bundle)
                .build()
                .unwrap();
            api::exec_with_empty(client, endpoint).await
        }

        pub mod int {
            use crate::api;
            use crate::{
                api::pki::{
                    requests::{
                        CrossSignRequest, CrossSignRequestBuilder, GenerateIntermediateRequest,
                        GenerateIntermediateRequestBuilder, SetSignedIntermediateRequest,
                    },
                    responses::{CrossSignResponse, GenerateIntermediateResponse},
                },
                client::Client,
                error::ClientError,
            };

            /// Generates an intermediate CA
            ///
            /// See [GenerateIntermediateRequest]
            #[instrument(skip(client, opts), err)]
            pub async fn generate(
                client: &impl Client,
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
                api::exec_with_result(client, endpoint).await
            }

            /// Sets the signed CA certificate
            ///
            /// See [SetSignedIntermediateRequest]
            #[instrument(skip(client, certificate), err)]
            pub async fn set_signed(
                client: &impl Client,
                mount: &str,
                certificate: &str,
            ) -> Result<(), ClientError> {
                let endpoint = SetSignedIntermediateRequest::builder()
                    .mount(mount)
                    .certificate(certificate)
                    .build()
                    .unwrap();
                api::exec_with_empty(client, endpoint).await
            }

            /// Generates intermediate CSR
            ///
            /// See [CrossSignRequest]
            #[instrument(skip(client, opts), err)]
            pub async fn cross_sign(
                client: &impl Client,
                mount: &str,
                opts: Option<&mut CrossSignRequestBuilder>,
            ) -> Result<CrossSignResponse, ClientError> {
                let mut t = CrossSignRequest::builder();
                let endpoint = opts.unwrap_or(&mut t).mount(mount).build().unwrap();
                api::exec_with_result(client, endpoint).await
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
        use crate::client::Client;
        use crate::error::ClientError;

        /// Rotates the CRL
        ///
        /// See [RotateCRLsRequest]
        #[instrument(skip(client), err)]
        pub async fn rotate(
            client: &impl Client,
            mount: &str,
        ) -> Result<RotateCRLsResponse, ClientError> {
            let endpoint = RotateCRLsRequest::builder().mount(mount).build().unwrap();
            api::exec_with_result(client, endpoint).await
        }

        /// Reads the CRL configuration
        ///
        /// See [ReadCRLConfigRequest]
        #[instrument(skip(client), err)]
        pub async fn read_config(
            client: &impl Client,
            mount: &str,
        ) -> Result<ReadCRLConfigResponse, ClientError> {
            let endpoint = ReadCRLConfigRequest::builder()
                .mount(mount)
                .build()
                .unwrap();
            api::exec_with_result(client, endpoint).await
        }

        /// Sets the CRL configuration
        ///
        /// See [SetCRLConfigRequest]
        #[instrument(skip(client, opts), err)]
        pub async fn set_config(
            client: &impl Client,
            mount: &str,
            opts: Option<&mut SetCRLConfigRequestBuilder>,
        ) -> Result<(), ClientError> {
            let mut t = SetCRLConfigRequest::builder();
            let endpoint = opts.unwrap_or(&mut t).mount(mount).build().unwrap();
            exec_with_empty(client, endpoint).await
        }
    }

    pub mod urls {
        use crate::api;
        use crate::api::pki::{
            requests::{ReadURLsRequest, SetURLsRequest, SetURLsRequestBuilder},
            responses::ReadURLsResponse,
        };
        use crate::client::Client;
        use crate::error::ClientError;

        /// Reads the configured certificate URLs
        ///
        /// See [ReadURLsRequest]
        #[instrument(skip(client), err)]
        pub async fn read(
            client: &impl Client,
            mount: &str,
        ) -> Result<ReadURLsResponse, ClientError> {
            let endpoint = ReadURLsRequest::builder().mount(mount).build().unwrap();
            api::exec_with_result(client, endpoint).await
        }

        /// Sets the configured certificate URLs
        ///
        /// See [SetURLsRequest]
        #[instrument(skip(client, opts), err)]
        pub async fn set(
            client: &impl Client,
            mount: &str,
            opts: Option<&mut SetURLsRequestBuilder>,
        ) -> Result<(), ClientError> {
            let mut t = SetURLsRequest::builder();
            let endpoint = opts.unwrap_or(&mut t).mount(mount).build().unwrap();
            api::exec_with_empty(client, endpoint).await
        }
    }
}

pub mod issuer {
    use crate::{
        api::{
            self,
            pki::{
                requests::{
                    DeleteIssuerRequest, ImportIssuerRequest, ReadIssuerCertificateRequest,
                    SetDefaultIssuerRequest, SignIntermediateIssuerRequest,
                    SignIntermediateIssuerRequestBuilder,
                },
                responses::{
                    ImportIssuerResponse, ReadIssuerCertificateResponse, SetDefaultIssuerResponse,
                    SignIntermediateIssuerResponse,
                },
            },
        },
        client::Client,
        error::ClientError,
    };

    /// Read issuer's certificate
    ///
    /// # Arguments
    ///
    /// * `client`: vault client
    /// * `mount`: vault pki mount path
    /// * `issuer`: issuer name or id (`default` if not specified)
    ///
    /// See [ReadIssuerCertificateRequest]
    #[instrument(skip(client, issuer), err)]
    pub async fn read(
        client: &impl Client,
        mount: &str,
        issuer: Option<&str>,
    ) -> Result<ReadIssuerCertificateResponse, ClientError> {
        let endpoint = ReadIssuerCertificateRequest::builder()
            .mount(mount)
            .issuer(issuer.unwrap_or("default"))
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Sign Intermediate CSR
    ///
    /// # Arguments
    ///
    /// * `client`: vault client
    /// * `mount`: vault pki mount path
    /// * `csr`: PEM-encoded CSR to be signed
    /// * `common_name`: CN for certificate
    /// * `issuer`: name or id of existing issuer ("default" if not specified)
    ///
    /// See [SignIntermediateIssuerRequest]
    #[instrument(skip(client, opts), err)]
    pub async fn sign_intermediate(
        client: &impl Client,
        mount: &str,
        csr: &str,
        common_name: &str,
        issuer: Option<&str>,
        opts: Option<&mut SignIntermediateIssuerRequestBuilder>,
    ) -> Result<SignIntermediateIssuerResponse, ClientError> {
        let mut t = SignIntermediateIssuerRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut t)
            .mount(mount)
            .csr(csr)
            .common_name(common_name)
            .issuer(issuer.unwrap_or("default"))
            .build()
            .unwrap();

        api::exec_with_result(client, endpoint).await
    }

    /// Import CA certificate and private key bundle
    ///
    /// # Arguments
    ///
    /// * `client`: vault client
    /// * `mount`: vault pki mount path
    /// * `pem_bundle`: certificate and private key concatenated in any order
    ///
    /// See [ImportIssuerRequest]
    #[instrument(skip(client), err)]
    pub async fn import(
        client: &impl Client,
        mount: &str,
        pem_bundle: &str,
    ) -> Result<ImportIssuerResponse, ClientError> {
        let endpoint = ImportIssuerRequest::builder()
            .mount(mount)
            .pem_bundle(pem_bundle)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Set default issuer
    ///
    /// # Arguments
    ///
    /// * `client`: vault client
    /// * `mount`: vault pki mount path
    /// * `default_issuer`: default issuer id or name
    ///
    /// See [SetDefaultIssuerRequest]
    #[instrument(skip(client), err)]
    pub async fn set_default(
        client: &impl Client,
        mount: &str,
        default_issuer: &str,
    ) -> Result<SetDefaultIssuerResponse, ClientError> {
        let endpoint = SetDefaultIssuerRequest::builder()
            .mount(mount)
            .default_issuer(default_issuer)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Delete issuer
    ///
    /// # Arguments
    ///
    /// * `client`: vault client
    /// * `mount`: vault pki mount path
    /// * `issuer`: issuer name or id
    ///
    /// See [DeleteIssuerRequest]
    #[instrument(skip(client), err)]
    pub async fn delete(
        client: &impl Client,
        mount: &str,
        issuer: &str,
    ) -> Result<(), ClientError> {
        let endpoint = DeleteIssuerRequest::builder()
            .mount(mount)
            .issuer(issuer)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }

    pub mod int {
        use crate::{
            api::{
                self,
                pki::{
                    requests::{
                        GenerateIntermediateCSRRequest, GenerateIntermediateCSRRequestBuilder,
                    },
                    responses::GenerateIntermediateCSRResponse,
                },
            },
            client::Client,
            error::ClientError,
        };

        /// Generates an intermediate CSR
        ///
        /// See [GenerateIntermediateCSRRequest]
        #[instrument(skip(client, opts), err)]
        pub async fn generate(
            client: &impl Client,
            mount: &str,
            request_type: &str,
            common_name: &str,
            opts: Option<&mut GenerateIntermediateCSRRequestBuilder>,
        ) -> Result<GenerateIntermediateCSRResponse, ClientError> {
            let mut t = GenerateIntermediateCSRRequest::builder();
            let endpoint = opts
                .unwrap_or(&mut t)
                .mount(mount)
                .request_type(request_type)
                .common_name(common_name)
                .build()
                .unwrap();
            api::exec_with_result(client, endpoint).await
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
    use crate::client::Client;
    use crate::error::ClientError;

    /// Deletes a role
    ///
    /// See [DeleteRoleRequest]
    #[instrument(skip(client), err)]
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
    #[instrument(skip(client), err)]
    pub async fn list(client: &impl Client, mount: &str) -> Result<ListRolesResponse, ClientError> {
        let endpoint = ListRolesRequest::builder().mount(mount).build().unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Reads a role
    ///
    /// See [ReadRoleRequest]
    #[instrument(skip(client), err)]
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
    #[instrument(skip(client, opts), err)]
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

pub mod key {
    use crate::{
        api::{self, pki::requests::DeleteKeyRequest},
        client::Client,
        error::ClientError,
    };

    /// Delete key
    ///
    /// # Arguments
    ///
    /// * `client`: vault client
    /// * `mount`: vault pki mount path
    /// * `key`: key name or id
    ///
    /// See [DeleteKeyRequest]
    #[instrument(skip(client, key), err)]
    pub async fn delete(client: &impl Client, mount: &str, key: &str) -> Result<(), ClientError> {
        let endpoint = DeleteKeyRequest::builder()
            .mount(mount)
            .key(key)
            .build()
            .unwrap();
        api::exec_with_empty(client, endpoint).await
    }
}
