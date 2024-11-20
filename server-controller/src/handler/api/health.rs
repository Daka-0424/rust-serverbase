use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub async fn health_check() -> impl IntoResponse {
    // JSONレスポンスを構築
    let body = Json(json!({ "status": "ok" }));

    // ステータスコードとボディを返却
    (StatusCode::OK, body)
}