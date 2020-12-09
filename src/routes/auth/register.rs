use crate::models::user::NewUser;
use crate::state::app::AppState;
use crate::validation::new_user_request::NewUserRequest;
use actix_web::{web, HttpResponse, Responder};
use actix_web_validator::Json;

/// Register new user with email and password
///
/// @param {String} email
/// @param {String} password
///
/// Success code 200
/// ```
/// {
///   "id": "be24fb8b-09ca-472c-abef-4ae04c530cfd",
///   "email": "test@barrage.net"
/// }
/// ```
///
/// Error: 400
pub async fn handle(user: Json<NewUserRequest>, state: web::Data<AppState>) -> impl Responder {
  match NewUser::create(&state.get_connection(), &user.email, &user.password) {
    Ok(created) => HttpResponse::Ok().json(created),
    Err(_) => HttpResponse::Ok().body("Error"),
  }
}
