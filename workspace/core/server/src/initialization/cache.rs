use async_graphql::Error;
use bosca_dc_client::client::Client;
use std::env;

pub async fn new_cache_client() -> Result<Client, Error> {
    let host = match env::var("CACHE_HOST") {
        Ok(host) => host,
        _ => "localhost".to_string(),
    };
    let port = match env::var("CACHE_PORT") {
        Ok(port) => port.parse::<u16>(),
        _ => Ok(2001),
    };
    let port = match port {
        Ok(port) => port,
        _ => {
            return Err(Error::new(
                "Environment variable CACHE_PORT is invalid".to_string(),
            ))
        }
    };
    let client = Client::new();
    client.connect(host, port).await?;
    Ok(client)
}
