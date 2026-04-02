use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Location {
    pub verifying_key: Vec<u8>,
    pub payload: Vec<u8>,
    pub signature: Vec<u8>,
    pub nonce: Vec<u8>,
}
