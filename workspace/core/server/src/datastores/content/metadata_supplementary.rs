use crate::context::BoscaContext;
use crate::datastores::notifier::Notifier;
use crate::models::content::supplementary::{MetadataSupplementary, MetadataSupplementaryInput};
use async_graphql::*;
use deadpool_postgres::Pool;
use log::error;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct MetadataSupplementaryDataStore {
    pool: Arc<Pool>,
    notifier: Arc<Notifier>,
}

impl MetadataSupplementaryDataStore {
    pub fn new(pool: Arc<Pool>, notifier: Arc<Notifier>) -> Self {
        Self { pool, notifier }
    }

    async fn on_metadata_changed(&self, id: &Uuid) -> Result<(), Error> {
        if let Err(e) = self.notifier.metadata_changed(id).await {
            error!("Failed to notify metadata changes: {:?}", e);
        }
        Ok(())
    }

    async fn on_metadata_supplementary_changed(&self, id: &Uuid, key: &str) -> Result<(), Error> {
        if let Err(e) = self.notifier.metadata_supplementary_changed(id, key).await {
            error!("Failed to notify metadata supplementary changes: {:?}", e);
        }
        Ok(())
    }

    pub async fn get_supplementary(
        &self,
        id: &Uuid,
        key: &String,
    ) -> Result<Option<MetadataSupplementary>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "select * from metadata_supplementary where metadata_id = $1 and key = $2",
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
    ) -> Result<Vec<MetadataSupplementary>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from metadata_supplementary where metadata_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn add_supplementary(
        &self,
        supplementary: &MetadataSupplementaryInput,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("insert into metadata_supplementary (metadata_id, key, name, content_type, content_length, attributes, source_id, source_identifier) values ($1, $2, $3, $4, $5, $6, $7, $8)").await?;
        let id = Uuid::parse_str(supplementary.metadata_id.as_str())?;
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
                    &supplementary.name,
                    &supplementary.content_type,
                    &supplementary.content_length,
                    &supplementary.attributes,
                    &sid,
                    &supplementary.source_identifier,
                ],
            )
            .await?;
        self.on_metadata_supplementary_changed(&id, &supplementary.key)
            .await?;
        Ok(())
    }

    pub async fn set_supplementary_uploaded(
        &self,
        metadata_id: &Uuid,
        key: &str,
        content_type: &str,
        len: usize,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("update metadata_supplementary set uploaded = now(), content_type = $1, content_length = $2 where metadata_id = $3 and key = $4").await?;
        let len: i64 = len as i64;
        let key = key.to_owned();
        let content_type = content_type.to_owned();
        connection
            .execute(&stmt, &[&content_type, &len, &metadata_id, &key])
            .await?;
        self.on_metadata_supplementary_changed(metadata_id, &key)
            .await?;
        Ok(())
    }

    pub async fn delete_supplementary(
        &self,
        ctx: &BoscaContext,
        metadata_id: &Uuid,
        key: &String,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "delete from metadata_supplementary where metadata_id = $1 and key = $2",
            )
            .await?;
        connection.execute(&stmt, &[&metadata_id, &key]).await?;
        ctx.content.metadata.index_metadata(ctx, metadata_id, None).await?;
        self.on_metadata_supplementary_changed(metadata_id, key).await?;
        Ok(())
    }

    pub async fn set_supplementary_public(&self, id: &Uuid, public: bool) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "update metadata set public_supplementary = $1, modified = now() where id = $2",
            )
            .await?;
        connection.execute(&stmt, &[&public, id]).await?;
        self.on_metadata_changed(id).await?;
        Ok(())
    }
}
