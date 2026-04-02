mod database_operations;
mod endpoints;
mod error;
mod structs;

use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use database_operations::{check_db_exists, create_db, get_all_users};
use endpoints::{
    get_locations, register_user_endpoint, submit_location_endpoint,
};
use structs::RegisterUserInfo;

#[get("/")]
async fn hello() -> impl Responder {
    let users = get_all_users().unwrap();

    HttpResponse::Ok().json(users)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    if !check_db_exists() {
        create_db();
    } else {
        println!("A database ja existe, nao precisa criar dnovo");
    }

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(register_user_endpoint)
            .service(submit_location_endpoint)
            .service(get_locations)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
