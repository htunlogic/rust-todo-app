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

pub const DEFAULT_PER_PAGE: u32 = 15;

async fn sanity_check() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(sanity_check))
            // Authentication routes
            .route("/register", web::post().to(routes::auth::register::handle))
            .route("/login", web::post().to(routes::auth::login::handle))
            // Todo routes
            .route(
                "/todos/{user_id}",
                web::get().to(routes::todos::index::handle),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
