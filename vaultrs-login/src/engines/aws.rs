use std::collections::HashMap;

use async_trait::async_trait;
use base64::{engine::general_purpose, Engine as _};
use std::time::SystemTime;
use vaultrs::{api::AuthInfo, client::Client, error::ClientError};

use crate::LoginMethod;
use aws_sigv4::http_request::{sign, SignableRequest, SigningParams, SigningSettings};

/// A login method which uses AWS credentials for obtaining a new token.
#[derive(Debug)]
pub struct AwsIamLogin {
    pub access_key: String,
    pub secret_key: String,
    pub session_token: Option<String>,
    pub region: String,
    pub role: Option<String>,
    pub header_value: Option<String>,
}

/// A login method which uses EC2 instance pkcs7 signature for obtaining a new token.
#[derive(Debug)]
pub struct AwsEc2Login {
    pub pkcs7: String,
    pub nonce: Option<String>,
    pub role: Option<String>,
}

#[async_trait]
impl LoginMethod for AwsIamLogin {
    async fn login(&self, client: &impl Client, mount: &str) -> Result<AuthInfo, ClientError> {
        let sts_endpoint = "https://sts.amazonaws.com";

        let mut req_builder = http::Request::builder()
            .uri(sts_endpoint)
            .method("POST")
            .header(
                "Content-Type",
                "application/x-www-form-urlencoded;charset=utf-8",
            );

        if let Some(header_value) = &self.header_value {
            req_builder = req_builder.header("X-Vault-AWS-IAM-Server-ID", header_value);
        }

        let mut request = req_builder
            .body("Action=GetCallerIdentity&Version=2011-06-15")
            .unwrap();

        let mut signing_params = SigningParams::builder()
            .access_key(&self.access_key)
            .secret_key(&self.secret_key)
            .region(&self.region)
            .service_name("sts")
            .settings(SigningSettings::default())
            .time(SystemTime::now());

        signing_params.set_security_token(self.session_token.as_deref());

        let signable_request = SignableRequest::from(&request);
        let (out, _sig) = sign(signable_request, &signing_params.build().unwrap())
            .unwrap()
            .into_parts();

        out.apply_to_request(&mut request);

        let iam_http_request_method = request.method().as_str();
        let iam_request_url = general_purpose::STANDARD.encode(request.uri().to_string());
        let iam_request_headers = general_purpose::STANDARD.encode(
            serde_json::to_string(
                &request
                    .headers()
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.to_str().unwrap()))
                    .collect::<HashMap<&str, &str>>(),
            )
            .unwrap(),
        );
        let iam_request_body = general_purpose::STANDARD.encode(request.body());

        vaultrs::auth::aws::iam_login(
            client,
            mount,
            iam_http_request_method,
            &iam_request_url,
            &iam_request_headers,
            &iam_request_body,
            self.role.as_deref(),
        )
        .await
    }
}

#[async_trait]
impl LoginMethod for AwsEc2Login {
    async fn login(&self, client: &impl Client, mount: &str) -> Result<AuthInfo, ClientError> {
        vaultrs::auth::aws::ec2_login(
            client,
            mount,
            self.pkcs7.as_str(),
            self.nonce.as_deref(),
            self.role.as_deref(),
        )
        .await
    }
}
