use rustify_derive::Endpoint;
use std::fmt::Debug;

use super::responses::{
    GenerateCredentialsResponse, GetConfigurationResponse, ListRolesResponse, ReadLeaseResponse,
    ReadRoleResponse, RotateRootCredentialsResponse,
};

/// ## Configure Root IAM Credentials
///
/// Configures the root IAM credentials to communicate with AWS.
///
/// * Path: {self.mount}/config/root
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/aws#configure-root-iam-credentials>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "{self.mount}/config/root", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct SetConfigurationRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub max_retries: Option<i32>,
    pub access_key: String,
    pub secret_key: String,
    pub region: Option<String>,
    pub iam_endpoint: Option<String>,
    pub sts_endpoint: Option<String>,
    pub username_template: Option<String>,
}

/// ## Read Root Configuration
///
/// Read non-secure values that have been configured in the config/root endpoint
///
/// * Path: {self.mount}/config/root
/// * Method: GET
/// * Response: [GetConfigurationResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/aws#read-root-configuration>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/config/root",
    method = "GET",
    builder = "true",
    response = "GetConfigurationResponse"
)]
#[builder(setter(into, strip_option), default)]
pub struct GetConfigurationRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Rotate Root IAM Credentials
///
/// When you have configured Vault with static credentials, you can use this endpoint to have Vault rotate the access key it used.
///
/// * Path: {self.mount}/config/rotate-root
/// * Method: GET
/// * Response: [RotateRootCredentialsResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/aws#rotate-root-iam-credentials>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/config/rotate-root",
    method = "POST",
    builder = "true",
    response = "RotateRootCredentialsResponse"
)]
#[builder(setter(into, strip_option), default)]
pub struct RotateRootCredentialsRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Configure Lease
///
/// Configures lease settings for the AWS secrets engine
///
/// * Path: {self.mount}/config/lease
/// * Method: POST
/// * Response: N.A.
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/aws#configure-lease>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "{self.mount}/config/lease", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct ConfigureLeaseRequest {
    #[endpoint(skip)]
    pub mount: String,

    pub lease: String,
    pub lease_max: String,
}

/// ## Read Lease
///
/// Returns the current lease settings for the AWS secrets engine
///
/// * Path: {self.mount}/config/lease
/// * Method: GET
/// * Response: [ReadLeaseResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/aws#read-lease>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/config/lease",
    method = "GET",
    response = "ReadLeaseResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadLeaseRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Create/Update Role
///
/// Creates or updates the role with the given name
///
/// * Path: {self.mount}/roles/{self.name}
/// * Method: POST
/// * Response: N.A.
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/aws#create-update-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/roles/{self.name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct CreateUpdateRoleRequest {
    #[endpoint(skip)]
    pub mount: String,

    pub name: String,
    pub credential_type: String,
    pub role_arns: Option<Vec<String>>,
    pub policy_arns: Option<Vec<String>>,
    pub policy_document: String,
    pub iam_groups: Option<Vec<String>>,
    pub iam_tags: Option<Vec<String>>,
    pub default_sts_ttl: Option<u32>,
    pub max_sts_ttl: Option<u32>,
    pub user_path: Option<String>,
    pub permissions_boundary_arn: Option<String>,

    pub policy: Option<String>,
    pub arn: Option<String>,
}

/// ## Read Role
///
/// Queries an existing role by the given name
///
/// * Path: {self.mount}/roles/{self.name}
/// * Method: GET
/// * Response: [ReadRoleResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/aws#read-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/roles/{self.name}",
    method = "GET",
    response = "ReadRoleResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadRoleRequest {
    #[endpoint(skip)]
    pub mount: String,

    pub name: String,
}

/// ## List Roles
///
///  lists all existing roles in the secrets engine
///
/// * Path: {self.mount}/roles
/// * Method: LIST
/// * Response: [ListRolesResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/aws#list-roles>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/roles",
    method = "LIST",
    response = "ListRolesResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListRolesRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Delete Role
///
/// Deletes an existing role by the given name
///
/// * Path: {self.mount}/roles/{self.name}
/// * Method: DELETE
/// * Response: N.A.
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/aws#delete-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/roles/{self.name}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub name: String,
}

/// ## Generate Credentials (/aws/creds)
///
/// Generates credentials based on the named role using /aws/creds endpoint
///
/// * Path: {self.mount}/creds/{self.name}
/// * Method: GET
/// * Response: [GenerateCredentialsResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/aws#generate-credentials>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/creds/{self.name}",
    method = "GET",
    response = "GenerateCredentialsResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct GenerateCredentialsRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub name: String,
    pub role_arn: Option<String>,
    pub role_session_name: Option<String>,
    pub ttl: Option<String>,
}

/// ## Generate Credentials (/aws/sts)
///
/// Generates credentials based on the named role using /aws/sts endpoint
///
/// * Path: {self.mount}/sts/{self.name}
/// * Method: POST
/// * Response: [GenerateCredentialsResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/secret/aws#generate-credentials>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "{self.mount}/sts/{self.name}",
    method = "POST",
    response = "GenerateCredentialsResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct GenerateCredentialsStsRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub name: String,
    pub role_arn: Option<String>,
    pub role_session_name: Option<String>,
    pub ttl: Option<String>,
}
