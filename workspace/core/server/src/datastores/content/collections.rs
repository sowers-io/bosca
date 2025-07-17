use crate::context::BoscaContext;
use crate::datastores::collection_cache::CollectionCache;
use crate::datastores::content::tag::{update_collection_etag, update_metadata_etag};
use crate::datastores::content::util::{build_find_args, build_ordering, build_ordering_names};
use crate::datastores::notifier::Notifier;
use crate::datastores::slug_cache::SlugCache;
use crate::models::content::category::Category;
use crate::models::content::collection::{
    Collection, CollectionChild, CollectionChildInput, CollectionInput, CollectionType,
};
use crate::models::content::collection_metadata_relationship::{
    CollectionMetadataRelationship, CollectionMetadataRelationshipInput,
};
use crate::models::content::find_query::FindQueryInput;
use crate::models::content::metadata::Metadata;
use crate::models::security::permission::{Permission, PermissionAction};
use crate::models::workflow::enqueue_request::EnqueueRequest;
use crate::workflow::core_workflow_ids::COLLECTION_UPDATE_STORAGE;
use async_graphql::*;
use bosca_database::TracingPool;
use deadpool_postgres::{GenericClient, Transaction};
use log::error;
use postgres_types::ToSql;
use serde_json::Value;
use std::collections::HashSet;
use std::sync::Arc;
use tokio_postgres::Statement;
use uuid::Uuid;

#[derive(Clone)]
pub struct CollectionsDataStore {
    pool: TracingPool,
    notifier: Arc<Notifier>,
    cache: CollectionCache,
    slug_cache: SlugCache,
}

impl CollectionsDataStore {
    pub async fn new(
        pool: TracingPool,
        cache: CollectionCache,
        slug_cache: SlugCache,
        notifier: Arc<Notifier>,
    ) -> Result<Self, Error> {
        Ok(Self {
            cache,
            slug_cache,
            pool,
            notifier,
        })
    }

    #[tracing::instrument(skip(self, ctx, id))]
    pub async fn on_collection_changed(&self, ctx: &BoscaContext, id: &Uuid) -> Result<(), Error> {
        self.update_storage(ctx, id).await?;
        if let Err(e) = self.notifier.collection_changed(id).await {
            error!("Failed to notify collection changes: {e:?}");
        }
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id))]
    async fn on_metadata_changed(&self, ctx: &BoscaContext, id: &Uuid) -> Result<(), Error> {
        ctx.content.metadata.on_metadata_changed(ctx, id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_slug(&self, id: &Uuid) -> Result<Option<String>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select slug from slugs where collection_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        Ok(rows.first().unwrap().get("slug"))
    }

    #[tracing::instrument(skip(self, query))]
    pub async fn find_system(&self, query: &FindQueryInput) -> Result<Vec<Collection>, Error> {
        let connection = self.pool.get().await?;
        let category_ids = query.get_category_ids();
        let mut names = Vec::new();
        let (query, values) = build_find_args(
            "collection",
            "select c.* from collections as c ",
            "c",
            "system_attributes",
            "system_attributes",
            query,
            &category_ids,
            &query.trait_ids,
            false,
            &mut names,
        );
        let stmt = connection.prepare_cached(query.as_str()).await?;
        let rows = connection.query(&stmt, values.as_slice()).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, query))]
    pub async fn find(&self, query: &FindQueryInput) -> Result<Vec<Collection>, Error> {
        let connection = self.pool.get().await?;
        let category_ids = query.get_category_ids();
        let mut names = Vec::new();
        let (query, values) = build_find_args(
            "collection",
            "select c.* from collections as c ",
            "c",
            "attributes",
            "attributes",
            query,
            &category_ids,
            &query.trait_ids,
            false,
            &mut names,
        );
        let stmt = connection.prepare_cached(query.as_str()).await?;
        let rows = connection.query(&stmt, values.as_slice()).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, query))]
    pub async fn find_count(&self, query: &mut FindQueryInput) -> Result<i64, Error> {
        let connection = self.pool.get().await?;
        let category_ids = query.get_category_ids();
        let mut names = Vec::new();
        let (query, values) = build_find_args(
            "collection",
            "select count(*) as count from collections c ",
            "c",
            "attributes",
            "attributes",
            query,
            &category_ids,
            &query.trait_ids,
            true,
            &mut names,
        );
        let stmt = connection.prepare_cached(query.as_str()).await?;
        let rows = connection.query(&stmt, values.as_slice()).await?;
        if rows.is_empty() {
            Ok(0)
        } else {
            Ok(rows.first().unwrap().get("count"))
        }
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_categories(&self, id: &Uuid) -> Result<Vec<Category>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select c.* from collection_categories mc inner join categories c on (mc.category_id = c.id) where collection_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, ctx, id, category_ids))]
    pub async fn set_categories(
        &self,
        ctx: &BoscaContext,
        id: &Uuid,
        category_ids: &Vec<Uuid>,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        txn.execute(
            "delete from collection_categories where collection_id = $1",
            &[id],
        )
        .await?;
        for category_id in category_ids {
            let stmt = txn
                .prepare_cached(
                    "insert into collection_categories (collection_id, category_id) values ($1, $2)",
                )
                .await?;
            txn.execute(&stmt, &[id, &category_id]).await?;
        }
        txn.commit().await?;
        self.cache.evict_collection(id).await;
        self.on_collection_changed(ctx, id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_trait_ids(&self, id: &Uuid) -> Result<Vec<String>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select trait_id from collection_traits where collection_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        Ok(rows
            .iter()
            .map(|r| {
                let id: String = r.get("trait_id");
                id
            })
            .collect())
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get(&self, id: &Uuid) -> Result<Option<Collection>, Error> {
        if let Some(collection) = self.cache.get_collection(id).await {
            return Ok(Some(collection));
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from collections where id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        let collection = rows.first().unwrap().into();
        self.cache.set_collection(&collection).await;
        Ok(Some(collection))
    }

    #[tracing::instrument(skip(self, id, offset, limit))]
    pub async fn get_parents(
        &self,
        id: &Uuid,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<Collection>, Error> {
        if let Some(parent_ids) = self.cache.get_parent_ids(id).await {
            let mut parents = Vec::new();
            for parent_id in parent_ids {
                let parent = self.get(&parent_id).await?;
                if let Some(parent) = parent {
                    parents.push(parent);
                }
            }
            return Ok(parents);
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select c.id from collections c inner join collection_items ci on (c.id = ci.collection_id and c.deleted = false) where ci.child_collection_id = $1 offset $2 limit $3")
            .await?;
        let rows = connection.query(&stmt, &[id, &offset, &limit]).await?;
        let parent_ids: Vec<Uuid> = rows.iter().map(|r| r.get(0)).collect();
        self.cache.set_parent_ids(id, &parent_ids).await;
        let mut parents = Vec::new();
        for parent_id in parent_ids {
            let parent = self.get(&parent_id).await?;
            if let Some(parent) = parent {
                parents.push(parent);
            }
        }
        Ok(parents)
    }

    #[tracing::instrument(skip(
        self,
        ctx,
        collection_id,
        child_collection_id,
        child_metadata_id,
        attributes
    ))]
    pub async fn set_child_item_attributes(
        &self,
        ctx: &BoscaContext,
        collection_id: &Uuid,
        child_collection_id: Option<Uuid>,
        child_metadata_id: Option<Uuid>,
        attributes: Option<Value>,
    ) -> Result<(), Error> {
        if child_collection_id.is_none() && child_metadata_id.is_none() {
            return Err(Error::new(
                "you must supply either a child collection id or child metadata id",
            ));
        }
        if child_collection_id.is_some() && child_metadata_id.is_some() {
            return Err(Error::new(
                "you can only supply either a child collection id or child metadata id",
            ));
        }
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        if let Some(child_id) = child_collection_id {
            let stmt = txn.prepare_cached("update collection_items set attributes = $1 where collection_id = $2 and child_collection_id = $3").await?;
            txn.execute(&stmt, &[&attributes, collection_id, &child_id])
                .await?;
        } else if let Some(child_id) = child_metadata_id {
            let stmt = txn.prepare_cached("update collection_items set attributes = $1 where collection_id = $2 and child_metadata_id = $3").await?;
            txn.execute(&stmt, &[&attributes, collection_id, &child_id])
                .await?;
        }
        update_collection_etag(&txn, collection_id).await?;
        txn.commit().await?;
        if let Some(child_id) = child_collection_id {
            self.on_collection_changed(ctx, &child_id).await?;
        } else if let Some(child_id) = child_metadata_id {
            self.on_metadata_changed(ctx, &child_id).await?;
        }
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id, public))]
    pub async fn set_public(
        &self,
        ctx: &BoscaContext,
        id: &Uuid,
        public: bool,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached("update collections set public = $1, modified = now() where id = $2")
            .await?;
        txn.execute(&stmt, &[&public, id]).await?;
        update_collection_etag(&txn, id).await?;
        txn.commit().await?;
        self.on_collection_changed(ctx, id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id, public))]
    pub async fn set_public_list(
        &self,
        ctx: &BoscaContext,
        id: &Uuid,
        public: bool,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached(
                "update collections set public_list = $1, modified = now() where id = $2",
            )
            .await?;
        txn.execute(&stmt, &[&public, id]).await?;
        update_collection_etag(&txn, id).await?;
        txn.commit().await?;
        self.on_collection_changed(ctx, id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id, locked))]
    pub async fn set_locked(
        &self,
        ctx: &BoscaContext,
        id: &Uuid,
        locked: bool,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached("update collections set locked = $1, modified = now() where id = $2")
            .await?;
        txn.execute(&stmt, &[&locked, id]).await?;
        txn.commit().await?;
        self.cache.evict_collection(id).await;
        self.on_collection_changed(ctx, id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id))]
    pub async fn mark_deleted(&self, ctx: &BoscaContext, id: &Uuid) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("update collections set deleted = true, modified = now() where id = $1")
            .await?;
        connection.execute(&stmt, &[id]).await?;
        self.on_collection_changed(ctx, id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id))]
    pub async fn delete(&self, ctx: &BoscaContext, id: &Uuid) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "select collection_id from collection_items where child_collection_id = $1",
            )
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        let collection_ids: Vec<Uuid> = rows.iter().map(|r| r.get("collection_id")).collect();
        let stmt = connection
            .prepare_cached("delete from collections where id = $1")
            .await?;
        connection.execute(&stmt, &[id]).await?;
        self.on_collection_changed(ctx, id).await?;
        for collection_id in collection_ids {
            self.on_collection_changed(ctx, &collection_id).await?;
        }
        Ok(())
    }

    #[tracing::instrument(skip(self, collection, offset, limit))]
    pub async fn get_children(
        &self,
        collection: &Collection,
        offset: i64,
        limit: i64,
        state: &Option<String>,
    ) -> Result<Vec<CollectionChild>, Error> {
        let mut values = Vec::new();
        let mut names = Vec::new();
        values.push(&collection.id as &(dyn ToSql + Sync));
        if let Some(state) = state {
            values.push(state as &(dyn ToSql + Sync));
        }
        let ordering = if let Some(ordering) = &collection.ordering {
            build_ordering_names(ordering, &mut names);
            build_ordering(
                "collections",
                "collections.attributes",
                "metadata.attributes",
                "collection_items.attributes",
                if state.is_some() { 3 } else { 2 },
                ordering,
                &mut values,
                &names,
            )
            .0
        } else {
            String::new()
        };
        let mut query = "select child_collection_id, child_metadata_id, collection_items.attributes as attributes from collection_items ".to_owned();
        if state.is_some() {
            query.push_str(" left join collections on (child_collection_id = collections.id and collections.workflow_state_id = $2) ");
            query.push_str(" left join metadata on (child_metadata_id = metadata.id and metadata.workflow_state_id = $2) ");
        } else {
            query.push_str(" left join collections on (child_collection_id = collections.id) ");
            query.push_str(" left join metadata on (child_metadata_id = metadata.id) ");
        }
        query.push_str(" where collection_id = $1 and ((collections.id is not null and (collections.deleted is null or collections.deleted = false)) or (metadata.id is not null and (metadata.deleted is null or metadata.deleted = false))) ");
        if !ordering.is_empty() {
            query.push_str(ordering.as_str());
        } else {
            query.push_str(" order by lower(collections.name) asc, lower(metadata.name) asc");
        }
        query.push_str(
            format!(" offset ${} limit ${}", values.len() + 1, values.len() + 2).as_str(),
        );
        values.push(&offset as &(dyn ToSql + Sync));
        values.push(&limit as &(dyn ToSql + Sync));
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached(query.as_str()).await?;
        let rows = connection.query(&stmt, values.as_slice()).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, collection))]
    pub async fn get_children_count(
        &self,
        collection: &Collection,
        state: &Option<String>,
    ) -> Result<i64, Error> {
        let mut query = "select count(*) as count from collection_items ".to_owned();
        if state.is_some() {
            query.push_str(" left join collections on (child_collection_id = collections.id and collections.workflow_state_id = $2) ");
            query.push_str(" left join metadata on (child_metadata_id = metadata.id and metadata.workflow_state_id = $2) ");
        } else {
            query.push_str(" left join collections on (child_collection_id = collections.id) ");
            query.push_str(" left join metadata on (child_metadata_id = metadata.id) ");
        }
        query.push_str(" where collection_id = $1 and ((collections.id is not null and (collections.deleted is null or collections.deleted = false)) or (metadata.id is not null and (metadata.deleted is null or metadata.deleted = false))) ");
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached(query.as_str()).await?;
        let rows = if let Some(state) = state {
            connection
                .query_one(&stmt, &[&collection.id, state])
                .await?
        } else {
            connection.query_one(&stmt, &[&collection.id]).await?
        };
        Ok(rows.get(0))
    }

    #[tracing::instrument(skip(self, collection, offset, limit, state))]
    pub async fn get_expanded_metadata(
        &self,
        collection: &Collection,
        offset: i64,
        limit: i64,
        state: &Option<String>,
    ) -> Result<Vec<CollectionChild>, Error> {
        let mut values = Vec::new();
        let mut names = Vec::new();
        values.push(&collection.id as &(dyn ToSql + Sync));
        if let Some(state) = state {
            values.push(state as &(dyn ToSql + Sync));
        }
        let ordering = if let Some(ordering) = &collection.ordering {
            build_ordering_names(ordering, &mut names);
            build_ordering(
                "metadata",
                "",
                "metadata.attributes",
                "child.attributes",
                if state.is_some() { 3 } else { 2 },
                ordering,
                &mut values,
                &names,
            )
            .0
        } else {
            String::new()
        };
        let mut query = "select child.child_collection_id as child_collection_id, child.child_metadata_id as child_metadata_id, child.attributes as attributes from collection_items parent inner join collection_items as child on (parent.child_collection_id = child.collection_id and parent.child_collection_id is not null) ".to_owned();
        if state.is_some() {
            query.push_str(" left join metadata on (child.child_metadata_id = metadata.id and metadata.workflow_state_id = $2) ");
        } else {
            query.push_str(" left join metadata on (child.child_metadata_id = metadata.id) ");
        }
        query.push_str(" where parent.collection_id = $1 and (metadata.id is not null and (metadata.deleted is null or metadata.deleted = false)) ");
        if !ordering.is_empty() {
            query.push_str(ordering.as_str());
        } else {
            query.push_str(" order by lower(metadata.name) asc");
        }
        query.push_str(
            format!(" offset ${} limit ${}", values.len() + 1, values.len() + 2).as_str(),
        );
        values.push(&offset as &(dyn ToSql + Sync));
        values.push(&limit as &(dyn ToSql + Sync));
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached(query.as_str()).await?;
        let rows = connection.query(&stmt, values.as_slice()).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, collection, state))]
    pub async fn get_expanded_metadata_count(
        &self,
        collection: &Collection,
        state: &Option<String>,
    ) -> Result<i64, Error> {
        let mut values = Vec::new();
        values.push(&collection.id as &(dyn ToSql + Sync));
        if let Some(state) = state {
            values.push(state as &(dyn ToSql + Sync));
        }
        let mut query = "select count(*) from collection_items parent inner join collection_items as child on (parent.child_collection_id = child.collection_id and parent.child_collection_id is not null) ".to_owned();
        if state.is_some() {
            query.push_str(" left join metadata on (child.child_metadata_id = metadata.id and metadata.workflow_state_id = $2) ");
        } else {
            query.push_str(" left join metadata on (child.child_metadata_id = metadata.id) ");
        }
        query.push_str(" where parent.collection_id = $1 and (metadata.id is not null and (metadata.deleted is null or metadata.deleted = false)) ");
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached(query.as_str()).await?;
        let rows = connection.query_one(&stmt, values.as_slice()).await?;
        if rows.is_empty() {
            return Ok(0);
        }
        Ok(rows.get(0))
    }

    #[tracing::instrument(skip(self, collection, offset, limit))]
    pub async fn get_child_collections(
        &self,
        collection: &Collection,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<Collection>, Error> {
        let mut values = Vec::new();
        let mut names = Vec::new();
        values.push(&collection.id as &(dyn ToSql + Sync));
        let ordering = if let Some(ordering) = &collection.ordering {
            build_ordering_names(ordering, &mut names);
            build_ordering(
                "c",
                "c.attributes",
                "",
                "ci.attributes",
                2,
                ordering,
                &mut values,
                &names,
            )
            .0
        } else {
            String::new()
        };
        let mut query = "select c.*, ci.attributes as item_attributes from collections c inner join collection_items ci on (ci.child_collection_id = c.id and ci.collection_id = $1 and c.deleted = false) ".to_owned();
        if ordering.is_empty() {
            query.push_str("order by name asc");
        } else {
            query.push_str(ordering.as_str());
        }
        query.push_str(
            format!(" offset ${} limit ${}", values.len() + 1, values.len() + 2).as_str(),
        );
        values.push(&offset as &(dyn ToSql + Sync));
        values.push(&limit as &(dyn ToSql + Sync));
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached(query.as_str()).await?;
        let rows = connection.query(&stmt, values.as_slice()).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, collection))]
    pub async fn get_child_collections_count(&self, collection: &Collection) -> Result<i64, Error> {
        let query = "select count(child_collection_id is not null) from collection_items ci inner join collections c on (ci.child_collection_id = c.id and c.deleted = false) where ci.collection_id = $1 ".to_owned();
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached(query.as_str()).await?;
        let rows = connection.query_one(&stmt, &[&collection.id]).await?;
        if rows.is_empty() {
            return Ok(0);
        }
        Ok(rows.get(0))
    }

    #[tracing::instrument(skip(self, collection, offset, limit))]
    pub async fn get_child_metadata(
        &self,
        collection: &Collection,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<Metadata>, Error> {
        let mut values = Vec::new();
        let mut names = Vec::new();
        values.push(&collection.id as &(dyn ToSql + Sync));
        let ordering = if let Some(ordering) = &collection.ordering {
            build_ordering_names(ordering, &mut names);
            build_ordering(
                "m",
                "",
                "m.attributes",
                "ci.attributes",
                2,
                ordering,
                &mut values,
                &names,
            )
            .0
        } else {
            String::new()
        };
        let mut query = "select m.*, ci.attributes as item_attributes from metadata m inner join collection_items ci on (ci.child_metadata_id = m.id and ci.collection_id = $1 and m.deleted = false) ".to_owned();
        if ordering.is_empty() {
            query.push_str("order by name asc");
        } else {
            query.push_str(ordering.as_str());
        }
        query.push_str(
            format!(" offset ${} limit ${}", values.len() + 1, values.len() + 2).as_str(),
        );
        values.push(&offset as &(dyn ToSql + Sync));
        values.push(&limit as &(dyn ToSql + Sync));
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached(query.as_str()).await?;
        let rows = connection.query(&stmt, values.as_slice()).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, collection))]
    pub async fn get_child_metadata_count(&self, collection: &Collection) -> Result<i64, Error> {
        let query = "select count(child_metadata_id is not null) from collection_items ci inner join metadata m on (ci.child_metadata_id = m.id and m.deleted = false) where ci.collection_id = $1".to_owned();
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached(query.as_str()).await?;
        let rows = connection.query_one(&stmt, &[&collection.id]).await?;
        if rows.is_empty() {
            return Ok(0);
        }
        Ok(rows.get(0))
    }

    #[tracing::instrument(skip(self, ctx, collection))]
    pub async fn add(
        &self,
        ctx: &BoscaContext,
        collection: &CollectionInput,
    ) -> Result<Uuid, Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let mut is_root = false;
        let parent_id = if let Some(id) = &collection.parent_collection_id {
            Uuid::parse_str(id)?
        } else {
            is_root = true;
            Uuid::parse_str("00000000-0000-0000-0000-000000000000")?
        };
        match self.add_txn(&txn, collection, true).await {
            Ok((id, slug)) => {
                txn.commit().await?;
                self.on_collection_changed(ctx, &id).await?;
                if !is_root {
                    self.on_collection_changed(ctx, &parent_id).await?;
                }
                if let Some(slug) = slug {
                    self.slug_cache.set_collection_slug(&id, &slug).await;
                }
                Ok(id)
            }
            Err(err) => {
                txn.rollback().await?;
                Err(err)
            }
        }
    }

    #[tracing::instrument(skip(self, ctx, id, collection))]
    pub async fn edit(
        &self,
        ctx: &BoscaContext,
        id: &Uuid,
        collection: &CollectionInput,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;

        match self.edit_txn(&txn, id, collection).await {
            Ok(slug) => {
                txn.commit().await?;
                self.on_collection_changed(ctx, id).await?;
                if let Some(slug) = slug {
                    self.slug_cache.set_collection_slug(id, &slug).await;
                }
                Ok(())
            }
            Err(err) => {
                txn.rollback().await?;
                Err(err)
            }
        }
    }

    #[tracing::instrument(skip(self, txn, collection, update_etag))]
    pub async fn add_txn<'a>(
        &'a self,
        txn: &'a Transaction<'a>,
        collection: &CollectionInput,
        update_etag: bool,
    ) -> Result<(Uuid, Option<String>), Error> {
        let stmt: Statement = if collection.collection_type.unwrap_or(CollectionType::Folder)
            == CollectionType::Root
        {
            txn.prepare("insert into collections (id, name, description, type, labels, attributes, ordering, template_metadata_id, template_metadata_version) values ('00000000-0000-0000-0000-000000000000', $1, $2, $3, $4, $5, $6, $7, $8) returning id").await?
        } else {
            txn.prepare("insert into collections (name, description, type, labels, attributes, ordering, template_metadata_id, template_metadata_version) values ($1, $2, $3, $4, $5, $6, $7, $8) returning id").await?
        };
        let labels = collection.labels.clone().unwrap_or_default();
        let ordering = collection
            .ordering
            .as_ref()
            .map(|ordering| serde_json::to_value(ordering).unwrap());
        let template_id = collection
            .template_metadata_id
            .as_ref()
            .map(|id| Uuid::parse_str(id).unwrap());
        let rows = txn
            .query(
                &stmt,
                &[
                    &collection.name,
                    &collection.description,
                    &collection.collection_type.unwrap_or(CollectionType::Folder),
                    &labels,
                    &collection.attributes.as_ref().or(Some(&Value::Null)),
                    &ordering,
                    &template_id,
                    &collection.template_metadata_version,
                ],
            )
            .await?;

        let id: Uuid = rows.first().unwrap().get(0);

        let stmt = txn.prepare_cached("insert into slugs (slug, collection_id) values (case when length($1) > 0 then $1 else slugify($2) end, $3) on conflict (slug) do update set slug = slugify($2) || nextval('duplicate_slug_seq')").await?;
        txn.execute(&stmt, &[&collection.slug, &collection.name, &id])
            .await?;

        if let Some(trait_ids) = &collection.trait_ids {
            for trait_id in trait_ids {
                self.add_trait_txn(txn, &id, trait_id).await?
            }
        }

        if let Some(category_ids) = &collection.category_ids {
            for category_id in category_ids {
                let cid = Uuid::parse_str(category_id)?;
                self.add_category_txn(txn, &id, &cid).await?
            }
        }

        if update_etag {
            update_collection_etag(txn, &id).await?;
        }

        Ok((id, collection.slug.clone()))
    }

    #[tracing::instrument(skip(self, txn, id))]
    #[allow(dead_code)]
    async fn delete_trait_txn<'a>(
        &'a self,
        txn: &'a Transaction<'a>,
        id: &Uuid,
    ) -> Result<(), Error> {
        let stmt = txn
            .prepare("delete from collection_traits where collection_id = $1")
            .await?;
        txn.execute(&stmt, &[id]).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, txn, id, trait_id))]
    async fn add_trait_txn<'a>(
        &'a self,
        txn: &'a Transaction<'a>,
        id: &Uuid,
        trait_id: &String,
    ) -> Result<(), Error> {
        let stmt = txn
            .prepare("insert into collection_traits (collection_id, trait_id) values ($1, $2)")
            .await?;
        txn.execute(&stmt, &[id, trait_id]).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, txn, id))]
    async fn delete_categories_txn<'a>(
        &'a self,
        txn: &'a Transaction<'a>,
        id: &Uuid,
    ) -> Result<(), Error> {
        let stmt = txn
            .prepare("delete from collection_categories where collection_id = $1")
            .await?;
        txn.execute(&stmt, &[id]).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, txn, id, category_id))]
    async fn add_category_txn<'a>(
        &'a self,
        txn: &'a Transaction<'a>,
        id: &Uuid,
        category_id: &Uuid,
    ) -> Result<(), Error> {
        let stmt = txn
            .prepare(
                "insert into collection_categories (collection_id, category_id) values ($1, $2)",
            )
            .await?;
        txn.execute(&stmt, &[id, category_id]).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id, template_id, template_version))]
    pub async fn set_template(
        &self,
        ctx: &BoscaContext,
        id: &Uuid,
        template_id: &Uuid,
        template_version: i32,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare("update collections set modified = now(), template_metadata_id = $1, template_metadata_version = $2 where id = $3").await?;
        txn.execute(&stmt, &[template_id, &template_version, id]).await?;
        txn.commit().await?;
        self.update_storage(ctx, id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, txn, id, collection))]
    async fn edit_txn<'a>(
        &'a self,
        txn: &'a Transaction<'a>,
        id: &Uuid,
        collection: &CollectionInput,
    ) -> Result<Option<String>, Error> {
        let stmt = txn.prepare("update collections set name = $1, description = $2, type = $3, labels = $4, attributes = $5, ordering = $6, modified = now(), template_metadata_id = $7, template_metadata_version = $8 where id = $9").await?;
        let labels = collection.labels.clone().unwrap_or_default();
        let ordering = collection
            .ordering
            .as_ref()
            .map(|ordering| serde_json::to_value(ordering).unwrap());
        let template_id = collection
            .template_metadata_id
            .as_ref()
            .map(|id| Uuid::parse_str(id).unwrap());
        txn.execute(
            &stmt,
            &[
                &collection.name,
                &collection.description,
                &collection.collection_type.unwrap_or(CollectionType::Folder),
                &labels,
                &collection.attributes.as_ref().or(Some(&Value::Null)),
                &ordering,
                &template_id,
                &collection.template_metadata_version,
                id,
            ],
        )
        .await?;

        if let Some(slug) = collection.slug.as_ref() {
            let stmt = txn
                .prepare_cached("delete from slugs where collection_id = $1")
                .await?;
            txn.execute(&stmt, &[id]).await?;
            let stmt = txn.prepare_cached("insert into slugs (slug, collection_id) values (case when length($1) > 0 then $1 else slugify($2) end, $3) on conflict (slug) do update set slug = slugify($2) || nextval('duplicate_slug_seq')").await?;
            txn.execute(&stmt, &[slug, &collection.name, id]).await?;
        }

        if let Some(trait_ids) = &collection.trait_ids {
            self.delete_trait_txn(txn, id).await?;
            for trait_id in trait_ids {
                self.add_trait_txn(txn, id, trait_id).await?
            }
        }

        if let Some(category_ids) = &collection.category_ids {
            self.delete_categories_txn(txn, id).await?;
            for category_id in category_ids {
                let cid = Uuid::parse_str(category_id)?;
                self.add_category_txn(txn, id, &cid).await?
            }
        }

        update_collection_etag(txn, id).await?;

        Ok(collection.slug.clone())
    }

    #[tracing::instrument(skip(self, ctx, id, collection_id, attributes))]
    pub async fn add_child_collection(
        &self,
        ctx: &BoscaContext,
        id: &Uuid,
        collection_id: &Uuid,
        attributes: &Option<Value>,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        self.add_child_collection_txn(&txn, id, collection_id, attributes)
            .await?;
        txn.commit().await?;
        self.on_collection_changed(ctx, id).await?;
        self.on_collection_changed(ctx, collection_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, txn, id, collection_id, attributes))]
    pub async fn add_child_collection_txn(
        &self,
        txn: &Transaction<'_>,
        id: &Uuid,
        collection_id: &Uuid,
        attributes: &Option<Value>,
    ) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into collection_items (collection_id, child_collection_id, attributes) values ($1, $2, $3) on conflict(collection_id, child_collection_id) do nothing").await?;
        txn.execute(&stmt, &[id, collection_id, attributes]).await?;
        update_collection_etag(txn, id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id, metadata_id, attributes))]
    pub async fn add_child_metadata(
        &self,
        ctx: &BoscaContext,
        id: &Uuid,
        metadata_id: &Uuid,
        attributes: &Option<Value>,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        self.add_child_metadata_txn(&txn, id, metadata_id, attributes)
            .await?;
        txn.commit().await?;
        self.on_collection_changed(ctx, id).await?;
        self.on_metadata_changed(ctx, metadata_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, txn, id, metadata_id, attributes))]
    pub async fn add_child_metadata_txn(
        &self,
        txn: &Transaction<'_>,
        id: &Uuid,
        metadata_id: &Uuid,
        attributes: &Option<Value>,
    ) -> Result<(), Error> {
        let stmt = txn
            .prepare_cached(
                "insert into collection_items (collection_id, child_metadata_id, attributes) values ($1, $2, $3) on conflict(collection_id, child_metadata_id) do nothing",
            )
            .await?;
        txn.execute(&stmt, &[id, metadata_id, attributes]).await?;
        update_collection_etag(txn, id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id, collection_id))]
    pub async fn remove_child_collection(
        &self,
        ctx: &BoscaContext,
        id: &Uuid,
        collection_id: &Uuid,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached(
                "delete from collection_items where collection_id = $1 and child_collection_id = $2",
            )
            .await?;
        txn.execute(&stmt, &[id, collection_id]).await?;
        update_collection_etag(&txn, id).await?;
        txn.commit().await?;
        self.on_collection_changed(ctx, collection_id).await?;
        self.on_collection_changed(ctx, id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id, metadata_id))]
    pub async fn remove_child_metadata(
        &self,
        ctx: &BoscaContext,
        id: &Uuid,
        metadata_id: &Uuid,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached(
                "delete from collection_items where collection_id = $1 and child_metadata_id = $2",
            )
            .await?;
        txn.execute(&stmt, &[id, metadata_id]).await?;
        update_collection_etag(&txn, id).await?;
        txn.commit().await?;
        self.on_collection_changed(ctx, id).await?;
        self.on_metadata_changed(ctx, metadata_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, collection_id, attributes))]
    pub async fn set_attributes(
        &self,
        ctx: &BoscaContext,
        collection_id: &Uuid,
        attributes: Value,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached(
                "update collections set attributes = $1, modified = now() where id = $2",
            )
            .await?;
        txn.execute(&stmt, &[&attributes, &collection_id]).await?;
        update_collection_etag(&txn, collection_id).await?;
        txn.commit().await?;
        self.on_collection_changed(ctx, collection_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, collection_id, attributes))]
    pub async fn set_system_attributes(
        &self,
        ctx: &BoscaContext,
        collection_id: &Uuid,
        attributes: Value,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached(
                "update collections set system_attributes = $1, modified = now() where id = $2",
            )
            .await?;
        txn.execute(&stmt, &[&attributes, &collection_id]).await?;
        update_collection_etag(&txn, collection_id).await?;
        txn.commit().await?;
        self.on_collection_changed(ctx, collection_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, collection_id, attributes))]
    pub async fn merge_attributes(
        &self,
        ctx: &BoscaContext,
        collection_id: &Uuid,
        attributes: Value,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached("update collections set attributes = coalesce(attributes, '{}'::jsonb) || $1, modified = now() where id = $2")
            .await?;
        txn.execute(&stmt, &[&attributes, &collection_id]).await?;
        update_collection_etag(&txn, collection_id).await?;
        txn.commit().await?;
        self.on_collection_changed(ctx, collection_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, collection_id, attributes))]
    pub async fn merge_collection_item_attributes(
        &self,
        ctx: &BoscaContext,
        collection_id: &Uuid,
        item_id: &Uuid,
        attributes: Value,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached("update collection_items set attributes = coalesce(attributes, '{}'::jsonb) || $1 where collection_id = $2 and child_collection_id = $3")
            .await?;
        txn.execute(&stmt, &[&attributes, collection_id, item_id])
            .await?;
        update_collection_etag(&txn, collection_id).await?;
        update_collection_etag(&txn, item_id).await?;
        txn.commit().await?;
        self.on_collection_changed(ctx, collection_id).await?;
        self.on_collection_changed(ctx, item_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, collection_id, attributes))]
    pub async fn merge_metadata_item_attributes(
        &self,
        ctx: &BoscaContext,
        collection_id: &Uuid,
        item_id: &Uuid,
        attributes: Value,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached("update collection_items set attributes = coalesce(attributes, '{}'::jsonb) || $1 where collection_id = $2 and child_metadata_id = $3")
            .await?;
        txn.execute(&stmt, &[&attributes, collection_id, item_id])
            .await?;
        update_collection_etag(&txn, collection_id).await?;
        update_metadata_etag(&txn, item_id).await?;
        txn.commit().await?;
        self.on_collection_changed(ctx, collection_id).await?;
        self.on_metadata_changed(ctx, item_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, collection_id, metadata_id, relationship, attributes))]
    pub async fn merge_relationship_attributes(
        &self,
        ctx: &BoscaContext,
        collection_id: &Uuid,
        metadata_id: &Uuid,
        relationship: &str,
        attributes: Value,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached("update collection_metadata_relationships set attributes = coalesce(attributes, '{}'::jsonb) || $1 where collection_id = $2 and metadata_id = $3 and relationship = $4")
            .await?;
        let relationship = relationship.to_owned();
        txn.execute(
            &stmt,
            &[&attributes, &collection_id, &metadata_id, &relationship],
        )
        .await?;
        update_collection_etag(&txn, collection_id).await?;
        update_metadata_etag(&txn, metadata_id).await?;
        txn.commit().await?;
        self.on_collection_changed(ctx, collection_id).await?;
        self.on_metadata_changed(ctx, metadata_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, collection_id, ordering))]
    pub async fn set_ordering(
        &self,
        ctx: &BoscaContext,
        collection_id: &Uuid,
        ordering: Value,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached("update collections set ordering = $1, modified = now() where id = $2")
            .await?;
        txn.execute(&stmt, &[&ordering, &collection_id]).await?;
        update_collection_etag(&txn, collection_id).await?;
        txn.commit().await?;
        self.on_collection_changed(ctx, collection_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, collections))]
    pub async fn add_all(
        &self,
        ctx: &BoscaContext,
        collections: &mut [CollectionChildInput],
    ) -> Result<Vec<Uuid>, Error> {
        let ids = {
            let mut conn = self.pool.get().await?;
            let txn = conn.transaction().await?;
            let ids = self
                .add_all_txn(ctx, &txn, collections, false, None)
                .await?;
            txn.commit().await?;
            ids
        };
        let mut indexed = HashSet::new();
        for (id1, id2, metadata_ids) in &ids {
            for (metadata_id, _, _) in metadata_ids {
                self.on_metadata_changed(ctx, metadata_id).await?;
            }
            if !indexed.contains(id1) {
                self.on_collection_changed(ctx, id1).await?;
            }
            if !indexed.contains(id2) {
                self.on_collection_changed(ctx, id2).await?;
            }
            indexed.insert(id1);
            indexed.insert(id2);
        }
        Ok(ids.into_iter().map(|(id, _, _)| id).collect())
    }

    #[tracing::instrument(skip(self, ctx, txn, collections, ignore_permission_check, permissions))]
    #[allow(clippy::type_complexity)]
    #[allow(clippy::too_many_arguments)]
    async fn add_all_txn(
        &self,
        ctx: &BoscaContext,
        txn: &Transaction<'_>,
        collections: &mut [CollectionChildInput],
        ignore_permission_check: bool,
        permissions: Option<Vec<Permission>>,
    ) -> Result<Vec<(Uuid, Uuid, Vec<(Uuid, i32, i32)>)>, Error> {
        let mut new_collections = Vec::new();
        for collection_child in collections.iter_mut() {
            let collection = &mut collection_child.collection;
            let has_collection_id = collection.parent_collection_id.is_some();
            let parent_collection_id = match &collection.parent_collection_id {
                Some(id) => Uuid::parse_str(id.as_str())?,
                None => Uuid::parse_str("00000000-0000-0000-0000-000000000000")?,
            };
            if !ignore_permission_check {
                let c = ctx
                    .check_collection_action_txn(txn, &parent_collection_id, PermissionAction::Edit)
                    .await?;
                if c.items_locked && !ctx.has_service_account().await? {
                    return Err(Error::new("locked"));
                }
            }
            let (id, _) = self.add_txn(txn, collection, false).await?;
            let permissions = if let Some(permissions) = &permissions {
                permissions.clone()
            } else {
                ctx.content
                    .collection_permissions
                    .get_txn(txn, &parent_collection_id)
                    .await?
            };
            for permission in permissions.iter() {
                let collection_permission = Permission {
                    entity_id: id,
                    group_id: permission.group_id,
                    action: permission.action,
                };
                ctx.content
                    .collection_permissions
                    .add_txn(txn, &collection_permission)
                    .await?
            }
            if has_collection_id {
                self.add_child_collection_txn(
                    txn,
                    &parent_collection_id,
                    &id,
                    &collection_child.attributes,
                )
                .await?;
            }
            if let Some(children) = &mut collection.collections {
                for child in children.iter_mut() {
                    child.collection.parent_collection_id = Some(id.to_string());
                }
                let collections =
                    Box::pin(self.add_all_txn(ctx, txn, children, true, Some(permissions.clone())))
                        .await?;
                new_collections.extend(collections);
            }
            let mut metadata_ids = Vec::new();
            if let Some(children) = &mut collection.metadata {
                for child in children.iter_mut() {
                    child.metadata.parent_collection_id = Some(id.to_string());
                }
                let ids = ctx
                    .content
                    .metadata
                    .add_all_txn(ctx, txn, children, false)
                    .await?;
                for (metadata_id, _, _) in ids.iter() {
                    ctx.content
                        .metadata_permissions
                        .add_metadata_permissions_txn(txn, metadata_id, &permissions)
                        .await?;
                }
                metadata_ids.extend(ids);
            }
            update_collection_etag(txn, &id).await?;
            new_collections.push((id, parent_collection_id, metadata_ids));
        }
        Ok(new_collections)
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_metadata_relationships(
        &self,
        id: &Uuid,
    ) -> Result<Vec<CollectionMetadataRelationship>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare("select * from collection_metadata_relationships where collection_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[&id]).await?;
        Ok(rows
            .iter()
            .map(CollectionMetadataRelationship::from)
            .collect())
    }

    #[tracing::instrument(skip(self, id, metadata_id))]
    pub async fn get_metadata_relationship(
        &self,
        id: &Uuid,
        metadata_id: &Uuid,
    ) -> Result<Option<CollectionMetadataRelationship>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare("select * from collection_metadata_relationships where collection_id = $1 and metadata_id = $2")
            .await?;
        let rows = connection.query(&stmt, &[id, metadata_id]).await?;
        Ok(rows.first().map(CollectionMetadataRelationship::from))
    }

    #[tracing::instrument(skip(self, ctx, relationship))]
    pub async fn add_metadata_relationship(
        &self,
        ctx: &BoscaContext,
        relationship: &CollectionMetadataRelationshipInput,
    ) -> Result<(), Error> {
        let id = Uuid::parse_str(relationship.id.as_str())?;
        let metadata_id = Uuid::parse_str(relationship.metadata_id.as_str())?;
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("insert into collection_metadata_relationships (collection_id, metadata_id, relationship, attributes) values ($1, $2, $3, $4)").await?;
        connection
            .execute(
                &stmt,
                &[
                    &id,
                    &metadata_id,
                    &relationship.relationship,
                    &relationship.attributes,
                ],
            )
            .await?;
        self.on_collection_changed(ctx, &id).await?;
        self.on_metadata_changed(ctx, &metadata_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, relationship))]
    pub async fn edit_metadata_relationship(
        &self,
        ctx: &BoscaContext,
        relationship: &CollectionMetadataRelationshipInput,
    ) -> Result<(), Error> {
        let id = Uuid::parse_str(relationship.id.as_str())?;
        let metadata_id = Uuid::parse_str(relationship.metadata_id.as_str())?;
        let relationship = relationship.to_owned();
        let connection = self.pool.get().await?;
        let stmt = connection.prepare("update metadata_relationships set relationship = $1, attributes = $2 where collection_id = $3 and metadata_id = $4 and (relationship = $1 or relationship is null or relationship = '')").await?;
        connection
            .query(
                &stmt,
                &[
                    &relationship.relationship,
                    &relationship.attributes,
                    &id,
                    &metadata_id,
                ],
            )
            .await?;
        self.on_collection_changed(ctx, &id).await?;
        self.on_metadata_changed(ctx, &metadata_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id, metadata_id, relationship))]
    pub async fn delete_metadata_relationship(
        &self,
        ctx: &BoscaContext,
        id: &Uuid,
        metadata_id: &Uuid,
        relationship: &str,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let relationship = relationship.to_owned();
        let stmt = connection
            .prepare_cached(
                "delete from collection_metadata_relationships where collection_id = $1 and metadata_id = $2 and relationship = $3",
            )
            .await?;
        connection
            .execute(&stmt, &[id, metadata_id, &relationship])
            .await?;
        self.on_collection_changed(ctx, id).await?;
        self.on_metadata_changed(ctx, metadata_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id))]
    pub async fn update_storage(&self, ctx: &BoscaContext, id: &Uuid) -> Result<(), Error> {
        self.cache.evict_collection(id).await;
        let mut request = EnqueueRequest {
            workflow_id: Some(COLLECTION_UPDATE_STORAGE.to_string()),
            collection_id: Some(*id),
            ..Default::default()
        };
        ctx.workflow.enqueue_workflow(ctx, &mut request).await?;
        Ok(())
    }
}
