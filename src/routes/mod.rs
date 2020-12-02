pub mod auth;
pub mod todos;
pub mod users;

use actix_web::{HttpResponse, Responder};

pub async fn sanity_check() -> impl Responder {
  HttpResponse::Ok().body("Hello world")
}
