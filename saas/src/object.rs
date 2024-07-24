// object.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Object {
  pub id: String,
  pub data: serde_json::Value,
}
