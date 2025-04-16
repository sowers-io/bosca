use std::collections::HashMap;
use std::sync::Arc;
use async_graphql::extensions::apollo_persisted_queries::CacheStorage;
use async_graphql::parser::types::ExecutableDocument;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct PersistedQueriesCache {

    pub queries: Arc<RwLock<HashMap<String, ExecutableDocument>>>,
}

impl PersistedQueriesCache {
    pub fn new() -> Self {
        Self { queries: Arc::new(RwLock::new(HashMap::new())) }
    }
}

#[async_trait::async_trait]
impl CacheStorage for PersistedQueriesCache {

    #[tracing::instrument(skip(self, key))]
    async fn get(&self, key: String) -> Option<ExecutableDocument> {
        let queries = self.queries.read().await;
        let document = queries.get(&key);
        document.cloned()
    }

    async fn set(&self, _: String, _: ExecutableDocument) {
        panic!("Not Supported")
    }
}