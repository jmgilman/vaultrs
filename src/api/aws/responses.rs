use serde::{Deserialize, Serialize};

/// Response from executing
/// [GetConfigurationRequest][crate::api::aws::requests::GetConfigurationRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct GetConfigurationResponse {
    pub access_key: String,
    pub region: String,
    pub iam_endpoint: String,
    pub sts_endpoint: String,
    pub max_retries: u32,
}

/// Response from executing
/// [RotateRootCredentialsRequest][crate::api::aws::requests::RotateRootCredentialsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct RotateRootCredentialsResponse {
    pub access_key: String,
}

/// Response from executing
/// [ReadLeaseRequest][crate::api::aws::requests::ReadLeaseRequest]

#[derive(Deserialize, Debug, Serialize)]
pub struct ReadLeaseResponse {
    pub lease: String,
    pub lease_max: String,
}

/// Response from executing
/// [ReadRoleRequest][crate::api::aws::requests::ReadRoleRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadRoleResponse {
    pub policy_document: Option<String>,
    pub policy_arns: Option<Vec<String>>,
    pub credential_type: String,
    pub role_arns: Option<Vec<String>>,
    pub iam_groups: Option<Vec<String>>,
}

/// Response from executing
/// [ListRolesRequest][crate::api::aws::requests::ListRolesRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListRolesResponse {
    pub keys: Vec<String>,
}

/// Response from executing
/// [GenerateCredentialsRequest][crate::api::aws::requests::GenerateCredentialsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct GenerateCredentialsResponse {
    pub access_key: String,
    pub secret_key: String,
    pub security_token: Option<String>,
    pub arn: String,
}
