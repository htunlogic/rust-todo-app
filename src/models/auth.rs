use super::user::User;
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::schema::users;
use std::{error::Error, fmt};

#[derive(Queryable, PartialEq, Debug, serde::Deserialize)]
pub struct AuthenticableUser {
  pub email: String,
  pub password: String,
}

impl AuthenticableUser {
  /// Try to authenticate the user with given email and password
  pub fn authenticate<'b>(
    connection: &crate::diesel::PgConnection,
    email: &'b str,
    password: &'b str,
  ) -> Result<(User, String), AuthenticationError> {
    let user = match users::table
      .filter(users::email.eq(&email))
      .load::<User>(connection)
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
      Ok(res) => {
        if res == true {
          let token = user.generate_jwt();
          Ok((user, token))
        } else {
          println!("Authentication: bcrypt verify error for: {}", &user.email);
          Err(AuthenticationError)
        }
      }
      Err(e) => {
        println!(
          "Authentication: bcrypt verify error: {}, for: {}",
          e, &user.email
        );
        Err(AuthenticationError)
      }
    }
  }
}

/// Error that will let us know we have authentication issue, either
/// with email or the password. Error itself won't be specific about it.
#[derive(Debug)]
pub struct AuthenticationError;

impl Error for AuthenticationError {
  fn description(&self) -> &str {
    "Unauthorized"
  }
}

impl fmt::Display for AuthenticationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Unauthorized")
  }
}
