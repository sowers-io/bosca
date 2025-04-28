use crate::context::BoscaContext;
use crate::datastores::content::tag::update_metadata_etag;
use crate::datastores::content::util::build_find_args;
use crate::datastores::metadata_cache::MetadataCache;
use crate::datastores::notifier::Notifier;
use crate::models::content::category::Category;
use crate::models::content::collection::MetadataChildInput;
use crate::models::content::find_query::FindQueryInput;
use crate::models::content::metadata::{Metadata, MetadataInput};
use crate::models::content::metadata_profile::MetadataProfile;
use crate::models::content::metadata_relationship::{
    MetadataRelationship, MetadataRelationshipInput,
};
use crate::models::workflow::enqueue_request::EnqueueRequest;
use crate::workflow::core_workflow_ids::{METADATA_DELETE_FINALIZE, METADATA_UPDATE_STORAGE};
use async_graphql::*;
use bosca_database::TracingPool;
use bosca_dc_client::client::Client;
use chrono::{TimeDelta, Utc};
use deadpool_postgres::Transaction;
use log::error;
use serde_json::{Map, Value};
use std::ops::Add;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

#[derive(Clone)]
pub struct MetadataDataStore {
    cache: MetadataCache,
    pool: TracingPool,
    client: Client,
    notifier: Arc<Notifier>,
    update_cache: String,
}

impl MetadataDataStore {
    pub async fn new(
        pool: TracingPool,
        cache: MetadataCache,
        notifier: Arc<Notifier>,
        client: Client,
    ) -> Result<Self, Error> {
        let prefix = option_env!("NAMESPACE").unwrap_or("").to_string();
        let bucket = format!("{}_metadata_update_storage_queue", prefix);
        client.create(&bucket, 5000, 5, 0).await?;
        Ok(Self {
            cache,
            pool,
            notifier,
            client,
            update_cache: bucket,
        })
    }

    pub fn watch(&self, ctx: &BoscaContext) {
        let watcher_ctx = ctx.clone();
        let client = self.client.clone();
        let update_cache = self.update_cache.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = Self::enqueue_items(&client, &watcher_ctx, &update_cache).await {
                    error!("Failed to enqueue metadata updates: {:?}", e);
                }
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        });
    }

    async fn enqueue_items(
        client: &Client,
        ctx: &BoscaContext,
        update_cache: &str,
    ) -> Result<(), Error> {
        let mut subscription = client.subscribe();
        while let Ok(notification) = subscription.recv().await {
            if notification.cache == update_cache {
                let id = notification.key.unwrap();
                let id = Uuid::parse_str(&id)?;
                let mut request = EnqueueRequest {
                    workflow_id: Some(METADATA_UPDATE_STORAGE.to_string()),
                    metadata_id: Some(id),
                    ..Default::default()
                };
                ctx.workflow.enqueue_workflow(ctx, &mut request).await?;
            }
        }
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id))]
    pub async fn on_metadata_changed(&self, ctx: &BoscaContext, id: &Uuid) -> Result<(), Error> {
        self.update_storage(ctx, id).await?;
        if let Err(e) = self.notifier.metadata_changed(id).await {
            error!("Failed to notify metadata changes: {:?}", e);
        }
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id))]
    async fn on_collection_changed(&self, ctx: &BoscaContext, id: &Uuid) -> Result<(), Error> {
        ctx.content
            .collections
            .on_collection_changed(ctx, id)
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_slug(&self, id: &Uuid) -> Result<Option<String>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select slug from slugs where metadata_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        Ok(rows.first().unwrap().get("slug"))
    }

    #[tracing::instrument(skip(self, query))]
    pub async fn find(&self, query: &mut FindQueryInput) -> Result<Vec<Metadata>, Error> {
        let category_ids = query.get_category_ids();
        let mut names = Vec::new();
        let (query, values) = build_find_args(
            "metadata",
            "select m.* from metadata m ",
            "m",
            "attributes",
            "attributes",
            query,
            &category_ids,
            false,
            &mut names,
        );
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached(query.as_str()).await?;
        let rows = connection.query(&stmt, values.as_slice()).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, query))]
    pub async fn find_count(&self, query: &mut FindQueryInput) -> Result<i64, Error> {
        let category_ids = query.get_category_ids();
        let mut names = Vec::new();
        let (query, values) = build_find_args(
            "metadata",
            "select count(*) as count from metadata m ",
            "m",
            "attributes",
            "attributes",
            query,
            &category_ids,
            true,
            &mut names,
        );
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached(query.as_str()).await?;
        let rows = connection.query(&stmt, values.as_slice()).await?;
        if rows.is_empty() {
            Ok(0)
        } else {
            Ok(rows.first().unwrap().get("count"))
        }
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get(&self, id: &Uuid) -> Result<Option<Metadata>, Error> {
        if let Some(metadata) = self.cache.get_metadata(id).await {
            return Ok(Some(metadata));
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from metadata where id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        let metadata = rows.first().unwrap().into();
        self.cache.set_metadata(&metadata).await;
        Ok(Some(metadata))
    }

    #[tracing::instrument(skip(self, id, version))]
    pub async fn get_by_version(&self, id: &Uuid, version: i32) -> Result<Option<Metadata>, Error> {
        if let Some(metadata) = self.cache.get_metadata_by_version(id, version).await {
            return Ok(Some(metadata));
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from metadata_versions where id = $1 and version = $2")
            .await?;
        let rows = connection.query(&stmt, &[id, &version]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        let metadata = rows.first().unwrap().into();
        self.cache.set_metadata(&metadata).await;
        Ok(Some(metadata))
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_categories(&self, id: &Uuid) -> Result<Vec<Category>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select c.* from metadata_categories mc inner join categories c on (mc.category_id = c.id) where metadata_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, id, offset, limit))]
    pub async fn get_parent_ids(
        &self,
        id: &Uuid,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<Uuid>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select ci.collection_id from collection_items ci inner join collections c on (ci.collection_id = c.id and c.deleted = false) where ci.child_metadata_id = $1 offset $2 limit $3")
            .await?;
        let rows = connection.query(&stmt, &[id, &offset, &limit]).await?;
        Ok(rows.iter().map(|r| r.get("collection_id")).collect())
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_trait_ids(&self, id: &Uuid) -> Result<Vec<String>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select trait_id from metadata_traits where metadata_id = $1")
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
    pub async fn get_profiles(&self, id: &Uuid) -> Result<Vec<MetadataProfile>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "select * from metadata_profiles where metadata_id = $1 order by sort asc",
            )
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, ctx, metadata_id))]
    pub async fn mark_deleted(&self, ctx: &BoscaContext, metadata_id: &Uuid) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("update metadata set deleted = true, modified = now() where id = $1 returning version")
            .await?;
        let row = connection.query_one(&stmt, &[metadata_id]).await?;
        let mut request = EnqueueRequest {
            workflow_id: Some(METADATA_DELETE_FINALIZE.to_string()),
            metadata_id: Some(*metadata_id),
            metadata_version: Some(row.get("version")),
            ..Default::default()
        };
        ctx.workflow.enqueue_workflow(ctx, &mut request).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, metadata_id))]
    pub async fn delete(&self, ctx: &BoscaContext, metadata_id: &Uuid) -> Result<(), Error> {
        let Some(metadata) = self.get(metadata_id).await? else {
            return Ok(());
        };
        let supplementaries = ctx
            .content
            .metadata_supplementary
            .get_supplementaries(metadata_id)
            .await?;
        for supplementary in supplementaries {
            let path = ctx
                .storage
                .get_metadata_path(&metadata, Some(supplementary.id))
                .await?;
            ctx.storage.delete(&path).await?;
        }
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached(
                "select collection_id from collection_items where child_metadata_id = $1",
            )
            .await?;
        let rows = txn.query(&stmt, &[metadata_id]).await?;
        let collection_ids: Vec<Uuid> = rows.iter().map(|r| r.get("collection_id")).collect();
        let stmt = txn
            .prepare_cached("delete from metadata_versions where id = $1")
            .await?;
        txn.execute(&stmt, &[&metadata_id]).await?;
        let stmt = txn
            .prepare_cached("delete from metadata where id = $1")
            .await?;
        txn.execute(&stmt, &[&metadata_id]).await?;
        txn.commit().await?;

        self.cache.evict_metadata(metadata_id).await;
        self.on_metadata_changed(ctx, metadata_id).await?;
        for collection_id in collection_ids {
            self.on_collection_changed(ctx, &collection_id).await?;
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
            .prepare_cached("update metadata set public = $1, modified = now() where id = $2")
            .await?;
        txn.execute(&stmt, &[&public, id]).await?;
        update_metadata_etag(&txn, id).await?;
        txn.commit().await?;
        self.on_metadata_changed(ctx, id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id, public))]
    pub async fn set_public_content(
        &self,
        ctx: &BoscaContext,
        id: &Uuid,
        public: bool,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached(
                "update metadata set public_content = $1, modified = now() where id = $2",
            )
            .await?;
        txn.execute(&stmt, &[&public, id]).await?;
        update_metadata_etag(&txn, id).await?;
        txn.commit().await?;
        self.cache.evict_metadata(id).await;
        self.on_metadata_changed(ctx, id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id, metadata))]
    pub async fn edit(
        &self,
        ctx: &BoscaContext,
        id: &Uuid,
        metadata: &MetadataInput,
    ) -> Result<(), Error> {
        let mut source_id: Option<Uuid> = None;
        let mut source_identifier: Option<String> = None;
        let mut source_url: Option<String> = None;
        if let Some(source) = &metadata.source {
            source_id = source.id.as_ref().map(|id| Uuid::parse_str(id).unwrap());
            source_identifier = source.identifier.clone();
            source_url = source.source_url.clone();
        }
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;

        let stmt = txn
            .prepare_cached(
                "select collection_id from collection_items where child_metadata_id = $1",
            )
            .await?;
        let rows = txn.query(&stmt, &[id]).await?;
        let collection_ids: Vec<Uuid> = rows.iter().map(|r| r.get("collection_id")).collect();

        match self
            .edit_txn(
                ctx,
                &txn,
                id,
                metadata,
                &source_id,
                &source_identifier,
                &source_url,
            )
            .await
        {
            Ok(_) => {
                txn.commit().await?;
                self.cache.evict_metadata(id).await;
                self.on_metadata_changed(ctx, id).await?;
                for collection_id in collection_ids {
                    self.on_collection_changed(ctx, &collection_id).await?;
                }
                Ok(())
            }
            Err(err) => {
                txn.rollback().await?;
                Err(err)
            }
        }
    }

    #[tracing::instrument(skip(self, ctx, metadata_id, attributes))]
    pub async fn set_attributes(
        &self,
        ctx: &BoscaContext,
        metadata_id: &Uuid,
        attributes: Value,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached("update metadata set attributes = $1, modified = now() where id = $2")
            .await?;
        txn.execute(&stmt, &[&attributes, &metadata_id]).await?;
        update_metadata_etag(&txn, metadata_id).await?;
        txn.commit().await?;
        self.on_metadata_changed(ctx, metadata_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, metadata_id, attributes))]
    pub async fn merge_attributes(
        &self,
        ctx: &BoscaContext,
        metadata_id: &Uuid,
        attributes: Value,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached("update metadata set attributes = coalesce(attributes, '{}'::jsonb) || $1, modified = now() where id = $2")
            .await?;
        txn.execute(&stmt, &[&attributes, &metadata_id]).await?;
        update_metadata_etag(&txn, metadata_id).await?;
        txn.commit().await?;
        self.on_metadata_changed(ctx, metadata_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, metadata1_id, metadata2_id, relationship, attributes))]
    pub async fn merge_relationship_attributes(
        &self,
        ctx: &BoscaContext,
        metadata1_id: &Uuid,
        metadata2_id: &Uuid,
        relationship: &str,
        attributes: Value,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached("update metadata_relationships set attributes = coalesce(attributes, '{}'::jsonb) || $1 where metadata1_id = $2 and metadata2_id = $3 and relationship = $4")
            .await?;
        let relationship = relationship.to_owned();
        txn.execute(
            &stmt,
            &[&attributes, &metadata1_id, &metadata2_id, &relationship],
        )
        .await?;
        update_metadata_etag(&txn, metadata1_id).await?;
        update_metadata_etag(&txn, metadata2_id).await?;
        txn.commit().await?;
        self.on_metadata_changed(ctx, metadata1_id).await?;
        self.on_metadata_changed(ctx, metadata2_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, metadata_id, attributes))]
    pub async fn set_system_attributes(
        &self,
        ctx: &BoscaContext,
        metadata_id: &Uuid,
        attributes: Value,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached(
                "update metadata set system_attributes = $1, modified = now() where id = $2",
            )
            .await?;
        txn.execute(&stmt, &[&attributes, &metadata_id]).await?;
        update_metadata_etag(&txn, metadata_id).await?;
        txn.commit().await?;
        self.on_metadata_changed(ctx, metadata_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, metadata_id, original_file_name, content_type, len))]
    pub async fn set_uploaded(
        &self,
        ctx: &BoscaContext,
        metadata_id: &Uuid,
        original_file_name: &Option<String>,
        content_type: &Option<String>,
        len: usize,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached("update metadata set uploaded = now(), system_attributes = $1, modified = now(), content_type = $2, content_length = $3 where id = $4")
            .await?;
        let len = len as i64;
        let mut attrs = Map::new();
        attrs.insert(
            "original_file_name".to_owned(),
            Value::String(original_file_name.clone().unwrap_or("--".to_owned())),
        );
        let attrs = Value::Object(attrs);
        txn.execute(&stmt, &[&attrs, content_type, &len, metadata_id])
            .await?;
        if let Some(content_type) = content_type {
            self.ensure_content_type_traits(metadata_id, content_type, &txn)
                .await?;
        }
        update_metadata_etag(&txn, metadata_id).await?;
        txn.commit().await?;
        self.on_metadata_changed(ctx, metadata_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, metadata_id, content_type, txn))]
    async fn ensure_content_type_traits(
        &self,
        metadata_id: &Uuid,
        content_type: &str,
        txn: &Transaction<'_>,
    ) -> Result<(), Error> {
        let current_traits = self.get_trait_ids(metadata_id).await?;
        let stmt = txn
            .prepare_cached("select trait_id from trait_content_types where content_type = $1")
            .await?;
        let content_type = content_type.to_owned();
        let result = txn.query(&stmt, &[&content_type]).await?;
        for row in result {
            let content_type = row.get(0);
            if current_traits.contains(&content_type) {
                continue;
            }
            self.add_trait_txn(txn, metadata_id, &content_type).await?;
        }
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, metadata_id))]
    pub async fn set_upload_removed(
        &self,
        ctx: &BoscaContext,
        metadata_id: &Uuid,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("update metadata set uploaded = null, modified = now(), content_length = 0 where id = $1")
            .await?;
        connection.execute(&stmt, &[&metadata_id]).await?;
        self.cache.evict_metadata(metadata_id).await;
        self.on_metadata_changed(ctx, metadata_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(
        self,
        ctx,
        txn,
        id,
        metadata,
        source_id,
        source_identifier,
        source_url
    ))]
    #[allow(clippy::too_many_arguments)]
    async fn edit_txn<'a>(
        &'a self,
        ctx: &BoscaContext,
        txn: &'a Transaction<'a>,
        id: &Uuid,
        metadata: &MetadataInput,
        source_id: &Option<Uuid>,
        source_identifier: &Option<String>,
        source_url: &Option<String>,
    ) -> Result<i32, Error> {
        let stmt = txn.prepare("update metadata set name = $1, labels = $2, attributes = $3, language_tag = $4, source_id = $5, source_identifier = $6, source_url = $7, content_type = $8, modified = now() where id = $9 returning version").await?;
        let labels = metadata.labels.clone().unwrap_or_default();
        let result = txn
            .query_one(
                &stmt,
                &[
                    &metadata.name,
                    &labels,
                    &metadata.attributes.as_ref().or(Some(&Value::Null)),
                    &metadata.language_tag,
                    source_id,
                    source_identifier,
                    source_url,
                    &metadata.content_type,
                    &id,
                ],
            )
            .await?;
        let version: i32 = result.get(0);

        let stmt = txn
            .prepare_cached("delete from slugs where metadata_id = $1")
            .await?;
        txn.execute(&stmt, &[id]).await?;
        let stmt = txn.prepare_cached("insert into slugs (slug, metadata_id) values (case when length($1) > 0 then $1 else slugify($2) end, $3) on conflict (slug) do update set slug = slugify($2) || nextval('duplicate_slug_seq')").await?;
        txn.execute(&stmt, &[&metadata.slug, &metadata.name, id])
            .await?;

        if let Some(trait_ids) = &metadata.trait_ids {
            self.delete_traits_txn(txn, id).await?;
            for trait_id in trait_ids {
                self.add_trait_txn(txn, id, trait_id).await?
            }
        }

        if let Some(category_ids) = &metadata.category_ids {
            self.delete_categories_txn(txn, id).await?;
            for category_id in category_ids {
                let cid = Uuid::parse_str(category_id)?;
                self.add_category_txn(txn, id, &cid).await?
            }
        }

        if let Some(profiles) = &metadata.profiles {
            self.delete_profiles_txn(txn, id).await?;
            for (index, profile) in profiles.iter().enumerate() {
                let pid = Uuid::parse_str(&profile.profile_id)?;
                self.add_profile_txn(txn, id, &pid, &profile.relationship, index as i32)
                    .await?
            }
        }

        if let Some(document) = &metadata.document {
            ctx.content
                .documents
                .edit_document_txn(txn, id, version, document)
                .await?;
        }
        if let Some(document_template) = &metadata.document_template {
            ctx.content
                .documents
                .edit_template_txn(txn, id, version, document_template)
                .await?;
        }
        if let Some(guide) = &metadata.guide {
            ctx.content
                .guides
                .edit_guide_txn(ctx, txn, id, version, guide)
                .await?;
        }
        if let Some(guide_template) = &metadata.guide_template {
            ctx.content
                .guides
                .edit_template_txn(txn, id, version, guide_template)
                .await?;
        }
        if let Some(collection_template) = &metadata.collection_template {
            ctx.content
                .collection_templates
                .edit_template_txn(txn, id, version, collection_template)
                .await?;
        }

        self.ensure_content_type_traits(id, &metadata.content_type, txn)
            .await?;

        update_metadata_etag(txn, id).await?;

        Ok(version)
    }

    #[tracing::instrument(skip(self, txn, id, profile_id, relationship, sort))]
    async fn add_profile_txn<'a>(
        &'a self,
        txn: &'a Transaction<'a>,
        id: &Uuid,
        profile_id: &Uuid,
        relationship: &String,
        sort: i32,
    ) -> Result<(), Error> {
        let stmt = txn
            .prepare("insert into metadata_profiles (metadata_id, profile_id, relationship, sort) values ($1, $2, $3, $4)")
            .await?;
        txn.execute(&stmt, &[id, profile_id, relationship, &sort])
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, txn, id))]
    async fn delete_profiles_txn<'a>(
        &'a self,
        txn: &'a Transaction<'a>,
        id: &Uuid,
    ) -> Result<(), Error> {
        let stmt = txn
            .prepare("delete from metadata_profiles where metadata_id = $1")
            .await?;
        txn.execute(&stmt, &[id]).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id, trait_id))]
    pub async fn delete_trait(
        &self,
        ctx: &BoscaContext,
        id: &Uuid,
        trait_id: &String,
    ) -> Result<(), Error> {
        let conn = self.pool.get().await?;
        let stmt = conn
            .prepare("delete from metadata_traits where metadata_id = $1 and trait_id = $2")
            .await?;
        conn.execute(&stmt, &[id, trait_id]).await?;
        self.cache.evict_metadata(id).await;
        self.on_metadata_changed(ctx, id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, txn, id))]
    async fn delete_traits_txn<'a>(
        &'a self,
        txn: &'a Transaction<'a>,
        id: &Uuid,
    ) -> Result<(), Error> {
        let stmt = txn
            .prepare("delete from metadata_traits where metadata_id = $1")
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
            .prepare("insert into metadata_traits (metadata_id, trait_id) values ($1, $2)")
            .await?;
        txn.execute(&stmt, &[id, trait_id]).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id, trait_id))]
    pub async fn add_trait(
        &self,
        ctx: &BoscaContext,
        id: &Uuid,
        trait_id: &String,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("insert into metadata_traits (metadata_id, trait_id) values ($1, $2)")
            .await?;
        connection.execute(&stmt, &[id, trait_id]).await?;
        self.cache.evict_metadata(id).await;
        self.on_metadata_changed(ctx, id).await?;
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
            .prepare("insert into metadata_categories (metadata_id, category_id) values ($1, $2)")
            .await?;
        txn.execute(&stmt, &[id, category_id]).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, txn, id))]
    async fn delete_categories_txn<'a>(
        &'a self,
        txn: &'a Transaction<'a>,
        id: &Uuid,
    ) -> Result<(), Error> {
        let stmt = txn
            .prepare("delete from metadata_categories where metadata_id = $1")
            .await?;
        txn.execute(&stmt, &[id]).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id, category_id))]
    pub async fn add_category(
        &self,
        ctx: &BoscaContext,
        id: &Uuid,
        category_id: &Uuid,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "insert into metadata_categories (metadata_id, category_id) values ($1, $2)",
            )
            .await?;
        connection.execute(&stmt, &[id, category_id]).await?;
        self.cache.evict_metadata(id).await;
        self.on_metadata_changed(ctx, id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id, category_id))]
    pub async fn delete_category(
        &self,
        ctx: &BoscaContext,
        id: &Uuid,
        category_id: &Uuid,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "delete from metadata_categories where metadata_id = $1 and category_id = $2",
            )
            .await?;
        connection.execute(&stmt, &[id, category_id]).await?;
        self.cache.evict_metadata(id).await;
        self.on_metadata_changed(ctx, id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, relationship))]
    pub async fn add_relationship(
        &self,
        ctx: &BoscaContext,
        relationship: &MetadataRelationshipInput,
    ) -> Result<(), Error> {
        let id1 = Uuid::parse_str(relationship.id1.as_str())?;
        let id2 = Uuid::parse_str(relationship.id2.as_str())?;
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("insert into metadata_relationships (metadata1_id, metadata2_id, relationship, attributes) values ($1, $2, $3, $4)").await?;
        connection
            .execute(
                &stmt,
                &[
                    &id1,
                    &id2,
                    &relationship.relationship,
                    &relationship.attributes,
                ],
            )
            .await?;
        self.cache.evict_metadata(&id1).await;
        self.cache.evict_metadata(&id2).await;
        self.on_metadata_changed(ctx, &id1).await?;
        self.on_metadata_changed(ctx, &id2).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_relationships(&self, id: &Uuid) -> Result<Vec<MetadataRelationship>, Error> {
        if let Some(relationships) = self.cache.get_relationships(id).await {
            return Ok(relationships);
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare("select r.* from metadata_relationships r inner join metadata m on (r.metadata2_id = m.id and m.deleted = false) where metadata1_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[&id]).await?;
        let relationships = rows.iter().map(MetadataRelationship::from).collect();
        self.cache.set_relationships(id, &relationships).await;
        Ok(relationships)
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_relationships_inverse(
        &self,
        id: &Uuid,
    ) -> Result<Vec<MetadataRelationship>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare("select r.* from metadata_relationships r inner join metadata m on (r.metadata2_id = m.id and m.deleted = false) where metadata2_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[&id]).await?;
        Ok(rows.iter().map(MetadataRelationship::from).collect())
    }

    #[tracing::instrument(skip(self, id1, id2))]
    pub async fn get_relationship(
        &self,
        id1: &Uuid,
        id2: &Uuid,
    ) -> Result<Option<MetadataRelationship>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare("select * from metadata_relationships where metadata1_id = $1 and metadata2_id = $2").await?;
        let rows = connection.query(&stmt, &[id1, id2]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        Ok(Some(rows.first().unwrap().into()))
    }

    #[tracing::instrument(skip(self, ctx, id1, id2, relationship, attributes))]
    pub async fn edit_relationship(
        &self,
        ctx: &BoscaContext,
        id1: &Uuid,
        id2: &Uuid,
        relationship: &Option<String>,
        attributes: &Option<Value>,
    ) -> Result<(), Error> {
        let relationship = relationship.to_owned();
        let connection = self.pool.get().await?;
        let stmt = connection.prepare("update metadata_relationships set relationship = $1, attributes = $2 where metadata1_id = $3 and metadata2_id = $4 and (relationship = $1 or relationship is null or relationship = '')").await?;
        connection
            .query(&stmt, &[&relationship, &attributes, id1, id2])
            .await?;
        self.cache.evict_metadata(id1).await;
        self.cache.evict_metadata(id2).await;
        self.on_metadata_changed(ctx, id1).await?;
        self.on_metadata_changed(ctx, id2).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id1, id2, relationship))]
    pub async fn delete_relationship(
        &self,
        ctx: &BoscaContext,
        id1: &Uuid,
        id2: &Uuid,
        relationship: &str,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let relationship = relationship.to_owned();
        let stmt = connection
            .prepare_cached(
                "delete from metadata_relationships where metadata1_id = $1 and metadata2_id = $2 and relationship = $3",
            )
            .await?;
        connection
            .execute(&stmt, &[id1, id2, &relationship])
            .await?;
        self.cache.evict_metadata(id1).await;
        self.cache.evict_metadata(id2).await;
        self.on_metadata_changed(ctx, id1).await?;
        self.on_metadata_changed(ctx, id2).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, metadata, collection_item_attributes))]
    pub async fn add(
        &self,
        ctx: &BoscaContext,
        metadata: &MetadataInput,
        collection_item_attributes: Option<Value>,
    ) -> Result<(Uuid, i32, i32), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let (id, version, active_version) = self
            .add_txn(ctx, &txn, metadata, true, &collection_item_attributes)
            .await?;
        txn.commit().await?;
        Ok((id, version, active_version))
    }

    #[tracing::instrument(skip(
        self,
        ctx,
        txn,
        metadata,
        inherit_permissions,
        collection_item_attributes
    ))]
    pub async fn add_txn<'a>(
        &'a self,
        ctx: &BoscaContext,
        txn: &'a Transaction<'a>,
        metadata: &MetadataInput,
        inherit_permissions: bool,
        collection_item_attributes: &Option<Value>,
    ) -> Result<(Uuid, i32, i32), Error> {
        let mut source_id: Option<Uuid> = None;
        let mut source_identifier: Option<String> = None;
        let mut source_url: Option<String> = None;
        if let Some(source) = &metadata.source {
            source_id = source.id.as_ref().map(|id| Uuid::parse_str(id).unwrap());
            source_identifier = source.identifier.clone();
            source_url = source.source_url.clone();
        }
        let stmt = txn.prepare("insert into metadata (name, type, content_type, content_length, labels, attributes, source_id, source_identifier, source_url, language_tag) values ($1, 'standard', $2, $3, $4, ($5)::jsonb, $6, $7, $8, $9) returning id, version, active_version").await?;
        let labels = metadata.labels.clone().unwrap_or_default();
        let rows = txn
            .query(
                &stmt,
                &[
                    &metadata.name,
                    &metadata.content_type,
                    &metadata.content_length,
                    &labels,
                    &metadata.attributes.as_ref().or(Some(&Value::Null)),
                    &source_id,
                    &source_identifier,
                    &source_url,
                    &metadata.language_tag,
                ],
            )
            .await?;

        let id: Uuid = rows.first().unwrap().get(0);
        let version: i32 = rows.first().unwrap().get(1);
        let active_version: i32 = rows.first().unwrap().get(2);

        let stmt = txn.prepare_cached("insert into slugs (slug, metadata_id) values (case when length($1) > 0 then $1 else slugify($2) end, $3) on conflict (slug) do update set slug = slugify($2) || nextval('duplicate_slug_seq')").await?;
        txn.execute(&stmt, &[&metadata.slug, &metadata.name, &id])
            .await?;

        if let Some(trait_ids) = &metadata.trait_ids {
            for trait_id in trait_ids {
                self.add_trait_txn(txn, &id, trait_id).await?
            }
        }
        if let Some(category_ids) = &metadata.category_ids {
            for category_id in category_ids {
                let cid = Uuid::parse_str(category_id)?;
                self.add_category_txn(txn, &id, &cid).await?
            }
        }
        if let Some(profiles) = &metadata.profiles {
            for (index, profile) in profiles.iter().enumerate() {
                let pid = Uuid::parse_str(&profile.profile_id)?;
                self.add_profile_txn(txn, &id, &pid, &profile.relationship, index as i32)
                    .await?
            }
        }
        if let Some(document) = &metadata.document {
            ctx.content
                .documents
                .add_document_txn(txn, &id, version, document)
                .await?;
        }
        if let Some(document_template) = &metadata.document_template {
            ctx.content
                .documents
                .add_template_txn(txn, &id, version, document_template)
                .await?;
        }
        if let Some(guide) = &metadata.guide {
            ctx.content
                .guides
                .add_guide_txn(ctx, txn, &id, version, guide)
                .await?;
        }
        if let Some(guide_template) = &metadata.guide_template {
            ctx.content
                .guides
                .add_template_txn(txn, &id, version, guide_template)
                .await?;
        }
        if let Some(collection_template) = &metadata.collection_template {
            ctx.content
                .collection_templates
                .add_template_txn(txn, &id, version, collection_template)
                .await?;
        }

        self.ensure_content_type_traits(&id, &metadata.content_type, txn)
            .await?;

        update_metadata_etag(txn, &id).await?;
        self.cache.evict_metadata(&id).await;

        if let Some(parent_collection_id) = &metadata.parent_collection_id {
            let parent_collection_id = Uuid::parse_str(parent_collection_id.as_str())?;
            ctx.content
                .collections
                .add_child_metadata_txn(txn, &parent_collection_id, &id, collection_item_attributes)
                .await?;
            if inherit_permissions {
                ctx.content
                    .metadata_permissions
                    .add_inherited_metadata_permissions_txn(ctx, txn, &parent_collection_id, &id)
                    .await?;
            }
        } else if inherit_permissions {
            let parent_collection_id = Uuid::parse_str("00000000-0000-0000-0000-000000000000")?;
            ctx.content
                .metadata_permissions
                .add_inherited_metadata_permissions_txn(ctx, txn, &parent_collection_id, &id)
                .await?;
        }

        Ok((id, version, active_version))
    }

    #[tracing::instrument(skip(self, ctx, metadatas, inherit_permissions))]
    pub async fn add_all(
        &self,
        ctx: &BoscaContext,
        metadatas: &mut [MetadataChildInput],
        inherit_permissions: bool,
    ) -> Result<Vec<(Uuid, i32, i32)>, Error> {
        let mut conn = self.pool.get().await?;
        let txn = conn.transaction().await?;
        let ids = self
            .add_all_txn(ctx, &txn, metadatas, inherit_permissions)
            .await?;
        txn.commit().await?;
        for (id, _, _) in &ids {
            self.on_metadata_changed(ctx, id).await?
        }
        Ok(ids)
    }

    #[tracing::instrument(skip(self, ctx, txn, metadatas, inherit_permissions))]
    pub async fn add_all_txn(
        &self,
        ctx: &BoscaContext,
        txn: &Transaction<'_>,
        metadatas: &[MetadataChildInput],
        inherit_permissions: bool,
    ) -> Result<Vec<(Uuid, i32, i32)>, Error> {
        let mut new_metadatas = Vec::new();
        for metadata_child in metadatas {
            let (id, version, active_version) = self
                .add_txn(
                    ctx,
                    txn,
                    &metadata_child.metadata,
                    inherit_permissions,
                    &metadata_child.attributes,
                )
                .await?;
            new_metadatas.push((id, version, active_version));
        }
        Ok(new_metadatas)
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn update_storage(&self, _: &BoscaContext, id: &Uuid) -> Result<(), Error> {
        self.cache.evict_metadata(id).await;
        let id_str = id.to_string();
        let next = Utc::now()
            .add(TimeDelta::new(5, 0).unwrap())
            .timestamp_millis();
        let next = next.to_be_bytes();
        if let Err(e) = self
            .client
            .put(&self.update_cache, &id_str, next.to_vec())
            .await
        {
            error!("Failed to put update for metadata {}: {:?}", id, e);
        }
        Ok(())
    }
}
