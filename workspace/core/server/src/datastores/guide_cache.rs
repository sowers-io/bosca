use crate::datastores::cache::cache::BoscaCache;
use crate::datastores::cache::manager::BoscaCacheManager;
use async_graphql::Error;
use uuid::Uuid;

#[derive(Clone)]
pub struct GuideCache {
    guide_step_ids: BoscaCache<Vec<i64>>,
    guide_step_count: BoscaCache<i64>,
}

impl GuideCache {

    // TODO: use version for keys too

    pub async fn new(cache: &mut BoscaCacheManager) -> Result<Self, Error> {
        Ok(Self {
            guide_step_ids: cache.new_id_tiered_cache("guide_step_ids").await?,
            guide_step_count: cache.new_id_tiered_cache("guide_step_count").await?,
        })
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn evict_guide(&self, id: &Uuid) {
        self.guide_step_count.remove(id).await;
        self.guide_step_ids.remove(id).await;
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

    #[tracing::instrument(skip(self, id))]
    pub async fn get_guide_step_ids(&self, id: &Uuid) -> Option<Vec<i64>> {
        self.guide_step_ids.get(id).await
    }

    #[tracing::instrument(skip(self, id, ids))]
    pub async fn set_guide_step_ids(&self, id: &Uuid, ids: &Vec<i64>) {
        self.guide_step_ids.set(id, ids).await;
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn evict_guide_step_ids(&self, id: &Uuid) {
        self.guide_step_ids.remove(id).await;
    }
}
