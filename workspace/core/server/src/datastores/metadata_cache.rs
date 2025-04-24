use crate::datastores::cache::cache::BoscaCache;
use crate::datastores::cache::manager::BoscaCacheManager;
use crate::models::content::metadata::Metadata;
use crate::models::content::metadata_supplementary::MetadataSupplementary;
use crate::models::security::permission::Permission;
use async_graphql::Error;
use uuid::Uuid;

#[derive(Clone)]
pub struct MetadataCache {
    metadata_id: BoscaCache<Metadata>,
    metadata_supplementary_id: BoscaCache<MetadataSupplementary>,
    metadata_supplementaries: BoscaCache<Vec<Uuid>>,
    permissions: BoscaCache<Vec<Permission>>,
}

impl MetadataCache {
    pub async fn new(cache: &mut BoscaCacheManager) -> Result<Self, Error> {
        Ok(Self {
            metadata_id: cache.new_id_tiered_cache("metadata_id", 5000).await?,
            metadata_supplementary_id: cache
                .new_id_tiered_cache("metadata_supplementary_id", 5000)
                .await?,
            metadata_supplementaries: cache
                .new_id_tiered_cache("metadata_supplementaries", 5000)
                .await?,
            permissions: cache.new_id_tiered_cache("permission_cache", 5000).await?,
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
        self.metadata_supplementaries.remove(id).await;
        self.permissions.remove(id).await;
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_permissions(&self, id: &Uuid) -> Option<Vec<Permission>> {
        self.permissions.get(id).await
    }

    #[tracing::instrument(skip(self, id, permissions))]
    pub async fn set_permissions(&self, id: &Uuid, permissions: &Vec<Permission>) {
        self.permissions.set(id, permissions).await;
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn evict_permissions(&self, id: &Uuid) {
        self.permissions.remove(id).await;
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_supplementary(&self, id: &Uuid) -> Option<MetadataSupplementary> {
        self.metadata_supplementary_id.get(id).await
    }

    #[tracing::instrument(skip(self, id, supplementary))]
    pub async fn set_supplementary(&self, id: &Uuid, supplementary: &MetadataSupplementary) {
        self.metadata_supplementary_id.set(id, supplementary).await;
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn evict_supplementary(&self, id: &Uuid) {
        self.metadata_supplementary_id.remove(id).await;
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_supplementaries(&self, id: &Uuid) -> Option<Vec<Uuid>> {
        self.metadata_supplementaries.get(id).await
    }

    #[tracing::instrument(skip(self, id, supplementaries))]
    pub async fn set_supplementaries(&self, id: &Uuid, supplementaries: &Vec<Uuid>) {
        self.metadata_supplementaries.set(id, supplementaries).await;
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn evict_supplementaries(&self, id: &Uuid) {
        self.metadata_supplementaries.remove(id).await;
    }
}
