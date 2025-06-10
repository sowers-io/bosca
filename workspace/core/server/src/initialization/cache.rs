use async_graphql::Error;
use std::env;
use crate::redis::RedisClient;

pub async fn new_cache_client() -> Result<RedisClient, Error> {
    let host = match env::var("CACHE_HOST") {
        Ok(host) => host,
        _ => "127.0.0.1".to_string(),
    };
    let password = env::var("CACHE_PASSWORD").ok();
    let port = match env::var("CACHE_PORT") {
        Ok(port) => port.parse::<u16>(),
        _ => Ok(6380),
    };
    let port = match port {
        Ok(port) => port,
        _ => {
            return Err(Error::new(
                "Environment variable CACHE_PORT is invalid".to_string(),
            ))
        }
    };
    let client = RedisClient::new_cache(host, port, password).await?;
    Ok(client)
}
