use async_graphql::Error;
use crate::datastores::cache::manager::BoscaCacheManager;
use crate::models::content::metadata::Metadata;
use uuid::Uuid;
use crate::datastores::cache::cache::BoscaCache;

#[derive(Clone)]
pub struct MetadataCache {
    metadata_id: BoscaCache<Metadata>,
}

impl MetadataCache {
    pub async fn new(cache: &mut BoscaCacheManager) -> Result<Self, Error> {
        Ok(Self {
            metadata_id: cache.new_id_tiered_cache(
                "metadata_id",
                5000,
            ).await?,
        })
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_metadata(&self, id: &Uuid) -> Option<Metadata> {
        self.metadata_id.get(id).await
    }

    #[tracing::instrument(skip(self, id, version))]
    pub async fn get_metadata_by_version(&self, id: &Uuid, version: i32) -> Option<Metadata> {
        let metadata = self.metadata_id.get(id).await;
        if let Some(metadata) = metadata {
            if metadata.version == version {
                return Some(metadata);
            }
        }
        None
    }

    #[tracing::instrument(skip(self, metadata))]
    pub async fn set_metadata(&self, metadata: &Metadata) {
        if metadata.version == metadata.active_version {
            self.metadata_id.set(&metadata.id, metadata).await;
        }
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn evict_metadata(&self, id: &Uuid) {
        self.metadata_id.remove(id).await;
    }
}
