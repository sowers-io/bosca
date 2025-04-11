use crate::datastores::cache::cache::{BoscaCache, BoscaCacheInterface};
use crate::datastores::cache::manager::BoscaCacheManager;
use crate::datastores::cache::tiered_cache::TieredCacheType;
use crate::datastores::notifier::Notifier;
use crate::models::content::collection::Collection;
use crate::models::security::permission::{Permission, PermissionAction};
use crate::models::security::principal::Principal;
use crate::security::evaluator::Evaluator;
use async_graphql::*;
use deadpool_postgres::{GenericClient, Pool, Transaction};
use log::error;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct CollectionPermissionsDataStore {
    pool: Arc<Pool>,
    cache: BoscaCache<Uuid, Vec<Permission>>,
    notifier: Arc<Notifier>,
}

impl CollectionPermissionsDataStore {
    pub async fn new(pool: Arc<Pool>, cache: &mut BoscaCacheManager, notifier: Arc<Notifier>) -> Self {
        Self {
            pool,
            cache: cache.new_id_tiered_cache(
                "collection_permissions",
                5000,
                TieredCacheType::Collection,
            ).await,
            notifier,
        }
    }

    async fn on_collection_changed(&self, id: &Uuid) -> Result<(), Error> {
        if let Err(e) = self.notifier.collection_changed(id).await {
            error!("Failed to notify collection changes: {:?}", e);
        }
        Ok(())
    }

    pub async fn has(
        &self,
        collection: &Collection,
        principal: &Principal,
        action: PermissionAction,
    ) -> Result<bool, Error> {
        if collection.deleted {
            return Ok(false);
        }
        if action == PermissionAction::View
            && collection.public
            && collection.workflow_state_id == "published"
        {
            return Ok(true);
        }
        if action == PermissionAction::List
            && collection.public_list
            && collection.workflow_state_id == "published"
        {
            return Ok(true);
        }
        let eval = Evaluator::new(self.get(&collection.id).await?);
        Ok(eval.evaluate(principal, &action))
    }

    pub async fn has_txn(
        &self,
        txn: &Transaction<'_>,
        collection: &Collection,
        principal: &Principal,
        action: PermissionAction,
    ) -> Result<bool, Error> {
        if collection.deleted {
            return Ok(false);
        }
        if action == PermissionAction::View
            && collection.public
            && collection.workflow_state_id == "published"
        {
            return Ok(true);
        }
        if action == PermissionAction::List
            && collection.public_list
            && collection.workflow_state_id == "published"
        {
            return Ok(true);
        }
        let eval = Evaluator::new(self.get_txn(txn, &collection.id).await?);
        Ok(eval.evaluate(principal, &action))
    }

    pub async fn get(&self, id: &Uuid) -> Result<Vec<Permission>, Error> {
        if let Some(permissions) = self.cache.get(id).await {
            return Ok(permissions);
        }
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select collection_id as entity_id, group_id, action from collection_permissions where collection_id = $1").await?;
        let rows = connection.query(&stmt, &[id]).await?;
        let permissions = rows.iter().map(|r| r.into()).collect();
        self.cache.set(id, &permissions).await;
        Ok(permissions)
    }

    pub async fn get_txn(
        &self,
        txn: &Transaction<'_>,
        id: &Uuid,
    ) -> Result<Vec<Permission>, Error> {
        if let Some(permissions) = self.cache.get(id).await {
            return Ok(permissions);
        }
        let stmt = txn.prepare_cached("select collection_id as entity_id, group_id, action from collection_permissions where collection_id = $1").await?;
        let rows = txn.query(&stmt, &[id]).await?;
        let permissions = rows.iter().map(|r| r.into()).collect();
        self.cache.set(id, &permissions).await;
        Ok(permissions)
    }

    pub async fn add(&self, permission: &Permission) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("insert into collection_permissions (collection_id, group_id, action) values ($1, $2, $3) on conflict do nothing").await?;
        connection
            .execute(
                &stmt,
                &[
                    &permission.entity_id,
                    &permission.group_id,
                    &permission.action,
                ],
            )
            .await?;
        self.on_collection_changed(&permission.entity_id).await?;
        Ok(())
    }

    pub async fn add_txn(
        &self,
        txn: &Transaction<'_>,
        permission: &Permission,
    ) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into collection_permissions (collection_id, group_id, action) values ($1, $2, $3) on conflict do nothing").await?;
        txn.execute(
            &stmt,
            &[
                &permission.entity_id,
                &permission.group_id,
                &permission.action,
            ],
        )
        .await?;
        Ok(())
    }

    pub async fn delete(&self, permission: &Permission) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("delete from collection_permissions where collection_id = $1 and group_id = $2 and action = $3").await?;
        connection
            .execute(
                &stmt,
                &[
                    &permission.entity_id,
                    &permission.group_id,
                    &permission.action,
                ],
            )
            .await?;
        self.on_collection_changed(&permission.entity_id).await?;
        Ok(())
    }

    pub async fn has_supplementary_permission(
        &self,
        collection: &Collection,
        principal: &Principal,
        action: PermissionAction,
    ) -> Result<bool, Error> {
        if collection.deleted {
            return Ok(false);
        }
        if (action == PermissionAction::List || action == PermissionAction::View)
            && collection.public_supplementary
            && collection.workflow_state_id == "published"
            && !collection.deleted
        {
            return Ok(true);
        }
        let eval = Evaluator::new(self.get(&collection.id).await?);
        Ok(eval.evaluate(principal, &action))
    }
}
