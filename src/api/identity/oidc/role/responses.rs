use serde::{Deserialize, Serialize};

/// Response from executing
/// [ReadRoleRequest](crate::api::identity::oidc::role::ReadRoleRequest)
#[derive(Deserialize, Debug, Serialize)]
pub struct ReadRoleResponse {
    client_id: String,
    key: String,
    template: String,
    ttl: String,
}
