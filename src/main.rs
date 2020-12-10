#[macro_use]
extern crate diesel;
extern crate bcrypt;
extern crate dotenv;
extern crate jsonwebtoken;
extern crate r2d2;
extern crate r2d2_diesel;

pub mod application;
mod crons;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod schema;
pub mod services;
pub mod state;
pub mod validation;
pub mod virtual_schema;

pub const DEFAULT_PER_PAGE: u32 = 15;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::thread::spawn(move || {
        crons::run_crons();
    });

    application::setup_web_server().await
}
