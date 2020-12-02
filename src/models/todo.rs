use super::super::db;
use super::super::schema::todos;
use super::Paginated;
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use diesel::result;

#[derive(Queryable, PartialEq, Debug, serde::Serialize)]
pub struct Todo {
  pub id: String,
  pub user_id: String,
  pub content: String,
  pub checked: bool,
}

impl Todo {
  /// Get paginated todos for user
  pub fn paginated(
    page: u32,
    per_page: u32,
    user_id: String,
    checked: bool,
  ) -> Result<Paginated<Todo>, result::Error> {
    let mut last_page = 1;
    let mut data: Vec<Todo> = vec![];

    let connection = db::establish_connection();

    let total = todos::table
      .filter(todos::user_id.eq(&user_id))
      .filter(todos::checked.eq(&checked))
      .count()
      .get_result(&db::establish_connection());

    let total = match total {
      Ok(count) => count,
      Err(e) => {
        println!("Todos: Counting error: {:?}", e);
        0
      }
    };

    if total > 0 {
      last_page = total as u32 / per_page;
      let skip = (page - 1) * per_page;

      data = todos::table
        .filter(todos::user_id.eq(user_id))
        .filter(todos::checked.eq(checked))
        .offset(skip as i64)
        .limit(per_page.clone() as i64)
        .load::<Todo>(&connection)?;
    }

    Ok(Paginated {
      page: page,
      per_page: per_page,
      total: total as u32,
      last_page,
      data,
    })
  }

  /// Get single todo out of the database
  pub fn show(id: &str) -> Result<Self, result::Error> {
    let mut results = todos::table
      .filter(todos::id.eq(&id))
      .load::<Self>(&db::establish_connection())?;

    match results.pop() {
      Some(todo) => Ok(todo),
      _ => Err(result::Error::NotFound),
    }
  }

  /// Get todos for single user
  pub fn users(user_id: &str) -> Result<Vec<Todo>, result::Error> {
    todos::table
      .filter(todos::user_id.eq(&user_id))
      .load::<Todo>(&db::establish_connection())
  }

  // Check the todo as done
  pub fn check(&self) -> Result<bool, result::Error> {
    self.check_as(true)
  }

  // Remove done check for the todo
  pub fn uncheck(&self) -> Result<bool, result::Error> {
    self.check_as(false)
  }

  /// Check or uncheck the Todo
  fn check_as(&self, value: bool) -> Result<bool, result::Error> {
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
