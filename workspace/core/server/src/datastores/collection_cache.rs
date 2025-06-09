use crate::datastores::cache::cache::BoscaCache;
use crate::datastores::cache::manager::BoscaCacheManager;
use crate::models::content::collection::Collection;
use crate::models::security::permission::Permission;
use async_graphql::Error;
use uuid::Uuid;

#[derive(Clone)]
pub struct CollectionCache {
    collection_id: BoscaCache<Collection>,
    parent_id: BoscaCache<Vec<Uuid>>,
    permissions: BoscaCache<Vec<Permission>>,
}

impl CollectionCache {
    pub async fn new(cache: &mut BoscaCacheManager) -> Result<Self, Error> {
        Ok(Self {
            collection_id: cache.new_id_tiered_cache("collection_id").await?,
            parent_id: cache.new_id_tiered_cache("collection_parent_id").await?,
            permissions: cache.new_id_tiered_cache("collection_permissions").await?,
        })
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_collection(&self, id: &Uuid) -> Option<Collection> {
        self.collection_id.get(id).await
    }

    #[tracing::instrument(skip(self, collection))]
    pub async fn set_collection(&self, collection: &Collection) {
        self.collection_id.set(&collection.id, collection).await;
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn evict_collection(&self, id: &Uuid) {
        self.collection_id.remove(id).await;
        self.permissions.remove(id).await;
        self.parent_id.remove(id).await;
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_parent_ids(&self, id: &Uuid) -> Option<Vec<Uuid>> {
        self.parent_id.get(id).await
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn set_parent_ids(&self, id: &Uuid, parent_ids: &Vec<Uuid>) {
        self.parent_id.set(id, parent_ids).await
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn evict_parent_ids(&self, id: &Uuid) {
        self.parent_id.remove(id).await;
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_permissions(&self, id: &Uuid) -> Option<Vec<Permission>> {
        self.permissions.get(id).await
    }

    #[tracing::instrument(skip(self, permissions))]
    pub async fn set_permissions(&self, id: &Uuid, permissions: &Vec<Permission>) {
        self.permissions.set(id, permissions).await;
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn evict_permissions(&self, id: &Uuid) {
        self.permissions.remove(id).await;
    }
}
