use std::env;
use async_graphql::Error;
use async_nats::{jetstream, Client};
use async_nats::jetstream::Context;

pub async fn new_nats_client() -> Result<(Client, Context), Error> {
    let url = match env::var("NATS_URL") {
        Ok(url) => url,
        _ => {
            return Err(Error::new(
                "Environment variable NATS_URL could not be read".to_string(),
            ))
        }
    };
    let nats = async_nats::connect(&url).await?;
    let jetstream = jetstream::new(nats.clone());
    Ok((nats, jetstream))
}
