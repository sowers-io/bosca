use std::env;
use async_graphql::Error;
use crate::redis::RedisClient;

pub async fn new_redis_client(key: &str) -> Result<RedisClient, Error> {
    let url = match env::var(key) {
        Ok(url) => url,
        _ => "redis://127.0.0.1:6380".to_string(),
    };
    RedisClient::new(url).await
}