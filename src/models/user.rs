#[macro_use]
use crate::db;
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::models;
use crate::schema::users;
use crate::virtual_schema::users_todos;
use bcrypt;
use diesel::result;
use diesel::sql_query;
use std::{error::Error, fmt};

/// Main user model that will be used for interaction with users
/// in the database. All the interaction methods should be attached
/// to this model.
#[derive(Queryable, PartialEq, Debug)]
pub struct User {
  pub id: String,
  pub email: String,
  password: String,
}

impl User {
  /// Get all users out of the db
  pub fn all() -> Result<Vec<Self>, result::Error> {
    users::table.load::<Self>(&db::establish_connection())
  }

  /// Get single user out of the database
  pub fn show(id: &str) -> Result<Self, result::Error> {
    let mut results = users::table
      .filter(users::id.eq(&id))
      .load::<Self>(&db::establish_connection())?;

    match results.pop() {
      Some(user) => Ok(user),
      _ => Err(result::Error::NotFound),
    }
  }

  /// Add todo for selected user
  pub fn add_todo(&self, todo_content: &str) -> Result<models::todo::Todo, result::Error> {
    models::todo::NewTodo::create(&self.id, &todo_content)
  }
}

/// User model that implements method to retrieve user with his todos together
#[derive(Queryable, PartialEq, Debug)]
pub struct UserWithTodo {
  pub id: String,
  pub email: String,
  todos: Vec<models::todo::Todo>,
}

/// Temporary struct for incoming data from the join sql query
#[derive(QueryableByName, PartialEq, Debug)]
#[table_name = "users_todos"]
struct TempUserWithTodo {
  id: String,
  email: String,
  content: String,
  checked: bool,
  user_id: String,
}

impl UserWithTodo {
  /// Get user struct with todos included
  pub fn show(id: &str) -> Result<Self, result::Error> {
    let results = sql_query(format!(include_str!("../../sql/user_with_todos.sql"), id))
      .load::<TempUserWithTodo>(&db::establish_connection())?;

    let mut user = match results.first() {
      Some(item) => UserWithTodo {
        id: String::from(&item.user_id),
        email: String::from(&item.email),
        todos: vec![],
      },
      _ => return Err(result::Error::NotFound),
    };

    for todo in results.iter() {
      user.todos.push(models::todo::Todo {
        id: String::from(&todo.id),
        user_id: String::from(&todo.user_id),
        content: String::from(&todo.content),
        checked: todo.checked,
      });
    }

    Ok(user)
  }
}

/// Error that will let us know we have authentication issue, either
/// with email or the password. Error itself won't be specific about it.
#[derive(Debug)]
pub struct AuthenticationError;

impl Error for AuthenticationError {
  fn description(&self) -> &str {
    "Unathorized"
  }
}

impl fmt::Display for AuthenticationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Unathorized")
  }
}

#[derive(Queryable, PartialEq, Debug)]
pub struct AuthenticableUser {
  pub id: String,
  pub email: String,
  password: String,
}

impl AuthenticableUser {
  /// Try to authenticate the user with given email and password
  pub fn authenticate<'b>(email: &'b str, password: &'b str) -> Result<User, AuthenticationError> {
    let user = match users::table
      .filter(users::email.eq(&email))
      .load::<User>(&db::establish_connection())
    {
      Ok(mut results) => match results.pop() {
        Some(item) => item,
        _ => {
          println!("Authentication: No user found with email: {}", &email);
          return Err(AuthenticationError);
        }
      },
      Err(e) => {
        println!(
          "Authentication: Something went wrong with getting the user out of db: {:?}",
          e
        );
        return Err(AuthenticationError);
      }
    };

    match bcrypt::verify(&password, &user.password) {
      Ok(_) => Ok(user),
      Err(e) => {
        println!(
          "Authentication: provided password does not match for user: {}",
          &user.email
        );
        Err(AuthenticationError)
      }
    }
  }
}

/// Struct for attributes required to create new user
/// new user can be created using this struct method create.
#[derive(Queryable, Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser {
  pub email: String,
  pub password: String,
}

impl NewUser {
  /// Create new user with email and password
  /// password will be automatically hashed into bcrypt.
  pub fn create<'a>(email: &'a str, password: &'a str) -> Result<User, result::Error> {
    let hashed_password = match bcrypt::hash(&password, bcrypt::DEFAULT_COST) {
      Ok(hashed) => hashed,
      Err(e) => {
        println!("Hashing password error: {:?}", e);
        return Err(result::Error::__Nonexhaustive);
      }
    };

    let values = Self {
      email: String::from(email),
      password: hashed_password.to_string(),
    };

    diesel::insert_into(users::table)
      .values(&values)
      .get_result::<User>(&db::establish_connection())
  }
}
