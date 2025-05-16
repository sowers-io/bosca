use crate::datastores::cache::cache::BoscaCache;
use crate::datastores::cache::manager::BoscaCacheManager;
use async_graphql::Error;
use uuid::Uuid;

#[derive(Clone)]
pub struct GuideCache {
    guide_step_count: BoscaCache<i64>,
}

impl GuideCache {
    pub async fn new(cache: &mut BoscaCacheManager) -> Result<Self, Error> {
        Ok(Self {
            guide_step_count: cache.new_id_tiered_cache("guide_step_count", 20000).await?,
        })
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn evict_guide(&self, id: &Uuid) {
        self.guide_step_count.remove(id).await;
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_guide_step_count(&self, id: &Uuid) -> Option<i64> {
        self.guide_step_count.get(id).await
    }

    #[tracing::instrument(skip(self, id, count))]
    pub async fn set_guide_step_count(&self, id: &Uuid, count: i64) {
        self.guide_step_count.set(id, &count).await;
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn evict_guide_step_count(&self, id: &Uuid) {
        self.guide_step_count.remove(id).await;
    }
}
