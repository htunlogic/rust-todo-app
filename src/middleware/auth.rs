use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::models::auth::AuthenticableUser;
use crate::state::app::AppState;
use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage};
use futures::future::{ok, Ready};
pub struct LoggedGuard;

impl<S> Transform<S> for LoggedGuard
where
  S: Service<Request = ServiceRequest, Response = ServiceResponse, Error = Error>,
  S::Future: 'static,
{
  type Request = ServiceRequest;
  type Response = ServiceResponse;
  type Error = Error;
  type InitError = ();
  type Transform = LoggedGuardMiddleware<S>;
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ok(LoggedGuardMiddleware { service })
  }
}

pub struct LoggedGuardMiddleware<S> {
  service: S,
}

impl<S> Service for LoggedGuardMiddleware<S>
where
  S: Service<Request = ServiceRequest, Response = ServiceResponse, Error = Error>,
  S::Future: 'static,
{
  type Request = ServiceRequest;
  type Response = ServiceResponse;
  type Error = Error;
  type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.service.poll_ready(cx)
  }

  fn call(&mut self, req: ServiceRequest) -> Self::Future {
    match is_logged(&req) {
      Ok(auth) => {
        req.extensions_mut().insert(auth);
        let fut = self.service.call(req);
        Box::pin(async move {
          let res = fut.await?;
          Ok(res)
        })
      }
      Err(e) => {
        println!("Got error: {}", e);
        Box::pin(async move {
          Ok(ServiceResponse::new(
            req.into_parts().0,
            actix_web::HttpResponse::Unauthorized().finish(),
          ))
        })
      }
    }
  }
}

/// Check if the user making the request is logged in
fn is_logged(req: &ServiceRequest) -> Result<crate::models::user::User, String> {
  let header = match &req.headers().get("Authorization") {
    Some(head) => match head.to_str().ok() {
      Some(val) => val.to_string(),
      None => return Err(String::from("Couldn't parse the header")),
    },
    None => return Err(String::from("Couldn't retrieve header")),
  };

  let mut split = header.split_whitespace();

  let auth_type = split.next();

  if Some("Bearer") == auth_type {
    bearer_auth(match split.next() {
      Some(v) => v,
      None => "",
    })
  } else if Some("Basic") == auth_type {
    basic_auth(
      match split.next() {
        Some(v) => v,
        None => "",
      },
      req,
    )
  } else {
    Err(String::from("Not valid authentication method"))
  }
}

/// Handle JWT authentication token
fn bearer_auth(data: &str) -> Result<crate::models::user::User, String> {
  match crate::services::jwt::verify(String::from(data)) {
    Ok(user) => Ok(user),
    Err(e) => {
      println!("Got error from jwt: {:?}", e);
      Err(String::from("Something wrong with the signature"))
    }
  }
}

/// Handle basic auth authentication token
fn basic_auth(data: &str, req: &ServiceRequest) -> Result<crate::models::user::User, String> {
  let decoded = match base64::decode(data) {
    Ok(d) => match std::str::from_utf8(&d[..]) {
      Ok(s) => String::from(s),
      Err(_) => {
        return Err(String::from("Could not parse the authentication header"));
      }
    },
    Err(_) => return Err(String::from("Could not decode base64 header")),
  };

  let mut decoded = decoded.split(":");

  let email = match decoded.next() {
    Some(v) => v,
    None => "",
  };

  let password = match decoded.next() {
    Some(v) => v,
    None => "",
  };

  // We will try to get app state here and unwrap it, in case the app data does not exist
  // we want to panic, there is no recovery from it missing.
  let state = req.app_data::<AppState>().unwrap();
  match AuthenticableUser::authenticate(&state.get_connection(), &email, &password) {
    Ok((user, _)) => Ok(user),
    Err(e) => {
      println!("Basic auth error: {:?}", e);

      Err(String::from("Invalid credentials for basic auth"))
    }
  }
}
