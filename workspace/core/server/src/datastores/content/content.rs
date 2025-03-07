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
use crate::datastores::content::categories::CategoriesDataStore;
use crate::datastores::content::collection_templates::CollectionTemplatesDataStore;
use crate::datastores::content::guides::GuidesDataStore;
use crate::datastores::content::sources::SourcesDataStore;

#[derive(Clone)]
pub struct ContentDataStore {
    pool: Arc<Pool>,

    pub categories: CategoriesDataStore,
    pub collections: CollectionsDataStore,
    pub collection_permissions: CollectionPermissionsDataStore,
    pub collection_workflows: CollectionWorkflowsDataStore,
    pub collection_templates: CollectionTemplatesDataStore,
    pub metadata: MetadataDataStore,
    pub metadata_permissions: MetadataPermissionsDataStore,
    pub metadata_workflows: MetadataWorkflowsDataStore,
    pub documents: DocumentsDataStore,
    pub guides: GuidesDataStore,
    pub sources: SourcesDataStore
}

impl ContentDataStore {
    pub fn new(pool: Arc<Pool>, notifier: Arc<Notifier>) -> Self {
        Self {
            categories: CategoriesDataStore::new(Arc::clone(&pool), Arc::clone(&notifier)),
            collections: CollectionsDataStore::new(Arc::clone(&pool), Arc::clone(&notifier)),
            collection_permissions: CollectionPermissionsDataStore::new(
                Arc::clone(&pool),
                Arc::clone(&notifier),
            ),
            collection_workflows: CollectionWorkflowsDataStore::new(
                Arc::clone(&pool),
                Arc::clone(&notifier),
            ),
            collection_templates: CollectionTemplatesDataStore::new(
                Arc::clone(&pool),
                Arc::clone(&notifier),
            ),
            metadata: MetadataDataStore::new(Arc::clone(&pool), Arc::clone(&notifier)),
            metadata_permissions: MetadataPermissionsDataStore::new(
                Arc::clone(&pool),
                Arc::clone(&notifier),
            ),
            metadata_workflows: MetadataWorkflowsDataStore::new(
                Arc::clone(&pool),
                Arc::clone(&notifier),
            ),
            documents: DocumentsDataStore::new(Arc::clone(&pool), Arc::clone(&notifier)),
            guides: GuidesDataStore::new(Arc::clone(&pool), Arc::clone(&notifier)),
            sources: SourcesDataStore::new(Arc::clone(&pool)),
            pool,
        }
    }

    pub async fn get_slug(&self, slug: &str) -> async_graphql::Result<Option<Slug>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "select metadata_id, collection_id, profile_id from slugs where slug = $1",
            )
            .await?;
        let slug = slug.to_string();
        let rows = connection.query(&stmt, &[&slug]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        let row = rows.first().unwrap();
        let metadata_id: Option<Uuid> = row.get("metadata_id");
        let collection_id: Option<Uuid> = row.get("collection_id");
        let profile_id: Option<Uuid> = row.get("profile_id");
        Ok(Some(Slug {
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
        }))
    }
}
