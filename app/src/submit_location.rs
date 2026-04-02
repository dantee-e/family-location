use aes_gcm::AeadCore;
use aes_gcm::Aes256Gcm;
use aes_gcm::KeyInit;
use aes_gcm::aead::Aead;
use ed25519_dalek::ed25519::signature::SignerMut;
use rand::rngs::OsRng;

use crate::helpers::LatLong;
use crate::helpers::get_keys;

pub fn submit_location(username: &str, location: LatLong) {
    let mut keys = get_keys(username);

    let client = reqwest::blocking::Client::new();

    let cypher = Aes256Gcm::new(&keys.aes_gcm_key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let location_string = serde_json::to_string(&location).unwrap();
    let location_bytes = location_string.as_bytes();

    let encrypted_loc = cypher.encrypt(&nonce, location_bytes).unwrap();

    let signature = keys.signing_key.sign(&encrypted_loc);

    let response_json = serde_json::json!({
        "verifying_key": keys.verifying_key.as_bytes(),
        "payload": encrypted_loc,
        "signature": signature.to_bytes().to_vec(),
        "nonce": *nonce
    });

    let response = match client
        .post("http://localhost:8080/submit-location")
        .json(&response_json)
        .send()
    {
        Ok(r) => r,
        Err(e) => panic!("{}", e),
    };
    let status = response.status();
    match response.json::<serde_json::Value>() {
        Ok(body) => println!("Response status: {}\nBody: {}", status, body),
        Err(_) => println!("Response status: {}", status),
    };
}
