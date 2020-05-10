use async_trait::async_trait;
use colored::*;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use tokio::time::delay_for;
use yaml_rust::Yaml;

use crate::actions::{Report, Runnable};
use crate::config;

use std::convert::TryFrom;
use std::time::Duration;

#[derive(Clone)]
pub struct Delay {
  name: String,
  seconds: u64,
}

impl Delay {
  pub fn is_that_you(item: &Yaml) -> bool {
    item["delay"].as_hash().is_some()
  }

  pub fn new(item: &Yaml, _with_item: Option<Yaml>) -> Delay {
    let seconds = u64::try_from(item["delay"]["seconds"].as_i64().unwrap()).expect("Invalid number of seconds");

    Delay {
      name: item["name"].as_str().unwrap().to_string(),
      seconds,
    }
  }
}

#[async_trait]
impl Runnable for Delay {
  async fn execute(&self, _context: &mut HashMap<String, Yaml>, _responses: &mut HashMap<String, Value>, _reports: &mut Vec<Report>, _pool: &mut HashMap<String, Client>, config: &config::Config) {
    delay_for(Duration::from_secs(self.seconds as u64)).await;

    if !config.quiet {
      println!("{:width$} {}{}", self.name.green(), self.seconds.to_string().cyan().bold(), "s".magenta(), width = 25);
    }
  }
}
