use async_graphql::Error;
use meilisearch_sdk::client::Client;
use std::env;
use std::sync::Arc;

pub fn new_search_client() -> Result<Arc<Client>, Error> {
    let url = match env::var("SEARCH_URL") {
        Ok(url) => url,
        _ => {
            return Err(Error::new(
                "Environment variable SEARCH_URL could not be read".to_string(),
            ))
        }
    };
    let key = match env::var("SEARCH_KEY") {
        Ok(url) => url,
        _ => {
            return Err(Error::new(
                "Environment variable SEARCH_KEY could not be read".to_string(),
            ))
        }
    };
    Ok(Arc::new(Client::new(url, Some(key))?))
}
