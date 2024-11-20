use axum::{Router};

use server_infra::redis_conn::redis_connection;
use server_controller::router::api::create_api_router;

#[tokio::main]
async fn main() {
    let redis_url = "redis://127.0.0.1:6379/";
    let redis_conn = redis_connection(redis_url).await.unwrap();

    let api = create_api_router().await;

    let app = Router::new()
        .nest("/", api);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}