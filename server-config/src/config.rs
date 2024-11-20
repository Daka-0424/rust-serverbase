use dotenv::dotenv;
use std::env;

pub struct Config {
  pub redis_conn: String,
}

impl Config {
  pub async fn new() -> Self {
    // .env ファイルをロード
    dotenv().ok();

    // 環境変数 REDIS_CONN を取得（デフォルト値を設定可能）
    let redis_conn = env::var("REDIS_CONN").unwrap_or_else(|_| "redis://127.0.0.1:6379/".to_string());

    Self {
      redis_conn,
    }
  }
}