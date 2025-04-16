use crate::datastores::notifier::Notifier;
use crate::models::content::category::{Category, CategoryInput};
use async_graphql::*;
use deadpool_postgres::{GenericClient, Pool};
use log::error;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct CategoriesDataStore {
    pool: Arc<Pool>,
    notifier: Arc<Notifier>,
}

impl CategoriesDataStore {
    pub fn new(pool: Arc<Pool>, notifier: Arc<Notifier>) -> Self {
        Self { pool, notifier }
    }

    #[tracing::instrument(skip(self, id))]
    async fn on_category_changed(&self, id: &Uuid) -> Result<(), Error> {
        if let Err(e) = self.notifier.category_changed(id).await {
            error!("Failed to notify category changes: {:?}", e);
        }
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_all(&self) -> Result<Vec<Category>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from categories order by name asc")
            .await?;
        let result = connection.query(&stmt, &[]).await?;
        Ok(result.iter().map(|c| c.into()).collect())
    }

    #[tracing::instrument(skip(self, category))]
    pub async fn add(&self, category: &CategoryInput) -> Result<Uuid, Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached("insert into categories (name) values ($1) returning id")
            .await?;
        let result = txn.query(&stmt, &[&category.name]).await?;
        let category_id = result.first().unwrap().get("id");
        txn.commit().await?;
        self.on_category_changed(&category_id).await?;
        Ok(category_id)
    }

    #[tracing::instrument(skip(self, id, category))]
    pub async fn edit(&self, id: &Uuid, category: &CategoryInput) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached("update categories set name = $1 where id = $2")
            .await?;
        txn.execute(&stmt, &[&category.name, id]).await?;
        txn.commit().await?;
        self.on_category_changed(id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn delete(&self, id: &Uuid) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached("delete from categories where id = $1")
            .await?;
        txn.execute(&stmt, &[id]).await?;
        txn.commit().await?;
        self.on_category_changed(id).await?;
        Ok(())
    }
}
