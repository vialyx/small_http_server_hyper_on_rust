use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserRequest {
    pub username: String,
    pub age: u8,
}

/// Validate incoming JSON request
pub fn validate_request(json: &str) -> Result<UserRequest, String> {
    serde_json::from_str::<UserRequest>(json).map_err(|e| e.to_string())
}