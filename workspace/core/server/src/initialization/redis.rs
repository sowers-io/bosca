use std::env;
use async_graphql::Error;
use crate::redis::RedisClient;

pub async fn new_redis_client(key: &str) -> Result<RedisClient, Error> {
    let host = match env::var(format!("{}_HOST", key)) {
        Ok(url) => url,
        _ => "127.0.0.1".to_string(),
    };
    let port = match env::var(format!("{}_PORT", key)) {
        Ok(port) => port.parse::<u16>()?,
        _ => 6380,
    };
    let password = env::var(format!("{}_PASSWORD", key)).ok();
    RedisClient::new(host, port, password).await
}