use async_graphql::*;
use deadpool_postgres::{GenericClient, Pool};
use std::sync::Arc;
use async_graphql::parser::parse_query;
use crate::queries::PersistedQueriesCache;

#[derive(Clone)]
pub struct PersistedQueriesDataStore {
    pool: Arc<Pool>,
    pub cache: PersistedQueriesCache,
}

#[derive(SimpleObject, Clone)]
pub struct PersistedQuery {
    pub sha256: String,
    pub query: String,
}

#[derive(InputObject, Clone)]
pub struct PersistedQueryInput {
    pub sha256: String,
    pub query: String,
}

impl PersistedQueriesDataStore {
    pub async fn new(pool: Arc<Pool>) -> Self {
        let ds = Self { pool, cache: PersistedQueriesCache::new() };
        ds.update_cache().await.unwrap();
        ds
    }

    pub async fn update_cache(&self) -> Result<(), Error> {
        let queries = self.get_queries().await?;
        let mut documents = Vec::new();
        for query in queries {
            let document = parse_query(&query.query)?;
            documents.push((query.sha256, document));
        }
        let mut cache = self.cache.queries.write().await;
        cache.clear();
        cache.extend(documents);
        Ok(())
    }
    
    pub async fn get_queries(&self) -> Result<Vec<PersistedQuery>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from gql_persisted_queries")
            .await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows
            .iter()
            .map(|r| PersistedQuery {
                sha256: r.get("sha256"),
                query: r.get("query"),
            })
            .collect())
    }

    pub async fn get_query(&self, sha256: &str) -> Result<Option<PersistedQuery>, Error> {
        let sha256 = sha256.to_owned();
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from gql_persisted_queries where sha256 = $1")
            .await?;
        let rows = connection.query(&stmt, &[&sha256]).await?;
        Ok(rows
            .first()
            .map(|r| PersistedQuery {
                sha256: r.get("sha256"),
                query: r.get("query"),
            }))
    }

    pub async fn add_queries(&self, queries: &Vec<PersistedQueryInput>) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached("insert into gql_persisted_queries (sha256, query) values ($1, $2) on conflict (sha256) do update set query = $2")
            .await?;
        for query in queries {
            let sha256 = query.sha256.to_owned();
            let query = query.query.to_owned();
            txn.execute(&stmt, &[&sha256, &query]).await?;
        }
        txn.commit().await?;
        self.update_cache().await?;
        Ok(())
    }

    pub async fn add_query(&self, sha256: &str, query: &str) -> Result<(), Error> {
        let sha256 = sha256.to_owned();
        let query = query.to_owned();
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("insert into gql_persisted_queries (sha256, query) values ($1, $2) on conflict (sha256) do update set query = $2")
            .await?;
        connection.execute(&stmt, &[&sha256, &query]).await?;
        self.update_cache().await?;
        Ok(())
    }

    pub async fn delete_queries(&self) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("delete from gql_persisted_queries")
            .await?;
        connection.execute(&stmt, &[]).await?;
        self.update_cache().await?;
        Ok(())
    }
    
    pub async fn delete_query(&self, sha256: &str) -> Result<(), Error> {
        let sha256 = sha256.to_owned();
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("delete from gql_persisted_queries where sha256 = $1")
            .await?;
        connection.execute(&stmt, &[&sha256]).await?;
        self.update_cache().await?;
        Ok(())
    }
}