use crate::keys_struct::{Keys, KeysRaw};
use aes_gcm::{Aes256Gcm, KeyInit};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use x25519_dalek::{PublicKey, StaticSecret};

pub fn register_user(username: String) {
    let mut csprng = OsRng;

    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();

    let aes_gcm_key = Aes256Gcm::generate_key(csprng);

    let private_key = StaticSecret::random_from_rng(csprng);
    let public_key = PublicKey::from(&private_key);

    let client = reqwest::blocking::Client::new();

    let response = match client
        .post("http://localhost:8080/register-user")
        .json(&serde_json::json!({
            "username": username,
            "verifying_key": verifying_key.to_bytes(),
            "public_key": public_key.to_bytes(),
        }))
        .send()
    {
        Ok(r) => r,
        Err(e) => panic!("{}", e),
    };
    let status = response.status();

    match response.json::<serde_json::Value>() {
        Ok(body) => {
            println!("Response status: {}\nBody: {}", status, body);
        }
        Err(_) => println!("Response status: {}", status),
    };

    if status == 200 {
        // Only saves if creation was successful
        let keys = Keys {
            private_key,
            public_key,
            signing_key,
            verifying_key,
            aes_gcm_key,
        };
        let keys_raw: KeysRaw = keys.into();
        std::fs::create_dir_all(format!("src/keys/{username}")).unwrap();
        std::fs::write(
            format!("src/keys/{username}/keys.json"),
            serde_json::to_string(&keys_raw).unwrap(),
        )
        .unwrap();
    }
}
