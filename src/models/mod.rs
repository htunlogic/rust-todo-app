pub mod auth;
pub mod todo;
pub mod user;

#[derive(serde::Serialize)]
pub struct Paginated<T> {
  page: u32,
  per_page: u32,
  total: u32,
  last_page: u32,
  data: Vec<T>,
}
