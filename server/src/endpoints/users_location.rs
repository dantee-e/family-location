use crate::database_operations::get_locations_from_username;
use crate::error::ServerError;
use actix_web::{HttpResponse, Result, get, web};

#[get("/get-locations-username/{username}")]
async fn get_locations(
    path: web::Path<String>,
) -> Result<HttpResponse, ServerError> {
    let username = path.into_inner();

    // let verifying_key = match verifying_key_from_username(username)? {
    //     Some(v) => v,
    //     None => {
    //         return Err(ServerError::UserNotFound);
    //     }
    // };

    let locations = get_locations_from_username(username)?;

    Ok(HttpResponse::Ok().json(locations))
}
