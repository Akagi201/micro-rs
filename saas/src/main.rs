use std::sync::Arc;

use axum::{
  extract::{Query, State},
  http::StatusCode,
  routing::get,
  Json, Router,
};
use serde::{Deserialize, Serialize};
use store::Store;
use tower_http::cors::CorsLayer;

mod error;
mod object;
mod store;

#[derive(Deserialize, Debug)]
struct GetObjectReq {
  pub id: String,
}

#[derive(Serialize, Debug)]
struct GetObjectResp {
  data: serde_json::Value,
}

#[derive(Serialize, Debug)]
struct GetObjectsResp {
  data: Vec<serde_json::Value>,
}

async fn get_object(
  State(data): State<Arc<Store>>,
  Query(params): Query<GetObjectReq>,
) -> (StatusCode, Json<GetObjectResp>) {
  let object = data.objects.get(&params.id);
  match object {
    Some(object) => (StatusCode::OK, Json(GetObjectResp { data: object.data.clone() })),
    None => (StatusCode::NOT_FOUND, Json(GetObjectResp { data: serde_json::Value::Null })),
  }
}

async fn get_objects(
  State(data): State<Arc<Store>>,
) -> (StatusCode, Json<GetObjectsResp>) {
  // let objects = data.objects.values().map(|object| object.data.clone()).collect();
  data.objects.clone().
  (StatusCode::OK, Json(GetObjectsResp { data: objects }))
}

#[tokio::main]
async fn main() {
  let app = Router::new()
    .route("/objects", get(get_object))
    .layer(CorsLayer::permissive())
    .with_state(Arc::new(Store::new("data.json")));

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

  axum::serve(listener, app).await.unwrap();
}
