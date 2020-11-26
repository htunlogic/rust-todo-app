use crate::models::todo::Todo;
use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct PaginatedTodoRequest {
  page: Option<u32>,
  per_page: Option<u32>,
  checked: Option<bool>,
}

/// Authenticate the user with email and password
///
/// @param {u32} [page]
/// @param {u32} [per_page]
/// @param {bool} [checked]
///
/// Success code 200:
/// ```
/// {
///   "page": 1,
///   "per_page": 10,
///   "total": 120,
///   "last_page": 12,
///   "data": [ ... ]
/// }
/// ```
///
/// Error: 400
pub async fn handle(
  req: web::HttpRequest,
  query: web::Query<PaginatedTodoRequest>,
) -> impl Responder {
  let page = match query.page {
    Some(n) => n,
    None => 1,
  };

  let per_page = match query.per_page {
    Some(n) => n,
    None => crate::DEFAULT_PER_PAGE,
  };

  let checked = match query.checked {
    Some(c) => c,
    None => false,
  };

  println!("{:?}", req);
  let user_id: String = req.match_info().get("user_id").unwrap().parse().unwrap();

  match Todo::paginated(page, per_page, user_id, checked) {
    Ok(paginated) => HttpResponse::Ok().json(paginated),
    Err(_) => HttpResponse::BadRequest().finish(),
  }
}