use actix_web_validator::JsonConfig;
use serde::Deserialize;

/// Request struct that will be used to extract data from the request
/// and to run validation on the extracted data.
#[derive(Deserialize, validator::Validate)]
pub struct NewUserRequest {
  #[validate(email, custom = "unique")]
  pub email: String,
  #[validate(length(min = 3))]
  pub password: String,
}

/// Custom function that will verify if the given email is unique
fn unique(email: &str) -> Result<(), validator::ValidationError> {
  let connection = crate::state::pool::get_single_connection();

  match crate::models::user::User::find_by_email(&connection, email) {
    Ok(res) => match res {
      Some(_user) => Err(validator::ValidationError::new("invalid_email")),
      None => Ok(()),
    },
    Err(_e) => Ok(()),
  }
}

// App configuration data that will setup the needed configurations on it.
pub fn app_data() -> JsonConfig {
  super::default_app_data::<NewUserRequest>()
}
