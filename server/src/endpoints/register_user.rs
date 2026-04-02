use crate::database_operations::register_new_user;
use crate::{RegisterUserInfo, error::ServerError};
use actix_web::{HttpResponse, Result, post, web};

/// Recebe json no formato
/// ```json
///{
///     "username": String,
///     "verifying_key": Vec<u8>,
///}
/// ```
#[post("/register-user")]
async fn register_user_endpoint(
    data: web::Json<RegisterUserInfo>,
) -> Result<HttpResponse, ServerError> {
    match register_new_user(data.into_inner()) {
        Ok(()) => Ok(HttpResponse::Ok().finish()),

        Err(rusqlite::Error::SqliteFailure(e, _))
            if e.code == rusqlite::ErrorCode::ConstraintViolation =>
        {
            Err(ServerError::UsernameTaken)
        }

        Err(e) => Err(ServerError::SqliteError(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, test};
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;
    use serde_json::json;

    fn create_user() -> (Vec<u8>, String) {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        (
            verifying_key.to_bytes().to_vec(),
            "dante".to_string(), //username
        )
    }

    #[actix_web::test]
    async fn test_index_post() {
        let (verifying_key, username) = create_user();

        let app =
            test::init_service(App::new().service(register_user_endpoint))
                .await;
        let req = test::TestRequest::post()
            .uri("/register-user")
            .set_json(json!({
                "username": username,
                "verifying_key": verifying_key,
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        let status = resp.response().status();
        let body = test::read_body(resp).await;
        println!("Response: status: {}, body: {:?}", status, body);
    }
}
