use std::sync::Arc;
use async_graphql::Error;
use deadpool_postgres::Pool;
use uuid::Uuid;
use crate::datastores::content::collection_permissions::CollectionPermissionsDataStore;
use crate::datastores::content::collection_workflows::CollectionWorkflowsDataStore;
use crate::datastores::content::collections::CollectionsDataStore;
use crate::datastores::content::metadata::MetadataDataStore;
use crate::datastores::content::metadata_permissions::MetadataPermissionsDataStore;
use crate::datastores::content::metadata_workflows::MetadataWorkflowsDataStore;
use crate::datastores::notifier::Notifier;
use crate::models::content::slug::{Slug, SlugType};
use crate::models::content::source::Source;

#[derive(Clone)]
pub struct ContentDataStore {
    pool: Arc<Pool>,

    pub collections: CollectionsDataStore,
    pub collection_permissions: CollectionPermissionsDataStore,
    pub collection_workflows: CollectionWorkflowsDataStore,
    pub metadata: MetadataDataStore,
    pub metadata_permissions: MetadataPermissionsDataStore,
    pub metadata_workflows: MetadataWorkflowsDataStore,
}

impl ContentDataStore {
    pub fn new(pool: Arc<Pool>, notifier: Arc<Notifier>) -> Self {
        Self {
            collections: CollectionsDataStore::new(Arc::clone(&pool), Arc::clone(&notifier)),
            collection_permissions: CollectionPermissionsDataStore::new(Arc::clone(&pool), Arc::clone(&notifier)),
            collection_workflows: CollectionWorkflowsDataStore::new(Arc::clone(&pool), Arc::clone(&notifier)),
            metadata: MetadataDataStore::new(Arc::clone(&pool), Arc::clone(&notifier)),
            metadata_permissions: MetadataPermissionsDataStore::new(Arc::clone(&pool), Arc::clone(&notifier)),
            metadata_workflows: MetadataWorkflowsDataStore::new(Arc::clone(&pool), Arc::clone(&notifier)),
            pool,
        }
    }

    pub async fn get_sources(&self) -> async_graphql::Result<Vec<Source>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from sources order by name asc")
            .await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn get_source_by_id(&self, id: &Uuid) -> async_graphql::Result<Option<Source>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from sources where id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        Ok(rows.first().map(|r| r.into()))
    }

    pub async fn get_source_by_name(&self, name: &String) -> async_graphql::Result<Option<Source>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from sources where name = $1")
            .await?;
        let rows = connection.query(&stmt, &[name]).await?;
        Ok(rows.first().map(|r| r.into()))
    }

    pub async fn get_slug(&self, slug: &str) -> async_graphql::Result<Option<Slug>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select metadata_id, collection_id from slugs where slug = $1")
            .await?;
        let slug = slug.to_string();
        let rows = connection.query(&stmt, &[&slug]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        let row = rows.first().unwrap();
        let metadata_id: Option<Uuid> = row.get("metadata_id");
        let collection_id: Option<Uuid> = row.get("collection_id");
        Ok(Some(Slug {
            id: if let Some(metadata_id) = metadata_id {
                metadata_id
            } else {
                collection_id.unwrap()
            },
            slug_type: if metadata_id.is_some() {
                SlugType::Metadata
            } else {
                SlugType::Collection
            },
        }))
    }
}