pub use crate::keys_struct::{Keys, KeysRaw};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Location {
    pub verifying_key: Vec<u8>,
    pub payload: Vec<u8>,
    pub signature: Vec<u8>,
    pub nonce: Vec<u8>,
}

#[derive(Deserialize, Serialize)]
pub struct LatLong {
    pub latitude: f64,
    pub longitude: f64,
}

pub fn get_keys(username: &str) -> Keys {
    let keys_str = std::fs::read_to_string(format!("src/keys/{username}/keys.json")).unwrap();
    let keys_raw: KeysRaw = serde_json::from_str(&keys_str).unwrap();
    keys_raw.into()
}
