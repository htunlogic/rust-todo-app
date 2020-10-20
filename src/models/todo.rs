use super::super::db;
use super::super::schema::todos;
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use diesel::result;

#[derive(Queryable, PartialEq, Debug)]
pub struct Todo {
  pub id: String,
  pub user_id: String,
  pub content: String,
  pub checked: bool,
}

impl Todo {
  /// Get all todos
  pub fn all() -> Result<Vec<Todo>, result::Error> {
    todos::table.load::<Todo>(&db::establish_connection())
  }

  /// Get todos for single user
  pub fn users(user_id: &str) -> Result<Vec<Todo>, result::Error> {
    todos::table
      .filter(todos::user_id.eq(&user_id))
      .load::<Todo>(&db::establish_connection())
  }

  // Check the todo as done
  pub fn check(&mut self) -> Result<bool, result::Error> {
    self.check_as(true)
  }

  // Remove done check for the todo
  pub fn uncheck(&mut self) -> Result<bool, result::Error> {
    self.check_as(false)
  }

  /// Check or uncheck the Todo
  fn check_as(&mut self, value: bool) -> Result<bool, result::Error> {
    let target = todos::table.filter(todos::id.eq(&self.id));
    let updated = diesel::update(target)
      .set(todos::checked.eq(value))
      .execute(&db::establish_connection())?;

    if updated <= 0 {
      Err(result::Error::NotFound)
    } else {
      Ok(true)
    }
  }
}

#[derive(Insertable)]
#[table_name = "todos"]
pub struct NewTodo {
  pub user_id: String,
  pub content: String,
}

impl NewTodo {
  /// Create new todo with given parameters.
  pub fn create<'a>(user_id: &'a str, content: &'a str) -> Result<Todo, result::Error> {
    let values = Self {
      content: String::from(content),
      user_id: String::from(user_id),
    };

    diesel::insert_into(todos::table)
      .values(&values)
      .get_result::<Todo>(&db::establish_connection())
  }
}
