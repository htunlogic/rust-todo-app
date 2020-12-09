pub mod new_todo_request;
pub mod new_user_request;

use actix_web::error::{Error as ActixError, InternalError};
use actix_web::FromRequest;
use actix_web::{HttpRequest, HttpResponse};
use actix_web_validator::error::Error;
use actix_web_validator::{Json, JsonConfig};
use serde::de::DeserializeOwned;
use validator::Validate;

/// Default error handler for validation request handling
pub fn default_error_handler(error: Error, _req: &HttpRequest) -> ActixError {
  let res = match &error {
    Error::Validate(errors) => HttpResponse::UnprocessableEntity().json(errors),
    Error::Deserialize(_) => HttpResponse::UnprocessableEntity().finish(),
    Error::JsonPayloadError(_) => HttpResponse::BadRequest().finish(),
  };

  InternalError::from_response(error, res).into()
}

/// Default app_data setup for a given type
pub fn default_app_data<T: DeserializeOwned + Validate + 'static>() -> JsonConfig {
  Json::<T>::configure(|cfg| cfg.error_handler(|err, req| default_error_handler(err, req)))
}
