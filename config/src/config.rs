use dotenv::dotenv;
use std::env;

pub struct Config {
  pub db_conn: String,
  pub redis_conn: String,
}

impl Config {
  pub async fn new() -> Self {
    // .env ファイルをロード
    dotenv().ok();

    let db_conn = env::var("DATABASE_URL").unwrap_or_else(|_| "mysql://root:pass@localhost:3306/appdb".to_string());
    let redis_conn = env::var("REDIS_CONN").unwrap_or_else(|_| "redis://localhost:6379/".to_string());

    Self {
      db_conn,
      redis_conn,
    }
  }
}