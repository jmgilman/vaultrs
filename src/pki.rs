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

    pub fn list(client: &VaultClient, mount: &str) -> Result<Vec<String>, ClientError> {
        let endpoint = ListCertificatesRequest::builder()
            .mount(mount)
            .build()
            .unwrap();
        Ok(api::exec_with_result(client, endpoint)?.keys)
    }

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

        pub fn delete(client: &VaultClient, mount: &str) -> Result<(), ClientError> {
            let endpoint = DeleteRootRequest::builder().mount(mount).build().unwrap();
            api::exec_with_empty(client, endpoint)
        }

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
        use crate::api::pki::requests::{
            ReadCRLConfigRequest, ReadCRLConfigRequestBuilder, RotateCRLsRequest,
            RotateCRLsRequestBuilder, SetCRLConfigRequest, SetCRLConfigRequestBuilder,
        };

        pub fn rotate(mount: &str) -> RotateCRLsRequestBuilder {
            RotateCRLsRequest::builder().mount(mount).to_owned()
        }

        pub fn read_config(mount: &str) -> ReadCRLConfigRequestBuilder {
            ReadCRLConfigRequest::builder().mount(mount).to_owned()
        }

        pub fn set_config(mount: &str) -> SetCRLConfigRequestBuilder {
            SetCRLConfigRequest::builder().mount(mount).to_owned()
        }
    }

    pub mod urls {
        use crate::api::pki::requests::{
            ReadURLsRequest, ReadURLsRequestBuilder, SetURLsRequest, SetURLsRequestBuilder,
        };

        pub fn read_urls(mount: &str) -> ReadURLsRequestBuilder {
            ReadURLsRequest::builder().mount(mount).to_owned()
        }

        pub fn set_urls(mount: &str) -> SetURLsRequestBuilder {
            SetURLsRequest::builder().mount(mount).to_owned()
        }
    }
}

pub mod role {
    use crate::api::pki::requests::{
        DeleteRoleRequest, DeleteRoleRequestBuilder, ListRolesRequest, ListRolesRequestBuilder,
        ReadRoleRequest, ReadRoleRequestBuilder, SetRoleRequest, SetRoleRequestBuilder,
    };

    pub fn delete(mount: &str, name: &str) -> DeleteRoleRequestBuilder {
        DeleteRoleRequest::builder()
            .mount(mount)
            .name(name)
            .to_owned()
    }

    pub fn list(mount: &str) -> ListRolesRequestBuilder {
        ListRolesRequest::builder().mount(mount).to_owned()
    }

    pub fn read(mount: &str, name: &str) -> ReadRoleRequestBuilder {
        ReadRoleRequest::builder()
            .mount(mount)
            .name(name)
            .to_owned()
    }

    pub fn set(mount: &str, name: &str) -> SetRoleRequestBuilder {
        SetRoleRequest::builder().mount(mount).name(name).to_owned()
    }
}
