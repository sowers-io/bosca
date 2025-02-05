use std::sync::Arc;
use async_graphql::Error;
use deadpool_postgres::Pool;
use uuid::Uuid;
use crate::models::profile::profile::Profile;

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
}