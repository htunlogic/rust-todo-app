use crate::models::user::User;
use crate::models::user::UserWithTodo;
use crate::state::app::AppState;
use actix_web::{web, HttpResponse, Responder};

/// Uncheck the todo
///
/// @param {String} todo_id
///
/// Success code 200:
/// ```
/// {
///   "id": "06b8ff8c-3e34-4226-b917-cb07bd98785e",
///   "user_id": "c4d5f3b2-5149-489e-b103-f9e3b8adc3ff",
///   "content": "Do something",
///   "checked": false
/// }
/// ```
///
/// Error: 400
pub async fn handle(req: web::HttpRequest, state: web::Data<AppState>) -> impl Responder {
  let auth = match req.extensions_mut().remove::<User>() {
    Some(user) => user,
    None => return HttpResponse::BadRequest().finish(),
  };

  match UserWithTodo::show(&state.get_connection(), &auth.id) {
    Ok(user) => HttpResponse::Ok().json(user),
    Err(_) => HttpResponse::BadRequest().finish(),
  }
}
