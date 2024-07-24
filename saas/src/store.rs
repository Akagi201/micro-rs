use std::{borrow::Borrow, fs::File, io::BufReader, sync::Arc};

use scc::HashMap;

use crate::object::Object;

#[derive(Debug, Clone)]
pub struct Store {
  pub objects: Arc<HashMap<String, Object>>,
  file_path: String,
}

impl Store {
  pub fn new(file_path: &str) -> Self {
    let objects = Arc::new(HashMap::new());
    Store { objects, file_path: file_path.to_string() }
  }
  fn load(file_path: &str) -> HashMap<String, Object> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
  }
  pub async fn shutdown(&self) -> std::io::Result<()> {
    let data = self.objects.clone();
    let json = serde_json::to_string(&*data).unwrap();
    tokio::fs::write(&self.file_path, json).await
  }
}
