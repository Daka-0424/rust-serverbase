use axum::{Router};

use server_controller::router::api::create_api_router;

#[tokio::main]
async fn main() {
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