use std::sync::Arc;

use server_config::config::Config;
use server_infra::redis_conn::redis_connection;

pub mod Modules{
  Config,
}

impl Modules {
  pub async fn new() -> Self {
    let config = Config::new().await;
    let redis_conn = redis_connection(&config.redis_conn).await.unwrap();

    Self {
      config: Arc::new(config),
    }
  }
}