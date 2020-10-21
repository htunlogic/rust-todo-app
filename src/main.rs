#[warn(unused_imports)]
#[warn(dead_code)]
#[macro_use]
extern crate diesel;
extern crate bcrypt;
extern crate dotenv;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod db;
mod models;
mod routes;
mod schema;
mod virtual_schema;

async fn sanity_check(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(sanity_check))
            .route("/register", web::post().to(routes::auth::register))
            .route("/login", web::post().to(routes::auth::login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
