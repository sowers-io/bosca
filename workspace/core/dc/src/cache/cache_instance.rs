use moka::future::Cache;

pub struct CacheInstance {
    pub(crate) id: String,
    cache: Cache<String, Vec<u8>>,
    pub max_capacity: u64,
}

impl CacheInstance {
    pub fn new(id: String, max_capacity: u64) -> Self {
        let cache = Cache::new(max_capacity);
        Self {
            id,
            cache,
            max_capacity,
        }
    }

    pub async fn get(&self, key: &str) -> Option<Vec<u8>> {
        self.cache.get(key).await
    }

    pub async fn put(&self, key: String, value: Vec<u8>) {
        self.cache.insert(key.clone(), value.clone()).await;
    }

    pub async fn delete(&self, key: &str) {
        self.cache.invalidate(key).await;
    }

    pub async fn clear(&self) {
        self.cache.invalidate_all();
    }
}
