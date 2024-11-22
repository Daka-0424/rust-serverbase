use redis::Client;

pub async fn redis_connection(redis_url: &str) -> Result<Client, redis::RedisError> {
  let client = Client::open(redis_url)?;
  Ok(client)
}