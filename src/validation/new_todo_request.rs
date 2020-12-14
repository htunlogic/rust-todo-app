use actix_web_validator::JsonConfig;

/// Request struct that will be used to extract data from the request
/// and to run validation on the extracted data.
#[derive(serde::Deserialize, validator::Validate)]
pub struct NewTodoRequest {
  #[validate(length(min = 3))]
  pub content: Option<String>,
}

// App configuration data that will setup the needed configurations on it.
pub fn app_data() -> JsonConfig {
  super::default_app_data::<NewTodoRequest>()
}
