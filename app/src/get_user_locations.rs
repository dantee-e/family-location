use aes_gcm::{Aes256Gcm, KeyInit, aead::Aead};
use ed25519_dalek::Signature;

use crate::helpers::{LatLong, Location, get_keys};

pub fn get_locations(username: String) {
    let client = reqwest::blocking::Client::new();
    let response = match client
        .get(format!(
            "http://localhost:8080/get-locations-username/{username}"
        ))
        .send()
    {
        Ok(r) => r,
        Err(e) => panic!("{}", e),
    };
    let status = response.status();

    if status != 200 {
        eprintln!("Error: Status code {status}");
        return;
    }

    let keys = get_keys(&username);

    let locations_raw: Vec<Location> = response.json().unwrap();

    #[allow(unused_variables)]
    let locations_decoded: Vec<LatLong> = Vec::new();

    for locations_raw in locations_raw {
        println!("Checkpoint1");
        let signature =
            Signature::from_bytes(locations_raw.signature.as_slice().try_into().unwrap());
        println!("Checkpoint2");
        keys.verifying_key
            .verify_strict(&locations_raw.payload, &signature)
            .expect("Wrong signature");
        println!("Checkpoint3");

        let nonce = aes_gcm::Nonce::from_slice(&locations_raw.nonce);
        println!("Checkpoint4");
        let cypher = Aes256Gcm::new(&keys.aes_gcm_key);
        println!("Checkpoint5");
        let decrypted_message = cypher.decrypt(nonce, &locations_raw.payload[..]).unwrap();
        println!("Checkpoint6");
        let decrypted_string = String::from_utf8(decrypted_message).unwrap();
        println!("Checkpoint7");
        println!("decrypted_string = {decrypted_string}");
    }
}
