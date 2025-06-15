use crate::datastores::cache::cache::BoscaCache;
use crate::datastores::cache::manager::BoscaCacheManager;
use crate::models::content::slug::{Slug, SlugType};
use async_graphql::Error;
use uuid::Uuid;

#[derive(Clone)]
pub struct SlugCache {
    slug_cache: BoscaCache<Slug>,
    metadata_slug_cache: BoscaCache<String>,
    collection_slug_cache: BoscaCache<String>,
}

impl SlugCache {
    pub async fn new(cache: &mut BoscaCacheManager) -> Result<Self, Error> {
        Ok(Self {
            slug_cache: cache.new_id_tiered_cache("slugs").await?,
            metadata_slug_cache: cache.new_id_tiered_cache("metadata_slugs").await?,
            collection_slug_cache: cache.new_id_tiered_cache("collection_slugs").await?,
        })
    }

    #[tracing::instrument(skip(self, slug))]
    pub async fn get_slug(&self, slug: &String) -> Option<Slug> {
        self.slug_cache.get(slug).await
    }

    #[tracing::instrument(skip(self, slug, slug_id))]
    pub async fn set_slug(&self, slug: &String, slug_id: &Slug) {
        self.slug_cache.set(slug, slug_id).await;
    }

    #[tracing::instrument(skip(self, id, slug))]
    pub async fn set_metadata_slug(&self, id: &Uuid, slug: &String) {
        let s = Slug {
            slug_type: SlugType::Metadata,
            id: *id
        };
        self.metadata_slug_cache.set(id, slug).await;
        self.set_slug(slug, &s).await;
    }

    #[tracing::instrument(skip(self, id, slug))]
    pub async fn set_collection_slug(&self, id: &Uuid, slug: &String) {
        let s = Slug {
            slug_type: SlugType::Collection,
            id: *id
        };
        self.collection_slug_cache.set(id, slug).await;
        self.set_slug(slug, &s).await;
    }

    // #[tracing::instrument(skip(self, slug))]
    // pub async fn evict_slug(&self, slug: &String) {
    //     self.slug_cache.remove(slug).await;
    // }
}
