use super::responses::{
    CreateRoleTagResponse, ListCertificateConfigurationsResponse, ListDenyListTagsResponse,
    ListIdentityAccessListEntriesResponse, ListRolesResponse, ListStsRolesResponse,
    ReadCertificateConfigurationResponse, ReadClientConfigurationResponse,
    ReadIdentityAccessListInformationResponse, ReadIdentityAccessListTidySettingsResponse,
    ReadIdentityConfigurationResponse, ReadRoleResponse, ReadRoleTagDenyListResponse,
    ReadRoleTagDenyListTidySettingsResponse, ReadStsRoleResponse, RotateRootCredentialsResponse,
};
use rustify_derive::Endpoint;
use serde::Serialize;

/// ## Configure Client
/// Configures the credentials required to perform API calls to AWS as well as custom endpoints to talk to AWS APIs.
///
/// * Path: /auth/{self.mount}/config/client
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#configure-client>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/config/client",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ConfigureClientRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub max_retries: Option<i64>,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub endpoint: Option<String>,
    pub iam_endpoint: Option<String>,
    pub sts_endpoint: Option<String>,
    pub sts_region: Option<String>,
    pub iam_server_id_header_value: Option<String>,
    pub allowed_sts_header_values: Option<String>,
}

/// ## Read Client Configuration
/// Returns the previously configured AWS access credentials.
///
/// * Path: /auth/{self.mount}/config/client
/// * Method: GET
/// * Response: [ReadClientConfigurationResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#read-config>
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/config/client",
    response = "ReadClientConfigurationResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct ReadClientConfigurationRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Delete Client Configuration
/// Deletes the previously configured AWS access credentials.
///
/// * Path: /auth/{self.mount}/config/client
/// * Method: DELETE
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#delete-config>
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/config/client",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into))]
pub struct DeleteClientConfigurationRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Rotate Root Credentials
/// When you have configured Vault with static credentials, you can use this endpoint to have Vault rotate the access key it used.
///
/// * Path: /auth/{self.mount}/config/rotate-root
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#rotate-root-credentials>
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/config/rotate-root",
    method = "POST",
    response = "RotateRootCredentialsResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct RotateRootCredentialsRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Configure Identity Integration
/// This configures the way that Vault interacts with the Identity store.
///
/// * Path: /auth/{self.mount}/config/identity
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#configure-identity-integration>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/config/identity",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ConfigureIdentityRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub iam_alias: Option<String>,
    pub iam_metadata: Option<Vec<String>>,
    pub ec2_alias: Option<String>,
    pub ec2_metadata: Option<Vec<String>>,
}

/// ## Read Identity Integration Configuration
/// Returns the previously configured Identity integration configuration
///
/// * Path: /auth/{self.mount}/config/identity
/// * Method: GET
/// * Response: [ReadIdentityConfigurationResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#read-identity-integration-configuration>
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/config/identity",
    response = "ReadIdentityConfigurationResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct ReadIdentityConfigurationRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Create Certificate Configuration
/// Registers an AWS public key to be used to verify the instance identity documents.
///
/// * Path: /auth/{self.mount}/config/certificate/{self.cert_name}
/// * Method: GET
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#create-certificate-configuration>
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "/auth/{self.mount}/config/certificate/{self.cert_name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct CreateCertificateConfigurationRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub cert_name: String,
    pub aws_public_cert: String,
    #[serde(rename = "type")]
    pub cert_type: Option<String>,
}

/// ## Read Certificate Configuration
/// Returns the previously configured AWS public key.
///
/// * Path: /auth/{self.mount}/config/certificate/{self.cert_name}
/// * Method: GET
/// * Response: [ReadCertificateConfigurationResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#read-certificate-configuration>
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/config/certificate/{self.cert_name}",
    response = "ReadCertificateConfigurationResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct ReadCertificateConfigurationRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub cert_name: String,
}

/// ## Delete Certificate Configuration
/// Removes the previously configured AWS public key.
///
/// * Path: /auth/{self.mount}/config/certificate/{self.cert_name}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#delete-certificate-configuration>
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/config/certificate/{self.cert_name}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into))]
pub struct DeleteCertificateConfigurationRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub cert_name: String,
}

/// ## List Certificate Configurations
/// Lists all the AWS public certificates that are registered with the method.
///
/// * Path: /auth/{self.mount}/config/certificates
/// * Method: LIST
/// * Response: [ListCertificateConfigurationsResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#list-certificate-configuration>
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/config/certificates",
    method = "LIST",
    response = "ListCertificateConfigurationsResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct ListCertificateConfigurationsRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Create STS Role
/// Allows the explicit association of STS roles to satellite AWS accounts (i.e. those which are
/// not the account in which the Vault server is running.)
///
/// * Path: /auth/{self.mount}/config/sts/{self.account_id}
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#create-sts-role>
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/config/sts/{self.account_id}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into))]
pub struct CreateStsRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub account_id: String,
    pub sts_role: String,
}

/// ## Read STS Role
/// Returns the previously configured STS role.
///
/// * Path: /auth/{self.mount}/config/sts/{self.account_id}
/// * Method: GET
/// * Response: [ReadStsRoleResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#read-sts-role>
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/config/sts/{self.account_id}",
    response = "ReadStsRoleResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct ReadStsRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub account_id: String,
}

/// ## List STS Roles
/// Lists all the AWS Account IDs for which an STS role is registered.
///
/// * Path: /auth/{self.mount}/config/sts
/// * Method: LIST
/// * Response: [ListStsRolesResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#list-sts-roles>
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/config/sts",
    method = "LIST",
    response = "ListStsRolesResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct ListStsRolesRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Delete STS Role
/// Deletes a previously configured AWS account/STS role association.
///
/// * Path: /auth/{self.mount}/config/sts/{self.account_id}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#delete-sts-role>
#[derive(Builder, Debug, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/config/sts/{self.account_id}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into))]
pub struct DeleteStsRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub account_id: String,
}

/// ## Configure Identity Access List Tidy Operation
/// Configures the periodic tidying operation of the access listed identity entries.
///
/// * Path: /auth/{self.mount}/config/tidy/identity-accesslist
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#configure-identity-access-list-tidy-operation>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/config/tidy/identity-accesslist",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ConfigureIdentityAccessListTidyOperationRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub safety_buffer: Option<String>,
    pub disable_periodic_tidy: Option<bool>,
}

/// ## Read Identity Access List Tidy Settings
/// Returns the previously configured periodic access list tidying settings.
///
/// * Path: /auth/{self.mount}/config/tidy/identity-accesslist
/// * Method: GET
/// * Response: ReadIdentityAccessListTidySettingsResponse
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#read-identity-access-list-tidy-settings>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/config/tidy/identity-accesslist",
    response = "ReadIdentityAccessListTidySettingsResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadIdentityAccessListTidySettingsRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Delete Identity Access List Tidy Settings
/// Deletes the previously configured periodic access list tidying settings.
///
/// * Path: /auth/{self.mount}/config/tidy/identity-accesslist
/// * Method: DELETE
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#delete-identity-access-list-tidy-settings>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/config/tidy/identity-accesslist",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteIdentityAccessListTidySettingsRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Configure Role Tag Deny List Tidy Operation
/// Configures the periodic tidying operation of the deny listed role tag entries.
///
/// * Path: /auth/{self.mount}/config/tidy/roletag-denylist
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#configure-role-tag-deny-list-tidy-operation>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/config/tidy/roletag-denylist",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ConfigureRoleTagDenyListTidyOperationRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub safety_buffer: Option<String>,
    pub disable_periodic_tidy: Option<bool>,
}

/// ## Read Role Tag Deny List Tidy Settings
/// Returns the previously configured periodic deny list tidying settings.
///
/// * Path: /auth/{self.mount}/config/tidy/roletag-denylist
/// * Method: GET
/// * Response: ReadRoleTagDebyListTidySettingsResponse
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#read-role-tag-deny-list-tidy-settings>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/config/tidy/roletag-denylist",
    response = "ReadRoleTagDenyListTidySettingsResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadRoleTagDenyListTidySettingsRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Delete Role Tag Deny List Tidy Settings
/// Deletes the previously configured periodic deny list tidying settings.
///
/// * Path: /auth/{self.mount}/config/tidy/roletag-denylist
/// * Method: DELETE
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#delete-role-tag-deny-list-tidy-settings>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/config/tidy/roletag-denylist",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteRoleTagDenyListTidySettingsRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Create Role
/// Registers a role in the method
///
/// * Path: /auth/{self.mount}/role/{self.role}
/// * Method: POST
/// * Response: [N/A]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#create-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/role/{self.role}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct CreateRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub role: String,
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

/// ## Read Role
/// Returns the previously registered role configuration
///
/// * Path: /auth/{self.mount}/role/{self.role}
/// * Method: GET
/// * Response: [ReadRoleResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#read-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/role/{self.role}",
    response = "ReadRoleResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub role: String,
}

/// ## List Roles
/// Lists all the roles that are registered with the method
///
/// * Path: /auth/{self.mount}/roles
/// * Method: LIST
/// * Response: [ListRolesResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#list-roles>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/roles",
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
/// Deletes the previously registered role
///
/// * Path: /auth/{self.mount}/role/{self.role}
/// * Method: DELETE
/// * Response: [N/A]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#delete-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/role/{self.role}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub role: String,
}

/// ## Create Role Tags
/// Creates a role tag on the role, which help in restricting the capabilities
/// that are set on the role
///
/// * Path: /auth/{self.mount}/role/{self.role}/tag
/// * Method: POST
/// * Response: [CreateRoleTagResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#create-role-tags>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/role/{self.role}/tag",
    method = "POST",
    response = "CreateRoleTagResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct CreateRoleTagRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub role: String,
    pub policies: Option<Vec<String>>,
    pub max_ttl: Option<String>,
    pub instance_id: Option<String>,
    pub allow_instance_migration: Option<bool>,
    pub disallow_reauthentication: Option<bool>,
}

/// ## Login(IAM method)
/// This endpoint verifies the pkcs7 signature of the signed GetCallerIdentity request.
///
/// * Path: /auth/{self.mount}/login
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#login>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "/auth/{self.mount}/login", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct IamLoginRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub role: Option<String>,
    pub iam_http_request_method: String,
    pub iam_request_url: String,
    pub iam_request_body: String,
    pub iam_request_headers: String,
}

/// ## Login(EC2 method)
/// This endpoint verifies the pkcs7 signature of the instance identity document.
///
/// * Path: /auth/{self.mount}/login
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#login>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "/auth/{self.mount}/login", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct Ec2LoginRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub role: Option<String>,
    pub nonce: Option<String>,
    pub identity: String,
    pub signature: String,
    pub pkcs7: String,
}

/// ## Place Role Tags in Deny List
/// Places a valid role tag in a deny list
///
/// * Path: /auth/{self.mount}/roletag-denylist/{self.tag_value}
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#place-role-tags-in-deny-list>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/roletag-denylist/{self.tag_value}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct PlaceRoleTagsInDenyListRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub tag_value: String,
}

/// ## Read Role Tag Deny List Information
/// Returns the deny list entry of a previously deny listed role tag.
///
/// * Path: /auth/{self.mount}/roletag-denylist/{self.role_tag}
/// * Method: GET
/// * Response: [ReadRoleTagDenyListResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#read-role-tag-deny-list-information>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/roletag-denylist/{self.tag_value}",
    response = "ReadRoleTagDenyListResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadRoleTagDenyListRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub tag_value: String,
}

/// ## List Deny List Tags
/// Lists all the role tags that are deny listed
///
/// * Path: /auth/{self.mount}/roletag-denylist
/// * Method: LIST
/// * Response: [ListDenyListTagsResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#list-deny-list-tags>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/roletag-denylist",
    method = "LIST",
    response = "ListDenyListTagsResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListDenyListTagsRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Delete Deny List Tags
/// Deletes a deny listed role tag
///
/// * Path: /auth/{self.mount}/roletag-denylist/{self.role_tag}
/// * Method: DELETE
/// * Response: [N/A]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#delete-deny-list-tags>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/roletag-denylist/{self.tag_value}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteDenyListTagsRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub tag_value: String,
}

/// ## Tidy Deny List Tags
///
/// Cleans up the entries in the deny listed based on expiration time on the entry and safety_buffer.
/// * Path: /auth/{self.mount}/tidy/roletag-denylist
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#tidy-deny-list-tags>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/tidy/roletag-denylist",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct TidyDenyListTagsRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub safety_buffer: Option<String>,
}

/// ## Read Identity Access List Information
/// Returns an entry in the identity access list.
///
/// * Path: /auth/{self.mount}/identity-accesslist/{self.instance_id}
/// * Method: GET
/// * Response: [ReadIdentityAccessListInformationResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#tidy-deny-list-tags>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/identity-accesslist/{self.instance_id}",
    response = "ReadIdentityAccessListInformationResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadIdentityAccessListInformationRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub instance_id: String,
}

/// ## List Identity Access List Entries
/// Lists all the instance IDs that are in the access list of successful logins
///
/// * Path: /auth/{self.mount}/identity-accesslist
/// * Method: LIST
/// * Response: [ListIdentityAccessListEntriesResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#list-identity-access-list-entries>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/identity-accesslist",
    method = "LIST",
    response = "ListIdentityAccessListEntriesResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListIdentityAccessListEntriesRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Delete Identity Access List Entries
/// Deletes a cache of the successful login from an instance
///
/// * Path: /auth/{self.mount}/identity-accesslist/{self.instance_id}
/// * Method: DELETE
/// * Response: [N/A]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#delete-identity-access-list-entries>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/identity-accesslist/{self.instance_id}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteIdentityAccessListEntriesRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub instance_id: String,
}

/// ## Tidy Identity Access List Entries
/// Cleans up the entries in the access list based on expiration time andsafety_buffer
///
/// * Path: /auth/{self.mount}/tidy/identity-accesslist
/// * Method: POST
/// * Response: [N/A]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/aws#tidy-identity-access-list-entries>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/tidy/identity-accesslist",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct TidyIdentityAccessListEntriesRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub safety_buffer: Option<String>,
}
