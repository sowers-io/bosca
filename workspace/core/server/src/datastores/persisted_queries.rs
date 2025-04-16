use async_graphql::*;
use deadpool_postgres::GenericClient;
use async_graphql::parser::parse_query;
use bosca_database::TracingPool;
use crate::queries::PersistedQueriesCache;

#[derive(Clone)]
pub struct PersistedQueriesDataStore {
    pool: TracingPool,
    pub cache: PersistedQueriesCache,
}

#[derive(SimpleObject, Clone)]
pub struct PersistedQuery {
    pub application: String,
    pub sha256: String,
    pub query: String,
}

#[derive(InputObject, Clone)]
pub struct PersistedQueryInput {
    pub sha256: String,
    pub query: String,
}

impl PersistedQueriesDataStore {
    pub async fn new(pool: TracingPool) -> Self {
        let ds = Self { pool, cache: PersistedQueriesCache::new() };
        ds.update_cache().await.unwrap();
        ds
    }

    #[tracing::instrument(skip(self))]
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

    #[tracing::instrument(skip(self))]
    pub async fn get_queries(&self) -> Result<Vec<PersistedQuery>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from gql_persisted_queries")
            .await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows
            .iter()
            .map(|r| PersistedQuery {
                application: r.get("application"),
                sha256: r.get("sha256"),
                query: r.get("query"),
            })
            .collect())
    }

    #[tracing::instrument(skip(self, sha256))]
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
                application: r.get("application"),
                sha256: r.get("sha256"),
                query: r.get("query"),
            }))
    }

    #[tracing::instrument(skip(self, application, queries))]
    pub async fn add_queries(&self, application: &str, queries: &Vec<PersistedQueryInput>) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached("insert into gql_persisted_queries (application, sha256, query) values ($1, $2, $3) on conflict (application, sha256) do update set query = $3")
            .await?;
        let application = application.to_string();
        for query in queries {
            let sha256 = query.sha256.to_owned();
            let query = query.query.to_owned();
            txn.execute(&stmt, &[&application, &sha256, &query]).await?;
        }
        txn.commit().await?;
        self.update_cache().await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, sha256, application, query))]
    pub async fn add_query(&self, sha256: &str, application: &str, query: &str) -> Result<(), Error> {
        let sha256 = sha256.to_owned();
        let query = query.to_owned();
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("insert into gql_persisted_queries (application, sha256, query) values ($1, $2, $3) on conflict (application, sha256) do update set query = $2")
            .await?;
        let application = application.to_string();
        connection.execute(&stmt, &[&application, &sha256, &query]).await?;
        self.update_cache().await?;
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn delete_queries(&self) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("delete from gql_persisted_queries")
            .await?;
        connection.execute(&stmt, &[]).await?;
        self.update_cache().await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, application, sha256))]
    pub async fn delete_query(&self, application: &str, sha256: &str) -> Result<(), Error> {
        let sha256 = sha256.to_owned();
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("delete from gql_persisted_queries where application = $1, sha256 = $2")
            .await?;
        let application = application.to_string();
        connection.execute(&stmt, &[&application, &sha256]).await?;
        self.update_cache().await?;
        Ok(())
    }
}
