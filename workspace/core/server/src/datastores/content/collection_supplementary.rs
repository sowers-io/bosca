use crate::context::BoscaContext;
use crate::datastores::notifier::Notifier;
use async_graphql::*;
use deadpool_postgres::Pool;
use log::error;
use std::sync::Arc;
use uuid::Uuid;
use crate::models::content::collection_supplementary::{CollectionSupplementary, CollectionSupplementaryInput};

#[derive(Clone)]
pub struct CollectionSupplementaryDataStore {
    pool: Arc<Pool>,
    notifier: Arc<Notifier>,
}

impl CollectionSupplementaryDataStore {
    pub fn new(pool: Arc<Pool>, notifier: Arc<Notifier>) -> Self {
        Self { pool, notifier }
    }

    async fn on_collection_changed(&self, ctx: &BoscaContext, id: &Uuid) -> Result<(), Error> {
        ctx.content.collections.update_storage(ctx, id).await?;
        if let Err(e) = self.notifier.collection_changed(id).await {
            error!("Failed to notify collection changes: {:?}", e);
        }
        Ok(())
    }

    async fn on_collection_supplementary_changed(&self, _: &BoscaContext, id: &Uuid, key: &str, plan_id: Option<String>) -> Result<(), Error> {
        if let Err(e) = self.notifier.collection_supplementary_changed(id, key, plan_id).await {
            error!("Failed to notify collection supplementary changes: {:?}", e);
        }
        Ok(())
    }

    pub async fn get_supplementary(
        &self,
        id: &Uuid,
        key: &String,
    ) -> Result<Option<CollectionSupplementary>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "select * from collection_supplementary where collection_id = $1 and key = $2",
            )
            .await?;
        let rows = connection.query(&stmt, &[id, key]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        Ok(Some(rows.first().unwrap().into()))
    }

    pub async fn get_supplementaries(
        &self,
        id: &Uuid,
    ) -> Result<Vec<CollectionSupplementary>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from collection_supplementary where collection_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn add_supplementary(
        &self,
        ctx: &BoscaContext,
        supplementary: &CollectionSupplementaryInput,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("insert into collection_supplementary (collection_id, key, plan_id, name, content_type, content_length, attributes, source_id, source_identifier) values ($1, $2, $3, $4, $5, $6, $7, $8)").await?;
        let id = Uuid::parse_str(supplementary.collection_id.as_str())?;
        let sid = if supplementary.source_identifier.is_some() {
            Some(Uuid::parse_str(
                supplementary.source_identifier.as_ref().unwrap().as_str(),
            )?)
        } else {
            None
        };
        connection
            .execute(
                &stmt,
                &[
                    &id,
                    &supplementary.key,
                    &supplementary.plan_id,
                    &supplementary.name,
                    &supplementary.content_type,
                    &supplementary.content_length,
                    &supplementary.attributes,
                    &sid,
                    &supplementary.source_identifier,
                ],
            )
            .await?;
        self.on_collection_supplementary_changed(ctx, &id, &supplementary.key, supplementary.plan_id.as_ref().map(|p| p.to_string()))
            .await?;
        Ok(())
    }

    pub async fn set_supplementary_uploaded(
        &self,
        ctx: &BoscaContext,
        metadata_id: &Uuid,
        key: &str,
        plan_id: Option<Uuid>,
        content_type: &str,
        len: usize,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("update collection_supplementary set uploaded = now(), content_type = $1, content_length = $2 where collection_id = $3 and key = $4 and plan_id = $5").await?;
        let len: i64 = len as i64;
        let key = key.to_owned();
        let content_type = content_type.to_owned();
        connection
            .execute(&stmt, &[&content_type, &len, &metadata_id, &key, &plan_id])
            .await?;
        self.on_collection_supplementary_changed(ctx, metadata_id, &key, plan_id.map(|p| p.to_string()))
            .await?;
        Ok(())
    }

    pub async fn delete_supplementary(
        &self,
        ctx: &BoscaContext,
        metadata_id: &Uuid,
        key: &String,
        plan_id: Option<Uuid>,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "delete from collection_supplementary where collection_id = $1 and key = $2 and plan_id = $3",
            )
            .await?;
        connection.execute(&stmt, &[&metadata_id, &key, &plan_id]).await?;
        self.on_collection_supplementary_changed(ctx, metadata_id, key, plan_id.map(|p| p.to_string())).await?;
        Ok(())
    }

    pub async fn set_supplementary_public(&self, ctx: &BoscaContext, id: &Uuid, public: bool) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "update collections set public_supplementary = $1, modified = now() where id = $2",
            )
            .await?;
        connection.execute(&stmt, &[&public, id]).await?;
        self.on_collection_changed(ctx, id).await?;
        Ok(())
    }
}
