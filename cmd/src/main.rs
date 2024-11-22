use tower::ServiceBuilder;
use axum::{Router, extract::Extension};
use sea_orm::DatabaseConnection;
use redis::Client;

use config::config::Config;
use infra::db_conn::db_connection;
use infra::redis_conn::redis_connection;
use controller::router::api::create_api_router;

use std::sync::Arc;

struct AppState {
  config: Config,
  db_conn: DatabaseConnection,
  redis_conn: Client,
}

#[tokio::main]
async fn main() {
  let config = Config::new().await;

  let db_conn = db_connection(&config.db_conn).await.unwrap();
  let redis_conn = redis_connection(&config.redis_conn).await.unwrap();

  let shared_state = Arc::new(AppState {
    config,
    db_conn,
    redis_conn,
  });

  let api = create_api_router().await;

  let app = Router::new()
    .nest("/", api)
    .layer(Extension(shared_state));

  let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
    .await
    .unwrap();

  axum::serve(listener, app)
    .await
    .unwrap();
}