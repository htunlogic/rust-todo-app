use crate::models::user::{AuthenticableUser, NewUser};
use actix_web::{web, HttpResponse, Responder};

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
pub async fn register(user: web::Json<NewUser>) -> impl Responder {
  match NewUser::create(&user.email, &user.password) {
    Ok(created) => HttpResponse::Ok().json(created),
    Err(_) => HttpResponse::Ok().body("Error"),
  }
}

/// Authenticate the user with email and password
///
/// @param {String} email
/// @param {String} password
///
/// Success code 200:
/// ```
/// {
///   "id": "be24fb8b-09ca-472c-abef-4ae04c530cfd",
///   "email": "test@barrage.net"
/// }
/// ```
///
/// Error: 400 or 401
pub async fn login(user: web::Json<AuthenticableUser>) -> impl Responder {
  match AuthenticableUser::authenticate(&user.email, &user.password) {
    Ok(authenticated) => HttpResponse::Ok().json(authenticated),
    Err(_) => HttpResponse::Unauthorized().finish(),
  }
}
