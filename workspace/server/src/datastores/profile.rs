use std::sync::Arc;
use async_graphql::Error;
use deadpool_postgres::Pool;
use uuid::Uuid;
use crate::models::profile::profile::{Profile, ProfileVisibility};

#[derive(Clone)]
pub struct ProfileDataStore {
    pool: Arc<Pool>,
}

impl ProfileDataStore {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }

    pub async fn get_profile_by_principal(&self, id: &Uuid) -> async_graphql::Result<Option<Profile>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from profiles where principal = $1")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        Ok(rows.first().map(|r| r.into()))
    }

    pub async fn add_profile(&self, principal: &Uuid, name: &str, visibility: &ProfileVisibility) -> async_graphql::Result<Uuid, Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("insert into profiles (principal, name, visibility) values ($1, $2, $3) returning id").await?;
        let name = name.to_string();
        let results = txn.query(&stmt, &[principal, &name, visibility]).await?;
        if results.is_empty() {
            return Err(Error::new("failed to create principal"));
        }
        let id = results[0].get("id");
        txn.commit().await?;
        Ok(id)
    }
}