use actix_web::HttpResponse;
use log::error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Not Found")]
    #[allow(dead_code)]
    NotFound,

    #[error("actix error")]
    InternalError(#[from] actix_web::Error),

    #[allow(dead_code)]
    #[error("unknown error")]
    Unknown,

    #[error("Invalid signature")]
    InvalidSignature,

    #[error("Ed25519 error")]
    Ed25519Error(#[from] ed25519_dalek::ed25519::Error),

    #[error("Invalid public key")]
    InvalidKey,

    #[error("Username taken")]
    UsernameTaken,

    #[error("User not found")]
    UserNotFound,

    #[error("sqlite3 error")]
    SqliteError(#[from] rusqlite::Error),

    #[error("serde error")]
    SerdeError(#[from] serde_json::Error),
    // #[error("Bad Request")]
    // BadRequest,
}

impl actix_web::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServerError::NotFound => {
                error!("NotFound");
                HttpResponse::NotFound().json("Not Found")
            }

            ServerError::UserNotFound => {
                error!("UserNotFound");
                HttpResponse::NotFound().json(
                    "User not registered. Try registering first at /register-user",
                )
            }

            ServerError::UsernameTaken => {
                error!("UsernameTaken");
                HttpResponse::Conflict().json("Username already taken")
            }

            ServerError::InvalidKey => {
                error!("InvalidKey");
                HttpResponse::BadRequest().json("Invalid verifying key")
            }

            ServerError::InvalidSignature => {
                error!("InvalidSignature");
                HttpResponse::Unauthorized()
                    .json("Signature could not be verified")
            }

            ServerError::Ed25519Error(e) => {
                error!("Ed25519Error: {:?}", e);
                HttpResponse::Unauthorized()
                    .json("Signature could not be verified")
            }

            ServerError::SqliteError(e) => {
                error!("SqliteError: {:?}", e);
                HttpResponse::InternalServerError()
                    .json("Internal server error")
            }

            ServerError::SerdeError(e) => {
                error!("SerdeError: {:?}", e);
                HttpResponse::InternalServerError()
                    .json("Internal server error")
            }

            ServerError::InternalError(e) => {
                error!("InternalError: {:?}", e);
                e.error_response()
            }

            ServerError::Unknown => {
                error!("Unknown error");
                HttpResponse::InternalServerError()
                    .json("Internal server error")
            }
        }
    }
}
