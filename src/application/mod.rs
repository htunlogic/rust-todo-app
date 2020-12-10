use actix_cors::Cors;
use actix_web::{http, middleware as actix_middleware, web, App, HttpServer};
use env_logger::Env;

pub async fn setup_web_server() -> std::io::Result<()> {
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
      .wrap(setup_cors())
      .service(web::scope("/").configure(setup_routes))
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}

/// Return cors configuration for the project
fn setup_cors() -> Cors {
  Cors::default()
    .send_wildcard()
    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
    .allowed_header(http::header::CONTENT_TYPE)
    .max_age(3600)
}

/// Setup all the routes
fn setup_routes(cfg: &mut web::ServiceConfig) {
  // GET /
  cfg.service(web::resource("/").route(web::get().to(crate::routes::sanity_check)));
  // POST /register
  cfg.service(
    web::resource("/register").route(web::post().to(crate::routes::auth::register::handle)),
  );
  // POST /login
  cfg.service(web::resource("/login").route(web::post().to(crate::routes::auth::login::handle)));
  // GET /todos
  // POST /todos
  cfg.service(
    web::resource("/todos")
      .route(web::post().to(crate::routes::todos::store::handle))
      .route(web::get().to(crate::routes::todos::index::handle))
      .wrap(crate::middleware::auth::LoggedGuard),
  );
  // POST /todos/{todo_id}/check
  cfg.service(
    web::resource("/todos/{todo_id}/check")
      .route(web::post().to(crate::routes::todos::check::handle))
      .wrap(crate::middleware::auth::LoggedGuard),
  );
  // POST /todos/{todo_id}/uncheck
  cfg.service(
    web::resource("/todos/{todo_id}/uncheck")
      .route(web::post().to(crate::routes::todos::uncheck::handle))
      .wrap(crate::middleware::auth::LoggedGuard),
  );
  // POST /self
  cfg.service(
    web::resource("/self")
      .route(web::post().to(crate::routes::users::index::handle))
      .wrap(crate::middleware::auth::LoggedGuard),
  );
}
