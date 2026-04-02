use aes_gcm::{Aes256Gcm, Key};
use ed25519_dalek::{SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};
use x25519_dalek::{PublicKey, StaticSecret};

#[derive(Serialize, Deserialize)]
pub struct KeysRaw {
    pub private_key: [u8; 32],
    pub public_key: [u8; 32],
    pub signing_key: [u8; 32],
    pub verifying_key: [u8; 32],
    pub aes_gcm_key: [u8; 32],
}

pub struct Keys {
    pub private_key: StaticSecret,
    pub public_key: PublicKey,
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
    pub aes_gcm_key: Key<Aes256Gcm>,
}

impl From<KeysRaw> for Keys {
    fn from(value: KeysRaw) -> Self {
        Keys {
            private_key: StaticSecret::from(value.private_key),
            public_key: PublicKey::from(value.public_key),
            signing_key: SigningKey::from(value.signing_key),
            verifying_key: VerifyingKey::from_bytes(&value.verifying_key).unwrap(),
            aes_gcm_key: value.aes_gcm_key.into(),
        }
    }
}

impl From<Keys> for KeysRaw {
    fn from(value: Keys) -> Self {
        Self {
            private_key: value.private_key.to_bytes(),
            public_key: value.public_key.to_bytes(),
            signing_key: value.signing_key.to_bytes(),
            verifying_key: value.verifying_key.to_bytes(),
            aes_gcm_key: value.aes_gcm_key.into(),
        }
    }
}
