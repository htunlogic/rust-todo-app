use crony::{Job, Runner, Schedule};
use dotenv::dotenv;
use std::env;
use std::str::FromStr;

struct ExampleJob;
impl Job for ExampleJob {
  fn schedule(&self) -> Schedule {
    Schedule::from_str("0 * * * * *").unwrap()
  }

  fn handle(&self) {
    println!("Hello, I am cron job running at: {}", self.now());
  }
}

/// Cron runner
pub fn run_crons() {
  dotenv().ok();
  let activate_cron = env::var("CRON_ACTIVE").unwrap_or("false".into());
  if activate_cron == String::from("true") {
    println!("Starting cron runner in its own thread...");
    Runner::new().add(Box::new(ExampleJob)).run();
  }
}
