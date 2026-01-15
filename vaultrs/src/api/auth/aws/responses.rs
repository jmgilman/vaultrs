use serde::{Deserialize, Serialize};

/// Response from executing
/// [ReadClientConfigurationRequest][crate::api::auth::aws::requests::ReadClientConfigurationRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadClientConfigurationResponse {
    pub access_key: Option<String>,
    pub endpoint: Option<String>,
    pub iam_endpoint: Option<String>,
    pub sts_endpoint: Option<String>,
    pub sts_region: Option<String>,
    pub iam_server_id_header_value: Option<String>,
    pub allowed_sts_header_values: Option<String>,
    pub max_retries: Option<i64>,
}

/// Response from executing
/// [RotateRootCredentialsRequest][crate::api::auth::aws::requests::RotateRootCredentialsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct RotateRootCredentialsResponse {
    pub access_key: String,
}

/// Response from executing
/// [ReadIdentityConfigurationRequest][crate::api::auth::aws::requests::ReadIdentityConfigurationRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadIdentityConfigurationResponse {
    pub iam_alias: Option<String>,
    pub iam_metadata: Option<Vec<String>>,
    pub ec2_alias: Option<String>,
    pub ec2_metadata: Option<Vec<String>>,
}

/// Response from executing
/// [ReadCertificateConfigurationRequest][crate::api::auth::aws::requests::ReadCertificateConfigurationRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadCertificateConfigurationResponse {
    pub aws_public_cert: String,
    #[serde(rename = "type")]
    pub cert_type: String,
}

/// Response from executing
/// [ListCertificateConfigurationsRequest][crate::api::auth::aws::requests::ListCertificateConfigurationsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListCertificateConfigurationsResponse {
    pub keys: Vec<String>,
}

/// Response from executing
/// [ReadStsRoleRequest][crate::api::auth::aws::requests::ReadStsRoleRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadStsRoleResponse {
    pub sts_role: String,
}

/// Response from executing
/// [ListStsRolesRequest][crate::api::auth::aws::requests::ListStsRolesRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListStsRolesResponse {
    pub keys: Vec<String>,
}

/// Response from executing
/// [ReadIdentityAccessListTidySettingsRequest][crate::api::auth::aws::requests::ReadIdentityAccessListTidySettingsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadIdentityAccessListTidySettingsResponse {
    pub safety_buffer: u64,
    pub disable_periodic_tidy: bool,
}

/// Response from executing
/// [ReadRoleTagDenyListTidySettingsRequest][crate::api::auth::aws::requests::ReadRoleTagDenyListTidySettingsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadRoleTagDenyListTidySettingsResponse {
    pub safety_buffer: u64,
    pub disable_periodic_tidy: bool,
}

/// Response from executing
/// [ReadRoleRequest][crate::api::auth::aws::requests::ReadRoleRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadRoleResponse {
    pub auth_type: Option<String>,
    pub bound_ami_id: Option<Vec<String>>,
    pub bound_account_id: Option<Vec<String>>,
    pub bound_region: Option<Vec<String>>,
    pub bound_vpc_id: Option<Vec<String>>,
    pub bound_subnet_id: Option<Vec<String>>,
    pub bound_iam_role_arn: Option<Vec<String>>,
    pub bound_iam_instance_profile_arn: Option<Vec<String>>,
    pub bound_ec2_instance_id: Option<Vec<String>>,
    pub role_tag: Option<String>,
    pub bound_iam_principal_arn: Option<Vec<String>>,
    pub inferred_entity_type: Option<String>,
    pub inferred_aws_region: Option<String>,
    pub resolve_aws_unique_ids: Option<bool>,
    pub allow_instance_migration: Option<bool>,
    pub disallow_reauthentication: Option<bool>,
    pub token_ttl: Option<i64>,
    pub token_max_ttl: Option<i64>,
    pub token_policies: Option<Vec<String>>,
    pub token_bound_cidrs: Option<Vec<String>>,
    pub token_explicit_max_ttl: Option<i64>,
    pub token_no_default_policy: Option<bool>,
    pub token_num_uses: Option<i64>,
    pub token_period: Option<i64>,
    pub token_type: Option<String>,
}

/// Response from executing
/// [ListRolesRequest][crate::api::auth::aws::requests::ListRolesRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListRolesResponse {
    pub keys: Vec<String>,
}

/// Response from executing
/// [CreateRoleTagRequest][crate::api::auth::aws::requests::CreateRoleTagRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct CreateRoleTagResponse {
    pub tag_value: String,
    pub tag_key: String,
}

/// Response from executing
/// [ReadRoleTagDenyListRequest][crate::api::auth::aws::requests::ReadRoleTagDenyListRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadRoleTagDenyListResponse {
    pub expiration_time: String,
    pub creation_time: String,
}

/// Response from executing
/// [ListDenyListTagsRequest][crate::api::auth::aws::requests::ListDenyListTagsRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListDenyListTagsResponse {
    pub keys: Vec<String>,
}

/// Response from executing
/// [ReadIdentityAccessListInformationRequest][crate::api::auth::aws::requests::ReadIdentityAccessListInformationRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadIdentityAccessListInformationResponse {
    pub pending_time: String,
    pub expiration_time: String,
    pub creation_time: String,
    pub client_nonce: String,
    pub role: String,
}

/// Response from executing
/// [ListIdentityAccessListEntriesRequest][crate::api::auth::aws::requests::ListIdentityAccessListEntriesRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListIdentityAccessListEntriesResponse {
    pub keys: Vec<String>,
}
