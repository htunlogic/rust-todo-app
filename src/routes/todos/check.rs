use crate::models::todo::Todo;
use crate::models::user::User;
use actix_web::{web, HttpResponse, Responder};

/// Check the todo
///
/// @param {String} todo_id
///
/// Success code 200:
/// ```
/// {
///   "id": "06b8ff8c-3e34-4226-b917-cb07bd98785e",
///   "user_id": "c4d5f3b2-5149-489e-b103-f9e3b8adc3ff",
///   "content": "Do something",
///   "checked": true
/// }
/// ```
///
/// Error: 400
pub async fn handle(req: web::HttpRequest, path: web::Path<String>) -> impl Responder {
  let auth = match req.extensions_mut().remove::<User>() {
    Some(user) => user,
    None => return HttpResponse::BadRequest().finish(),
  };

  let mut todo = match Todo::show(&path.0) {
    Ok(todo) => todo,
    Err(_) => return HttpResponse::NotFound().finish(),
  };

  // Allow change only to todos that the user actually owns
  if &todo.user_id != &auth.id {
    return HttpResponse::Forbidden().finish();
  }

  match todo.check() {
    Ok(_) => {
      todo.checked = true;
      HttpResponse::Ok().json(todo)
    }
    Err(_) => HttpResponse::BadRequest().finish(),
  }
}
