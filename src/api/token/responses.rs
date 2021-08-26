use serde::{Deserialize, Serialize};

/// Response from executing
/// [ListAccessorRequest][crate::api::token::requests::ListAccessorRequest]
#[derive(Deserialize, Debug, Serialize)]
pub struct ListAccessorResponse {
    pub keys: Vec<String>,
}
