pub mod cert {
    use crate::api::pki::requests::{
        GenerateCertificateRequest, GenerateCertificateRequestBuilder, ListCertificatesRequest,
        ListCertificatesRequestBuilder, ReadCertificateRequest, ReadCertificateRequestBuilder,
        RevokeCertificateRequest, RevokeCertificateRequestBuilder, TidyRequest, TidyRequestBuilder,
    };

    pub fn generate(mount: &str, role: &str) -> GenerateCertificateRequestBuilder {
        GenerateCertificateRequest::builder()
            .mount(mount)
            .role(role)
            .to_owned()
    }

    pub fn list(mount: &str) -> ListCertificatesRequestBuilder {
        ListCertificatesRequest::builder().mount(mount).to_owned()
    }

    pub fn read(mount: &str, serial: &str) -> ReadCertificateRequestBuilder {
        ReadCertificateRequest::builder()
            .mount(mount)
            .serial(serial)
            .to_owned()
    }

    pub fn revoke(mount: &str, serial: &str) -> RevokeCertificateRequestBuilder {
        RevokeCertificateRequest::builder()
            .mount(mount)
            .serial(serial)
            .to_owned()
    }

    pub fn tidy(mount: &str) -> TidyRequestBuilder {
        TidyRequest::builder().mount(mount).to_owned()
    }

    pub mod ca {
        use crate::api::pki::requests::{
            DeleteRootRequest, DeleteRootRequestBuilder, GenerateRootRequest,
            GenerateRootRequestBuilder, SignCertificateRequest, SignCertificateRequestBuilder,
            SignIntermediateRequest, SignIntermediateRequestBuilder, SignSelfIssuedRequest,
            SignSelfIssuedRequestBuilder, SubmitCARequest, SubmitCARequestBuilder,
        };

        pub fn delete(mount: &str) -> DeleteRootRequestBuilder {
            DeleteRootRequest::builder().mount(mount).to_owned()
        }

        pub fn generate(mount: &str, cert_type: &str) -> GenerateRootRequestBuilder {
            GenerateRootRequest::builder()
                .mount(mount)
                .cert_type(cert_type)
                .to_owned()
        }

        pub fn sign(mount: &str, csr: &str, common_name: &str) -> SignCertificateRequestBuilder {
            SignCertificateRequest::builder()
                .mount(mount)
                .csr(csr)
                .common_name(common_name)
                .to_owned()
        }

        pub fn sign_intermediate(
            mount: &str,
            csr: &str,
            common_name: &str,
        ) -> SignIntermediateRequestBuilder {
            SignIntermediateRequest::builder()
                .mount(mount)
                .csr(csr)
                .common_name(common_name)
                .to_owned()
        }

        pub fn sign_self_issued(mount: &str, certificate: &str) -> SignSelfIssuedRequestBuilder {
            SignSelfIssuedRequest::builder()
                .mount(mount)
                .certificate(certificate)
                .to_owned()
        }

        pub fn submit(mount: &str, pem_bundle: &str) -> SubmitCARequestBuilder {
            SubmitCARequest::builder()
                .mount(mount)
                .pem_bundle(pem_bundle)
                .to_owned()
        }

        pub mod int {
            use crate::api::pki::requests::{
                GenerateIntermediateRequest, GenerateIntermediateRequestBuilder,
                SetSignedIntermediateRequest, SetSignedIntermediateRequestBuilder,
            };

            pub fn generate(mount: &str, cert_type: &str) -> GenerateIntermediateRequestBuilder {
                GenerateIntermediateRequest::builder()
                    .mount(mount)
                    .cert_type(cert_type)
                    .to_owned()
            }

            pub fn set_signed(
                mount: &str,
                certificate: &str,
            ) -> SetSignedIntermediateRequestBuilder {
                SetSignedIntermediateRequest::builder()
                    .mount(mount)
                    .certificate(certificate)
                    .to_owned()
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
