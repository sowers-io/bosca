use crate::datastores::cache::cache::{BoscaCache, BoscaCacheInterface};
use crate::datastores::cache::manager::BoscaCacheManager;
use crate::datastores::cache::tiered_cache::TieredCacheType;
use crate::models::content::metadata::Metadata;
use uuid::Uuid;

#[derive(Clone)]
pub struct MetadataCache {
    metadata_id: BoscaCache<Uuid, Metadata>,
}

impl MetadataCache {
    pub async fn new(cache: &mut BoscaCacheManager) -> Self {
        Self {
            metadata_id: cache.new_id_tiered_cache(
                "metadata::id",
                5000,
                TieredCacheType::Metadata,
            ).await,
        }
    }

    pub async fn get_metadata(&self, id: &Uuid) -> Option<Metadata> {
        self.metadata_id.get(id).await
    }

    pub async fn get_metadata_by_version(&self, id: &Uuid, version: i32) -> Option<Metadata> {
        let metadata = self.metadata_id.get(id).await;
        if let Some(metadata) = metadata {
            if metadata.version == version {
                return Some(metadata);
            }
        }
        None
    }

    pub async fn set_metadata(&self, metadata: &Metadata) {
        if metadata.version == metadata.active_version {
            self.metadata_id.set(&metadata.id, metadata).await;
        }
    }

    pub async fn evict_metadata(&self, id: &Uuid) {
        self.metadata_id.remove(id).await;
    }
}
