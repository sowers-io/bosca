use async_graphql::Error;
use std::env;
use std::sync::Arc;
use crate::search::search::SearchClient;

pub fn new_search_client() -> Result<Arc<SearchClient>, Error> {
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
    Ok(Arc::new(SearchClient::new(url, key)?))
}
