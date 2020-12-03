use crate::models::todo::NewTodo;
use crate::models::user::User;
use crate::state::app::AppState;
use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct NewTodoRequest {
  content: Option<String>,
}

/// Create new todo
///
/// @param {String} [content]
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
pub async fn handle(
  req: web::HttpRequest,
  data: web::Json<NewTodoRequest>,
  state: web::Data<AppState>,
) -> impl Responder {
  let auth = match req.extensions_mut().remove::<User>() {
    Some(user) => user,
    None => return HttpResponse::BadRequest().finish(),
  };

  let content: String = match &data.content {
    Some(c) => c.into(),
    None => "".into(),
  };

  match NewTodo::create(&state.get_connection(), &auth.id, &content) {
    Ok(todo) => HttpResponse::Ok().json(todo),
    Err(_) => HttpResponse::BadRequest().finish(),
  }
}
