use crate::models::auth::AuthenticableUser;
use actix_web::{web, HttpResponse, Responder};

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
pub async fn handle(user: web::Json<AuthenticableUser>) -> impl Responder {
  match AuthenticableUser::authenticate(&user.email, &user.password) {
    Ok((authenticated, token)) => HttpResponse::Ok().header("jwt", token).json(authenticated),
    Err(_) => HttpResponse::Unauthorized().finish(),
  }
}
