use crate::context::BoscaContext;
use crate::datastores::metadata_cache::MetadataCache;
use crate::datastores::notifier::Notifier;
use crate::models::content::metadata_supplementary::{
    MetadataSupplementary, MetadataSupplementaryInput,
};
use async_graphql::*;
use bosca_database::TracingPool;
use log::error;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct MetadataSupplementaryDataStore {
    pool: TracingPool,
    cache: MetadataCache,
    notifier: Arc<Notifier>,
}

impl MetadataSupplementaryDataStore {
    pub fn new(pool: TracingPool, cache: MetadataCache, notifier: Arc<Notifier>) -> Self {
        Self {
            pool,
            cache,
            notifier,
        }
    }

    #[tracing::instrument(skip(self, ctx, id))]
    async fn on_metadata_changed(&self, ctx: &BoscaContext, id: &Uuid) -> Result<(), Error> {
        ctx.content.metadata.on_metadata_changed(ctx, id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, supplementary_id, metadata_id, key, plan_id))]
    async fn on_metadata_supplementary_changed(
        &self,
        _: &BoscaContext,
        supplementary_id: &Uuid,
        metadata_id: &Uuid,
        key: &str,
        plan_id: Option<Uuid>,
    ) -> Result<(), Error> {
        self.cache.evict_supplementary(supplementary_id).await;
        self.cache.evict_metadata(metadata_id).await;
        if let Err(e) = self
            .notifier
            .metadata_supplementary_changed(
                supplementary_id,
                metadata_id,
                key,
                plan_id.map(|p| p.to_string()),
            )
            .await
        {
            error!("Failed to notify metadata supplementary changes: {:?}", e);
        }
        Ok(())
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_supplementary(
        &self,
        id: &Uuid,
    ) -> Result<Option<MetadataSupplementary>, Error> {
        if let Some(supplementary) = self.cache.get_supplementary(id).await {
            return Ok(Some(supplementary));
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from metadata_supplementary where id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        let supplementary: MetadataSupplementary = rows.first().unwrap().into();
        self.cache
            .set_supplementary(&supplementary.id, &supplementary)
            .await;
        Ok(Some(supplementary))
    }

    #[tracing::instrument(skip(self, metadata_id, key))]
    pub async fn get_supplementary_by_key(
        &self,
        metadata_id: &Uuid,
        key: &str,
    ) -> Result<Option<MetadataSupplementary>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from metadata_supplementary where metadata_id = $1 and key = $2 order by created desc limit 1")
            .await?;
        let key = key.to_owned();
        let rows = connection.query(&stmt, &[metadata_id, &key]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        Ok(Some(rows.first().unwrap().into()))
    }

    #[tracing::instrument(skip(self, metadata_id, key, plan_id))]
    pub async fn get_supplementary_by_key_and_plan_id(
        &self,
        metadata_id: &Uuid,
        key: &str,
        plan_id: &Uuid,
    ) -> Result<Option<MetadataSupplementary>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from metadata_supplementary where metadata_id = $1 and key = $2 and plan_id = $3")
            .await?;
        let key = key.to_owned();
        let rows = connection
            .query(&stmt, &[metadata_id, &key, &plan_id])
            .await?;
        if rows.is_empty() {
            return Ok(None);
        }
        Ok(Some(rows.first().unwrap().into()))
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_supplementaries(
        &self,
        id: &Uuid,
    ) -> Result<Vec<MetadataSupplementary>, Error> {
        if let Some(supplementaries) = self.cache.get_supplementaries(id).await {
            let mut s = Vec::new();
            for supplementary in supplementaries {
                if let Some(supplementary) = self.get_supplementary(&supplementary).await? {
                    s.push(supplementary);
                }
            }
            return Ok(s);
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select id from metadata_supplementary where metadata_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        let ids: Vec<Uuid> = rows.iter().map(|r| r.get("id")).collect();
        self.cache.set_supplementaries(id, &ids).await;
        let mut s = Vec::new();
        for supplementary in ids {
            if let Some(supplementary) = self.get_supplementary(&supplementary).await? {
                s.push(supplementary);
            }
        }
        Ok(s)
    }

    #[tracing::instrument(skip(self, ctx, supplementary))]
    pub async fn add_supplementary(
        &self,
        ctx: &BoscaContext,
        supplementary: &MetadataSupplementaryInput,
    ) -> Result<Uuid, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("insert into metadata_supplementary (metadata_id, key, plan_id, name, content_type, content_length, attributes, source_id, source_identifier) values ($1, $2, $3, $4, $5, $6, $7, $8, $9) returning id").await?;
        let metadata_id = Uuid::parse_str(supplementary.metadata_id.as_str())?;
        let plan_id = Uuid::parse_str(supplementary.plan_id.as_ref())?;
        let sid = if supplementary.source_identifier.is_some() {
            Some(Uuid::parse_str(
                supplementary.source_identifier.as_ref().unwrap().as_str(),
            )?)
        } else {
            None
        };
        let row = connection
            .query_one(
                &stmt,
                &[
                    &metadata_id,
                    &supplementary.key,
                    &plan_id,
                    &supplementary.name,
                    &supplementary.content_type,
                    &supplementary.content_length,
                    &supplementary.attributes,
                    &sid,
                    &supplementary.source_identifier,
                ],
            )
            .await?;
        let id: Uuid = row.get("id");
        self.on_metadata_supplementary_changed(
            ctx,
            &id,
            &metadata_id,
            &supplementary.key,
            Some(plan_id),
        )
        .await?;
        Ok(id)
    }

    #[tracing::instrument(skip(self, ctx, supplementary_id, content_type, len))]
    pub async fn set_supplementary_uploaded(
        &self,
        ctx: &BoscaContext,
        supplementary_id: &Uuid,
        content_type: &str,
        len: usize,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("update metadata_supplementary set uploaded = now(), content_type = $1, content_length = $2 where id = $3 returning metadata_id, key, plan_id").await?;
        let len: i64 = len as i64;
        let content_type = content_type.to_owned();
        let result = connection
            .query_one(&stmt, &[&content_type, &len, supplementary_id])
            .await?;
        let metadata_id: Uuid = result.get("metadata_id");
        let key: String = result.get("key");
        let plan_id: Option<Uuid> = result.get("plan_id");
        self.on_metadata_supplementary_changed(ctx, supplementary_id, &metadata_id, &key, plan_id)
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id))]
    pub async fn delete_supplementary(&self, ctx: &BoscaContext, id: &Uuid) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "delete from metadata_supplementary where id = $1 returning metadata_id, key, plan_id",
            )
            .await?;
        let result = connection.query_one(&stmt, &[id]).await?;
        let metadata_id: Uuid = result.get("metadata_id");
        let key: String = result.get("key");
        let plan_id: Option<Uuid> = result.get("plan_id");
        self.on_metadata_supplementary_changed(ctx, id, &metadata_id, &key, plan_id)
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id))]
    pub async fn detach_supplementary(&self, ctx: &BoscaContext, id: &Uuid) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "update metadata_supplementary set plan_id = null where id = $1 returning metadata_id, key, plan_id",
            )
            .await?;
        let result = connection.query_one(&stmt, &[id]).await?;
        let metadata_id: Uuid = result.get("metadata_id");
        let key: String = result.get("key");
        let plan_id: Option<Uuid> = result.get("plan_id");
        self.on_metadata_supplementary_changed(ctx, id, &metadata_id, &key, plan_id)
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, id, public))]
    pub async fn set_supplementary_public(
        &self,
        ctx: &BoscaContext,
        id: &Uuid,
        public: bool,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "update metadata set public_supplementary = $1, modified = now() where id = $2",
            )
            .await?;
        connection.execute(&stmt, &[&public, id]).await?;
        self.on_metadata_changed(ctx, id).await?;
        Ok(())
    }
}
