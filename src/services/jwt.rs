use chrono::prelude::*;
use jsonwebtoken;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Claims {
  pub sub: String,
  pub email: String,
  pub exp: i64,
  pub created: i64,
}

/// Generate JWT for passed User
pub fn generate(user: &crate::models::user::User) -> String {
  let secret = match dotenv::var("JWT_SECRET") {
    Ok(s) => s,
    Err(_) => "".to_string(),
  };

  let duration = match dotenv::var("JWT_LIFETIME_IN_SECONDS") {
    Ok(d) => d,
    Err(_) => "300".to_string(),
  };

  let duration: i64 = duration.parse().unwrap();
  let exp = Utc::now() + chrono::Duration::seconds(duration);

  let claims = Claims {
    sub: String::from(&user.id),
    email: String::from(&user.email),
    exp: exp.timestamp(),
    created: Utc::now().timestamp(),
  };

  jsonwebtoken::encode(
    &jsonwebtoken::Header::default(),
    &claims,
    &jsonwebtoken::EncodingKey::from_secret(&secret.as_bytes()),
  )
  .unwrap_or_default()
}

/// Verify given token and return user if its okay
pub fn verify(token: String) -> Result<crate::models::user::User, jsonwebtoken::errors::Error> {
  let secret = match dotenv::var("JWT_SECRET") {
    Ok(s) => s,
    Err(_) => "".to_string(),
  };

  let token_data = jsonwebtoken::decode::<Claims>(
    &token,
    &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
    &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
  )?;

  Ok(crate::models::user::User::from_jwt(&token_data.claims))
}
