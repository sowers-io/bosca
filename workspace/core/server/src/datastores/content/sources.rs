use crate::models::content::source::{Source, SourceInput};
use async_graphql::Error;
use uuid::Uuid;
use bosca_database::TracingPool;

#[derive(Clone)]
pub struct SourcesDataStore {
    pool: TracingPool,
}

impl SourcesDataStore {
    pub fn new(pool: TracingPool) -> Self {
        Self { pool }
    }

    #[tracing::instrument(skip(self, source))]
    pub async fn add(&self, source: &SourceInput) -> Result<Uuid, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("insert into sources (name, description, configuration) values ($1, $2, $3) returning id")
            .await?;
        let rows = connection.query_one(&stmt, &[&source.name, &source.description, &source.configuration]).await?;
        let id = rows.get(0);
        Ok(id)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_sources(&self) -> Result<Vec<Source>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from sources order by name asc")
            .await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_source_by_id(
        &self,
        id: &Uuid,
    ) -> Result<Option<Source>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from sources where id = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        Ok(rows.first().map(|r| r.into()))
    }

    #[tracing::instrument(skip(self, name))]
    pub async fn get_source_by_name(
        &self,
        name: &String,
    ) -> Result<Option<Source>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from sources where name = $1")
            .await?;
        let rows = connection.query(&stmt, &[name]).await?;
        Ok(rows.first().map(|r| r.into()))
    }
}
