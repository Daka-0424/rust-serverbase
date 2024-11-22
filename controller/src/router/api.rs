use crate::handler::api::{health::health_check};
use axum::{Router, routing::get};

pub async fn create_api_router() -> Router {
  let api_router = Router::new()
    .route("/", get(health_check));

  api_router
}