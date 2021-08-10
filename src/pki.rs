pub mod cert {
    use rustify::endpoint::Endpoint;

    use crate::api::pki::{
        requests::{
            GenerateCertificateData, GenerateCertificateRequest, ListCertificatesRequest,
            ReadCertificateRequest, RevokeCertificateDataBuilder, RevokeCertificateRequest,
            TidyData, TidyRequest,
        },
        responses::GenerateCertificateResponse,
    };
    pub use crate::{client::VaultClient, error::ClientError};

    pub fn generate(
        client: &VaultClient,
        mount: &str,
        role: &str,
        data: GenerateCertificateData,
    ) -> Result<GenerateCertificateResponse, ClientError> {
        let req = GenerateCertificateRequest {
            mount: mount.to_string(),
            role: role.to_string(),
            data,
        };
        Ok(req
            .execute(&client.http)?
            .ok_or(ClientError::ResponseEmptyError)?
            .data)
    }

    pub fn list(client: &VaultClient, mount: &str) -> Result<Vec<String>, ClientError> {
        let req = ListCertificatesRequest {
            mount: mount.to_string(),
        };
        Ok(req
            .execute(&client.http)?
            .ok_or(ClientError::ResponseEmptyError)?
            .data
            .keys)
    }

    pub fn read(client: &VaultClient, mount: &str, serial: &str) -> Result<String, ClientError> {
        let req = ReadCertificateRequest {
            mount: mount.to_string(),
            serial: serial.to_string(),
        };
        Ok(req
            .execute(&client.http)?
            .ok_or(ClientError::ResponseEmptyError)?
            .data
            .certificate)
    }

    pub fn revoke(client: &VaultClient, mount: &str, serial: &str) -> Result<u64, ClientError> {
        let req = RevokeCertificateRequest {
            mount: mount.to_string(),
            data: RevokeCertificateDataBuilder::default()
                .serial(serial)
                .build()
                .unwrap(),
        };
        Ok(req
            .execute(&client.http)?
            .ok_or(ClientError::ResponseEmptyError)?
            .data
            .revocation_time)
    }

    pub fn tidy(client: &VaultClient, mount: &str, data: TidyData) -> Result<(), ClientError> {
        let req = TidyRequest {
            mount: mount.to_string(),
            data,
        };
        req.execute(&client.http)?;
        Ok(())
    }

    pub mod ca {
        use super::Endpoint;
        use crate::api::pki::{
            requests::{
                DeleteRootRequest, GenerateRootData, GenerateRootRequest, SignCertificateData,
                SignCertificateRequest, SignIntermediateData, SignIntermediateRequest,
                SignSelfIssuedData, SignSelfIssuedRequest, SubmitCADataBuilder, SubmitCARequest,
            },
            responses::{
                GenerateRootResponse, SignCertificateResponse, SignIntermediateResponse,
                SignSelfIssuedResponse,
            },
        };

        pub fn delete(client: &super::VaultClient, mount: &str) -> Result<(), super::ClientError> {
            let req = DeleteRootRequest {
                mount: mount.to_string(),
            };
            req.execute(&client.http)?;
            Ok(())
        }

        pub fn generate(
            client: &super::VaultClient,
            mount: &str,
            cert_type: &str,
            data: GenerateRootData,
        ) -> Result<GenerateRootResponse, super::ClientError> {
            let req = GenerateRootRequest {
                mount: mount.to_string(),
                cert_type: cert_type.to_string(),
                data,
            };
            Ok(req
                .execute(&client.http)?
                .ok_or(super::ClientError::ResponseEmptyError)?
                .data)
        }

        pub fn sign(
            client: &super::VaultClient,
            mount: &str,
            csr: &str,
            common_name: &str,
            data: SignCertificateData,
        ) -> Result<SignCertificateResponse, super::ClientError> {
            let data = SignCertificateData {
                csr: Some(csr.to_string()),
                common_name: Some(common_name.to_string()),
                ..data
            };
            let req = SignCertificateRequest {
                mount: mount.to_string(),
                data,
            };
            Ok(req
                .execute(&client.http)?
                .ok_or(super::ClientError::ResponseEmptyError)?
                .data)
        }

        pub fn sign_intermediate(
            client: &super::VaultClient,
            mount: &str,
            csr: &str,
            common_name: &str,
            data: SignIntermediateData,
        ) -> Result<SignIntermediateResponse, super::ClientError> {
            let data = SignIntermediateData {
                csr: Some(csr.to_string()),
                common_name: Some(common_name.to_string()),
                ..data
            };
            let req = SignIntermediateRequest {
                mount: mount.to_string(),
                data,
            };
            Ok(req
                .execute(&client.http)?
                .ok_or(super::ClientError::ResponseEmptyError)?
                .data)
        }

        pub fn sign_self_issued(
            client: &super::VaultClient,
            mount: &str,
            certificate: &str,
        ) -> Result<SignSelfIssuedResponse, super::ClientError> {
            let req = SignSelfIssuedRequest {
                mount: mount.to_string(),
                data: SignSelfIssuedData {
                    certificate: certificate.to_string(),
                },
            };
            Ok(req
                .execute(&client.http)?
                .ok_or(super::ClientError::ResponseEmptyError)?
                .data)
        }

        pub fn submit(
            client: &super::VaultClient,
            mount: &str,
            pem_bundle: &String,
        ) -> Result<(), super::ClientError> {
            let req = SubmitCARequest {
                mount: mount.to_string(),
                data: SubmitCADataBuilder::default()
                    .pem_bundle(pem_bundle)
                    .build()
                    .unwrap(),
            };
            req.execute(&client.http)?;
            Ok(())
        }

        pub mod int {
            use super::Endpoint;
            use crate::{
                api::pki::{
                    requests::{
                        GenerateIntermediateData, GenerateIntermediateRequest,
                        SetSignedIntermediateRequest, SubmitSignedIntermediateDataBuilder,
                    },
                    responses::GenerateIntermediateResponse,
                },
                error::ClientError,
            };

            pub fn generate(
                client: &super::super::VaultClient,
                mount: &str,
                cert_type: &str,
                data: GenerateIntermediateData,
            ) -> Result<GenerateIntermediateResponse, super::super::ClientError> {
                let req = GenerateIntermediateRequest {
                    mount: mount.to_string(),
                    cert_type: cert_type.to_string(),
                    data,
                };
                Ok(req
                    .execute(&client.http)?
                    .ok_or(ClientError::ResponseEmptyError)?
                    .data)
            }

            pub fn set_signed(
                client: &super::super::VaultClient,
                mount: &str,
                certificate: &String,
            ) -> Result<(), super::super::ClientError> {
                let req = SetSignedIntermediateRequest {
                    mount: mount.to_string(),
                    data: SubmitSignedIntermediateDataBuilder::default()
                        .certificate(certificate)
                        .build()
                        .unwrap(),
                };
                req.execute(&client.http)?;
                Ok(())
            }
        }
    }

    pub mod crl {
        use super::Endpoint;
        use crate::api::pki::{
            requests::{
                ReadCRLConfigRequest, RotateCRLsRequest, SetCRLConfigData, SetCRLConfigRequest,
            },
            responses::ReadCRLConfigResponse,
        };

        pub fn rotate(
            client: &super::VaultClient,
            mount: &str,
        ) -> Result<bool, super::ClientError> {
            let req = RotateCRLsRequest {
                mount: mount.to_string(),
            };
            Ok(req
                .execute(&client.http)?
                .ok_or(super::ClientError::ResponseEmptyError)?
                .data
                .success)
        }

        pub fn read_config(
            client: &super::VaultClient,
            mount: &str,
        ) -> Result<ReadCRLConfigResponse, super::ClientError> {
            let req = ReadCRLConfigRequest {
                mount: mount.to_string(),
            };
            Ok(req
                .execute(&client.http)?
                .ok_or(super::ClientError::ResponseEmptyError)?
                .data)
        }

        pub fn set_config(
            client: &super::VaultClient,
            mount: &str,
            data: SetCRLConfigData,
        ) -> Result<(), super::ClientError> {
            let req = SetCRLConfigRequest {
                mount: mount.to_string(),
                data,
            };
            req.execute(&client.http)?;
            Ok(())
        }
    }

    pub mod urls {
        use crate::api::pki::{
            requests::{ReadURLsRequest, SetURLsData, SetURLsRequest},
            responses::ReadURLsResponse,
        };

        use super::Endpoint;

        pub fn read_urls(
            client: &super::VaultClient,
            mount: &str,
        ) -> Result<ReadURLsResponse, super::ClientError> {
            let req = ReadURLsRequest {
                mount: mount.to_string(),
            };
            Ok(req
                .execute(&client.http)?
                .ok_or(super::ClientError::ResponseEmptyError)?
                .data)
        }

        pub fn set_urls(
            client: &super::VaultClient,
            mount: &str,
            data: SetURLsData,
        ) -> Result<(), super::ClientError> {
            let req = SetURLsRequest {
                mount: mount.to_string(),
                data,
            };
            req.execute(&client.http)?;
            Ok(())
        }
    }
}

pub mod role {
    use crate::api::pki::{
        requests::{
            DeleteRoleRequest, ListRolesRequest, ReadRoleRequest, SetRoleData, SetRoleRequest,
        },
        responses::ReadRoleResponse,
    };
    use crate::{client::VaultClient, error::ClientError};
    use rustify::endpoint::Endpoint;

    pub fn delete(client: &VaultClient, mount: &str, name: &str) -> Result<(), ClientError> {
        let req = DeleteRoleRequest {
            mount: mount.to_string(),
            name: name.to_string(),
        };
        req.execute(&client.http)?;
        Ok(())
    }

    pub fn list(client: &VaultClient, mount: &str) -> Result<Vec<String>, ClientError> {
        let req = ListRolesRequest {
            mount: mount.to_string(),
        };
        Ok(req
            .execute(&client.http)?
            .ok_or(ClientError::ResponseEmptyError)?
            .data
            .keys)
    }

    pub fn read(
        client: &VaultClient,
        mount: &str,
        name: &str,
    ) -> Result<ReadRoleResponse, ClientError> {
        let req = ReadRoleRequest {
            mount: mount.to_string(),
            name: name.to_string(),
        };
        Ok(req
            .execute(&client.http)?
            .ok_or(ClientError::ResponseEmptyError)?
            .data)
    }

    pub fn set(
        client: &VaultClient,
        mount: &str,
        name: &str,
        data: SetRoleData,
    ) -> Result<(), ClientError> {
        let req = SetRoleRequest {
            mount: mount.to_string(),
            name: name.to_string(),
            data,
        };
        req.execute(&client.http)?;
        Ok(())
    }
}
