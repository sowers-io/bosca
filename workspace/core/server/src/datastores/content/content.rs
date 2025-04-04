use crate::datastores::content::collection_permissions::CollectionPermissionsDataStore;
use crate::datastores::content::collection_workflows::CollectionWorkflowsDataStore;
use crate::datastores::content::collections::CollectionsDataStore;
use crate::datastores::content::documents::DocumentsDataStore;
use crate::datastores::content::metadata::MetadataDataStore;
use crate::datastores::content::metadata_permissions::MetadataPermissionsDataStore;
use crate::datastores::content::metadata_workflows::MetadataWorkflowsDataStore;
use crate::datastores::notifier::Notifier;
use crate::models::content::slug::{Slug, SlugType};
use async_graphql::Error;
use deadpool_postgres::Pool;
use std::sync::Arc;
use uuid::Uuid;
use crate::datastores::bible::bible::BiblesDataStore;
use crate::datastores::cache::cache::{BoscaCache, BoscaCacheInterface};
use crate::datastores::cache::manager::BoscaCacheManager;
use crate::datastores::cache::tiered_cache::TieredCacheType;
use crate::datastores::content::categories::CategoriesDataStore;
use crate::datastores::content::collection_supplementary::CollectionSupplementaryDataStore;
use crate::datastores::content::collection_templates::CollectionTemplatesDataStore;
use crate::datastores::content::guides::GuidesDataStore;
use crate::datastores::content::metadata_supplementary::MetadataSupplementaryDataStore;
use crate::datastores::content::sources::SourcesDataStore;

#[derive(Clone)]
pub struct ContentDataStore {
    slug_cache: BoscaCache<String, Slug>,
    pool: Arc<Pool>,

    pub categories: CategoriesDataStore,
    pub collections: CollectionsDataStore,
    pub collection_supplementary: CollectionSupplementaryDataStore,
    pub collection_permissions: CollectionPermissionsDataStore,
    pub collection_workflows: CollectionWorkflowsDataStore,
    pub collection_templates: CollectionTemplatesDataStore,
    pub metadata: MetadataDataStore,
    pub metadata_supplementary: MetadataSupplementaryDataStore,
    pub metadata_permissions: MetadataPermissionsDataStore,
    pub metadata_workflows: MetadataWorkflowsDataStore,
    pub documents: DocumentsDataStore,
    pub guides: GuidesDataStore,
    pub bibles: BiblesDataStore,
    pub sources: SourcesDataStore
}

impl ContentDataStore {

    pub async fn new(pool: Arc<Pool>, cache: &mut BoscaCacheManager, notifier: Arc<Notifier>) -> Self {
        Self {
            slug_cache: cache.new_string_tiered_cache("slugs", 5000, TieredCacheType::Slug).await,
            categories: CategoriesDataStore::new(Arc::clone(&pool), Arc::clone(&notifier)),
            collections: CollectionsDataStore::new(Arc::clone(&pool), Arc::clone(&notifier)),
            collection_supplementary: CollectionSupplementaryDataStore::new(Arc::clone(&pool), Arc::clone(&notifier)),
            collection_permissions: CollectionPermissionsDataStore::new(
                Arc::clone(&pool),
                cache,
                Arc::clone(&notifier),
            ).await,
            collection_workflows: CollectionWorkflowsDataStore::new(
                Arc::clone(&pool),
                Arc::clone(&notifier),
            ),
            collection_templates: CollectionTemplatesDataStore::new(
                Arc::clone(&pool),
            ),
            metadata: MetadataDataStore::new(Arc::clone(&pool), cache, Arc::clone(&notifier)).await,
            metadata_supplementary: MetadataSupplementaryDataStore::new(Arc::clone(&pool), Arc::clone(&notifier)),
            metadata_permissions: MetadataPermissionsDataStore::new(
                Arc::clone(&pool),
                cache,
                Arc::clone(&notifier),
            ).await,
            metadata_workflows: MetadataWorkflowsDataStore::new(
                Arc::clone(&pool),
                Arc::clone(&notifier),
            ),
            documents: DocumentsDataStore::new(Arc::clone(&pool), Arc::clone(&notifier)),
            guides: GuidesDataStore::new(Arc::clone(&pool), Arc::clone(&notifier)),
            sources: SourcesDataStore::new(Arc::clone(&pool)),
            bibles: BiblesDataStore::new(Arc::clone(&pool), Arc::clone(&notifier)),
            pool,
        }
    }

    pub async fn get_slug(&self, slug: &str) -> async_graphql::Result<Option<Slug>, Error> {
        let slug = slug.to_string();
        if let Some(s) = self.slug_cache.get(&slug).await {
            return Ok(Some(s));
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "select metadata_id, collection_id, profile_id from slugs where slug = $1",
            )
            .await?;
        let rows = connection.query(&stmt, &[&slug]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        let row = rows.first().unwrap();
        let metadata_id: Option<Uuid> = row.get("metadata_id");
        let collection_id: Option<Uuid> = row.get("collection_id");
        let profile_id: Option<Uuid> = row.get("profile_id");
        let s = Slug {
            id: if let Some(metadata_id) = metadata_id {
                metadata_id
            } else if let Some(collection_id) = collection_id {
                collection_id
            } else {
                profile_id.unwrap()
            },
            slug_type: if metadata_id.is_some() {
                SlugType::Metadata
            } else if collection_id.is_some() {
                SlugType::Collection
            } else {
                SlugType::Profile
            },
        };
        self.slug_cache.set(&slug, &s).await;
        Ok(Some(s))
    }
}
