use super::responses::{
    ListRolesResponse, ReadKubernetesAuthConfigResponse, ReadKubernetesRoleResponse,
};
use rustify_derive::Endpoint;

/// ## Configure Kubernetes Auth
/// Sets backend configuration for the Kubernetes auth mount
///
/// * Path: /auth/kubernetes/config
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/kubernetes#configure-method>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(path = "/auth/{self.mount}/config", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct ConfigureKubernetesAuthRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub kubernetes_host: String,
    pub kubernetes_ca_cert: Option<String>,
    pub pem_keys: Option<Vec<String>>,
    pub issuer: Option<String>,
    pub disable_iss_validation: bool,
    pub disable_local_ca_jwt: bool,
}

/// ## Read Kubernetes Auth Config
/// Gets backend configuration for the Kubernetes auth mount
///
/// * Path: /auth/kubernetes/config
/// * Method: GET
/// * Response: [ReadKubernetesAuthConfigResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docsauth/kubernetes#read-config>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/config",
    method = "GET",
    response = "ReadKubernetesAuthConfigResponse",
    builder = "true"
)]
#[builder(setter(into))]
pub struct ReadKubernetesAuthConfigRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Login with Kubernetes
/// Issues a Vault token based on the presented credentials.
///
/// * Path: /auth/kubernetes/login
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docsauth/kubernetes#login>
#[derive(Builder, Debug, Endpoint)]
#[endpoint(path = "/auth/{self.mount}/login", method = "POST", builder = "true")]
#[builder(setter(into))]
pub struct LoginWithKubernetesRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub role: String,
    pub jwt: String,
}

/// ## List Roles
/// Returns a list the existing Kubernetes roles.
///
/// * Path: /auth/{self.mount}/role
/// * Method: LIST
/// * Response: [ListRolesResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/kubernetes#list-roles>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/role",
    method = "LIST",
    response = "ListRolesResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListRolesRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Create Kubernetes role
/// Creates a new Kubernetes Role.
///
/// * Path: /auth/{self.mount}/role/{self.name}
/// * Method: POST
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docsauth/kubernetes#create-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/role/{self.name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct CreateKubernetesRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
    pub bound_service_account_names: Vec<String>,
    pub bound_service_account_namespaces: Vec<String>,
    pub audience: Option<String>,
    pub token_ttl: Option<String>,
    pub token_max_ttl: Option<String>,
    pub token_policies: Option<Vec<String>>,
    pub token_bound_cidrs: Option<Vec<String>>,
    pub token_explicit_max_ttl: Option<String>,
    pub token_no_default_policy: Option<bool>,
    pub token_num_uses: Option<u64>,
    pub token_period: Option<String>,
    pub token_type: Option<String>,
}

/// ## Read AppRole
/// Reads the properties of an existing Kubernetes role.
///
/// * Path: /auth/{self.mount}/role/{self.name}
/// * Method: GET
/// * Response: [ReadKubernetesRoleResponse]
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/kubernetes#read-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/role/{self.name}",
    response = "ReadKubernetesRoleResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadKubernetesRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}

/// ## Delete AppRole
/// Deletes an existing Kubernetes.
///
/// * Path: /auth/{self.mount}/role/{self.name}
/// * Method: DELETE
/// * Response: N/A
/// * Reference: <https://developer.hashicorp.com/vault/api-docs/auth/kubernetes#delete-role>
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/auth/{self.mount}/role/{self.name}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteKubernetesRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}
