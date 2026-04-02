use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegisterUserInfo {
    pub username: String,
    pub verifying_key: Vec<u8>,
    pub public_key: Vec<u8>,
}
