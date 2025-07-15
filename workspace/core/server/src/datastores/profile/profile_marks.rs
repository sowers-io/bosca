use crate::context::BoscaContext;
use async_graphql::Error;
use bosca_database::TracingPool;
use deadpool_postgres::GenericClient;
use serde_json::Value;
use std::fmt::Debug;
use uuid::Uuid;
use crate::models::profiles::profile_mark::ProfileMark;

#[derive(Clone)]
pub struct ProfileMarksDataStore {
    pool: TracingPool,
}

impl Debug for ProfileMarksDataStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProfileBookmarksDataStore").finish()
    }
}

impl ProfileMarksDataStore {
    pub fn new(pool: TracingPool) -> Self {
        Self { pool }
    }

    #[tracing::instrument(skip(self, profile_id))]
    pub async fn get_all_count(
        &self,
        profile_id: &Uuid,
    ) -> async_graphql::Result<i64, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select count(*) as c from profile_marks where profile_id = $1")
            .await?;
        let rows = connection.query(&stmt, &[profile_id]).await?;
        let row = rows.first().unwrap();
        let c = row.get("c");
        Ok(c)
    }

    #[tracing::instrument(skip(self, profile_id, offset, limit))]
    pub async fn get_all(
        &self,
        profile_id: &Uuid,
        offset: i64,
        limit: i64,
    ) -> async_graphql::Result<Vec<ProfileMark>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from profile_marks where profile_id = $1 order by created desc offset $2 limit $3")
            .await?;
        let rows = connection.query(&stmt, &[profile_id, &offset, &limit]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, profile_id))]
    pub async fn get_count(
        &self,
        profile_id: &Uuid,
        metadata_id: Option<Uuid>,
        metadata_version: Option<i32>,
        collection_id: Option<Uuid>,
    ) -> async_graphql::Result<i64, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from profile_marks where profile_id = $1 and ((metadata_id = $2 and metadata_version = $3) or (collection_id = $4))")
            .await?;
        let rows = connection
            .query(
                &stmt,
                &[&profile_id, &metadata_id, &metadata_version, &collection_id],
            )
            .await?;
        let row = rows.first().unwrap();
        let c = row.get("c");
        Ok(c)
    }

    #[tracing::instrument(skip(self, profile_id, metadata_id, metadata_version, collection_id))]
    pub async fn get(
        &self,
        profile_id: &Uuid,
        metadata_id: Option<Uuid>,
        metadata_version: Option<i32>,
        collection_id: Option<Uuid>,
        offset: i64,
        limit: i64,
    ) -> async_graphql::Result<Vec<ProfileMark>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "select * from profile_marks where profile_id = $1 and ((metadata_id = $2 and metadata_version = $3) or (collection_id = $4)) order by created desc offset $5 limit $6",
            ).await?;
        let rows = connection
            .query(
                &stmt,
                &[&profile_id, &metadata_id, &metadata_version, &collection_id, &offset, &limit],
            )
            .await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, profile_id, metadata_id, metadata_version, collection_id, attributes))]
    pub async fn add(
        &self,
        _: &BoscaContext,
        profile_id: &Uuid,
        metadata_id: Option<Uuid>,
        metadata_version: Option<i32>,
        collection_id: Option<Uuid>,
        attributes: Option<Value>,
    ) -> async_graphql::Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        if metadata_id.is_some() && metadata_version.is_some() {
            let stmt = txn
                .prepare_cached(
                    "insert into profile_marks (profile_id, metadata_id, metadata_version, attributes) values ($1, $2, $3, $4) on conflict (profile_id, metadata_id, metadata_version, collection_id) do nothing",
                )
                .await?;
            txn.execute(&stmt, &[&profile_id, &metadata_id, &metadata_version, &attributes])
                .await?;
        } else {
            let stmt = txn
                .prepare_cached(
                    "insert into profile_marks (profile_id, collection_id, attributes) values ($1, $2, $3) on conflict (profile_id, metadata_id, metadata_version, collection_id) do nothing",
                ).await?;
            txn.execute(&stmt, &[&profile_id, &collection_id, &attributes]).await?;
        }
        txn.commit().await?;
        // TODO: fire workflow
        Ok(())
    }

    #[tracing::instrument(skip(self, profile_id, id))]
    pub async fn delete(
        &self,
        _: &BoscaContext,
        profile_id: &Uuid,
        id: i64
    ) -> async_graphql::Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached(
                "delete from profile_marks where profile_id = $1 and id = $2",
            )
            .await?;
        txn.execute(&stmt, &[&profile_id, &id]).await?;
        txn.commit().await?;
        // TODO: fire workflow
        Ok(())
    }
}
