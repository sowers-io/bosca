use std::env;
use async_graphql::Error;
use crate::redis::RedisClient;

pub async fn new_redis_client(key: &str) -> Result<RedisClient, Error> {
    let host = match env::var(format!("{key}_HOST")) {
        Ok(url) => url,
        _ => "127.0.0.1".to_string(),
    };
    let port = match env::var(format!("{key}_PORT")) {
        Ok(port) => port.parse::<u16>()?,
        _ => 6380,
    };
    let password = env::var(format!("{key}_PASSWORD")).ok();
    RedisClient::new(host, port, password).await
}