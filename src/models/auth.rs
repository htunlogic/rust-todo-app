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

    AuthenticableUser::verify(password.into(), &user)?;

    let token = user.generate_jwt();

    Ok((user, token))
  }

  /// Verify the bcrypt password
  fn verify(password: String, user: &User) -> Result<(), AuthenticationError> {
    match bcrypt::verify(&password, &user.password) {
      Ok(res) => {
        if res == true {
          Ok(())
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

#[cfg(test)]
mod tests {
  use crate::models::user::User;
  #[test]
  fn verify_user_password_authentication() {
    let password: String = "123".into();
    let hashed_password = match crate::bcrypt::hash(&password, bcrypt::DEFAULT_COST) {
      Ok(hashed) => hashed,
      Err(e) => panic!(e),
    };

    let user = User::new("test@test.com".into(), hashed_password);

    let response = match super::AuthenticableUser::verify(password, &user) {
      Ok(r) => r,
      Err(e) => panic!(e),
    };

    assert_eq!((), response);
  }
}
