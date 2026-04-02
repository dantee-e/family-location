use crate::database_operations::{
    check_user_exists_from_verifying_key, submit_new_location,
};
use crate::error::ServerError;
use crate::structs::Location;
use actix_web::{HttpResponse, Result, post, web};
use ed25519_dalek::{Signature, VerifyingKey};

pub fn verify_payload_signature(
    public_key: &VerifyingKey,
    signature: &Signature,
    payload: &[u8],
) -> Result<(), ServerError> {
    Ok(public_key.verify_strict(payload, signature)?)
}

/// Recebe json no formato
/// ```json
///{
///     "verifying_key": Vec<u8>,
///     "payload": Vec<u8>,
///     "signature": Vec<u8>,
///     "nonce": Vec<u8>,
///}
/// ```
#[post("/submit-location")]
async fn submit_location_endpoint(
    location: web::Json<Location>,
) -> Result<HttpResponse, ServerError> {
    let verifying_key = VerifyingKey::from_bytes(
        &location
            .verifying_key
            .as_slice()
            .try_into()
            .map_err(|_| ServerError::InvalidKey)?,
    )?;

    // check user exists
    match check_user_exists_from_verifying_key(verifying_key)? {
        Some(username) => username,
        None => {
            return Err(ServerError::UserNotFound);
        }
    };

    let signature = Signature::from_bytes(
        location
            .signature
            .as_slice()
            .try_into()
            .map_err(|_| ServerError::InvalidSignature)?,
    );

    verify_payload_signature(&verifying_key, &signature, &location.payload)?;
    submit_new_location(&location.into_inner())?;

    Ok(HttpResponse::Ok().finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, test};
    use ed25519_dalek::{SigningKey, ed25519::signature::SignerMut};
    use rand::rngs::OsRng;
    use serde_json::json;

    fn create_and_sign_location() -> (Vec<u8>, String, Vec<u8>) {
        let mut csprng = OsRng;
        let mut signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        let message = String::from("Latitude: 72.02888, Longitude: 102.10738");

        let signature = signing_key.sign(message.as_bytes());

        (
            verifying_key.to_bytes().to_vec(),
            message,
            signature.to_bytes().to_vec(),
        )
    }

    #[actix_web::test]
    async fn test_submit_location() {
        let (verifying_key, message, signature) = create_and_sign_location();

        let app =
            test::init_service(App::new().service(submit_location_endpoint))
                .await;
        let req = test::TestRequest::post()
            .uri("/submit-location")
            .set_json(json!({
                "verifying_key": verifying_key,
                "payload": message.as_bytes(),
                "signature": signature,
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        let status = resp.response().status();
        let body = test::read_body(resp).await;
        println!("Response: status: {}, body: {:?}", status, body);
    }
}
