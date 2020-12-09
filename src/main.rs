#[macro_use]
extern crate diesel;
extern crate bcrypt;
extern crate dotenv;
extern crate jsonwebtoken;
extern crate r2d2;
extern crate r2d2_diesel;

use actix_cors::Cors;
use actix_web::{http, middleware as actix_middleware, web, App, HttpServer};
use env_logger::Env;

pub mod middleware;
pub mod models;
pub mod routes;
pub mod schema;
pub mod services;
pub mod state;
pub mod validation;
pub mod virtual_schema;

pub const DEFAULT_PER_PAGE: u32 = 15;

/// Return cors configuration for the project
fn cors() -> Cors {
    Cors::default()
        .send_wildcard()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
}

fn scoped_todos(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/todos")
            .route(web::post().to(routes::todos::store::handle))
            .route(web::get().to(routes::todos::index::handle))
            .wrap(middleware::auth::LoggedGuard),
    );
    cfg.service(
        web::resource("/todos/{todo_id}/check")
            .route(web::post().to(routes::todos::check::handle))
            .wrap(middleware::auth::LoggedGuard),
    );
    cfg.service(
        web::resource("/todos/{todo_id}/uncheck")
            .route(web::post().to(routes::todos::uncheck::handle))
            .wrap(middleware::auth::LoggedGuard),
    );
    cfg.service(
        web::resource("/self")
            .route(web::post().to(routes::users::index::handle))
            .wrap(middleware::auth::LoggedGuard),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(|| {
        App::new()
            // Init application state
            .data(crate::state::app::initialize())
            // Init setup of application request validators
            .app_data(crate::validation::new_todo_request::app_data())
            .app_data(crate::validation::new_user_request::app_data())
            // Logging setup
            .wrap(actix_middleware::Logger::default())
            .wrap(actix_middleware::Logger::new(
                "%a %t '%r' %s %b '%{Referer}i' '%{User-Agent}i' %T",
            ))
            .wrap(cors())
            .route("/", web::get().to(routes::sanity_check))
            // Authentication routes
            .route("/register", web::post().to(routes::auth::register::handle))
            .route("/login", web::post().to(routes::auth::login::handle))
            .service(web::scope("/").configure(scoped_todos))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
