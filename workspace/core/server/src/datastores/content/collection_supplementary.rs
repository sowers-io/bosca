use crate::context::BoscaContext;
use crate::datastores::notifier::Notifier;
use async_graphql::*;
use deadpool_postgres::{GenericClient, Pool};
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

    async fn on_collection_supplementary_changed(&self, _: &BoscaContext, supplementary_id: &Uuid, collection_id: &Uuid, key: &str, plan_id: Option<Uuid>) -> Result<(), Error> {
        if let Err(e) = self.notifier.collection_supplementary_changed(supplementary_id, collection_id, key, plan_id).await {
            error!("Failed to notify collection supplementary changes: {:?}", e);
        }
        Ok(())
    }

    pub async fn get_supplementary(
        &self,
        supplementary_id: &Uuid,
    ) -> Result<Option<CollectionSupplementary>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "select * from collection_supplementary where id = $1",
            )
            .await?;
        let rows = connection.query(&stmt, &[supplementary_id]).await?;
        if rows.is_empty() {
            return Ok(None);
        }
        Ok(Some(rows.first().unwrap().into()))
    }

    pub async fn get_supplementary_by_key(
        &self,
        collection_id: &Uuid,
        key: &str,
        plan_id: Option<Uuid>,
    ) -> Result<Option<CollectionSupplementary>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "select * from collection_supplementary where collection_id = $1 and key = $2 and plan_id = $3",
            )
            .await?;
        let key = key.to_owned();
        let rows = connection.query(&stmt, &[collection_id, &key, &plan_id]).await?;
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
    ) -> Result<Uuid, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("insert into collection_supplementary (collection_id, key, plan_id, name, content_type, content_length, attributes, source_id, source_identifier) values ($1, $2, $3, $4, $5, $6, $7, $8, $9) returning id").await?;
        let collection_id = Uuid::parse_str(supplementary.collection_id.as_str())?;
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
                    &collection_id,
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
        self.on_collection_supplementary_changed(ctx, &id, &collection_id, &supplementary.key, Some(plan_id))
            .await?;
        Ok(id)
    }

    pub async fn set_supplementary_uploaded(
        &self,
        ctx: &BoscaContext,
        supplementary_id: &Uuid,
        content_type: &str,
        len: usize,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("update collection_supplementary set uploaded = now(), content_type = $1, content_length = $2 where id = $3 returning collection_id, key, plan_id").await?;
        let len: i64 = len as i64;
        let content_type = content_type.to_owned();
        let row = connection
            .query_one(&stmt, &[&content_type, &len, supplementary_id])
            .await?;
        let collection_id: Uuid = row.get("collection_id");
        let key: String = row.get("key");
        let plan_id: Option<Uuid> = row.get("plan_id");
        self.on_collection_supplementary_changed(ctx, supplementary_id, &collection_id, &key, plan_id).await?;
        Ok(())
    }

    pub async fn delete_supplementary(
        &self,
        ctx: &BoscaContext,
        supplementary_id: &Uuid,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("delete from collection_supplementary where id = $1 returning collection_id, key, plan_id").await?;
        let row = connection.query_one(&stmt, &[supplementary_id]).await?;
        let metadata_id: Uuid = row.get("collection_id");
        let key: String = row.get("key");
        let plan_id: Option<Uuid> = row.get("plan_id");
        self.on_collection_supplementary_changed(ctx, supplementary_id, &metadata_id, &key, plan_id).await?;
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
