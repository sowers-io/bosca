use std::fmt::Debug;
use crate::models::content::collection::Collection;
use crate::models::security::permission::{Permission, PermissionAction};
use crate::models::security::principal::Principal;
use crate::security::evaluator::Evaluator;
use async_graphql::*;
use deadpool_postgres::{GenericClient, Transaction};
use uuid::Uuid;
use bosca_database::TracingPool;
use crate::context::BoscaContext;
use crate::datastores::collection_cache::CollectionCache;

#[derive(Clone)]
pub struct CollectionPermissionsDataStore {
    pool: TracingPool,
    cache: CollectionCache,
}

impl Debug for CollectionPermissionsDataStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CollectionPermissionsDataStore").finish()
    }
}

impl CollectionPermissionsDataStore {
    pub async fn new(pool: TracingPool, cache: CollectionCache) -> Result<Self, Error> {
        Ok(Self {
            pool,
            cache,
        })
    }

    async fn on_collection_changed(&self, ctx: &BoscaContext, id: &Uuid) -> Result<(), Error> {
        ctx.content.collections.on_collection_changed(ctx, id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, collection, principal, action))]
    pub async fn has(
        &self,
        collection: &Collection,
        principal: &Principal,
        groups: &Vec<Uuid>,
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
        let eval = Evaluator::new(collection.id, self.get(&collection.id).await?);
        Ok(eval.evaluate(principal, groups, &action))
    }

    #[tracing::instrument(skip(self, txn, collection, principal, action))]
    pub async fn has_txn(
        &self,
        txn: &Transaction<'_>,
        collection: &Collection,
        principal: &Principal,
        groups: &Vec<Uuid>,
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
        let eval = Evaluator::new(collection.id, self.get_txn(txn, &collection.id).await?);
        Ok(eval.evaluate(principal, groups, &action))
    }

    #[tracing::instrument]
    pub async fn get(&self, id: &Uuid) -> Result<Vec<Permission>, Error> {
        if let Some(permissions) = self.cache.get_permissions(id).await {
            return Ok(permissions);
        }
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select collection_id as entity_id, group_id, action from collection_permissions where collection_id = $1").await?;
        let rows = connection.query(&stmt, &[id]).await?;
        let permissions = rows.iter().map(|r| r.into()).collect();
        self.cache.set_permissions(id, &permissions).await;
        Ok(permissions)
    }

    #[tracing::instrument(skip(self, txn, id))]
    pub async fn get_txn(
        &self,
        txn: &Transaction<'_>,
        id: &Uuid,
    ) -> Result<Vec<Permission>, Error> {
        // if let Some(permissions) = self.cache.get_permissions(id).await {
            // KJB: TODO: There could be an inconsistency if the cache and the transaction are out of sync.  This is something that can be solved later.
            // return Ok(permissions);
        // }
        let stmt = txn.prepare_cached("select collection_id as entity_id, group_id, action from collection_permissions where collection_id = $1").await?;
        let rows = txn.query(&stmt, &[id]).await?;
        let permissions = rows.iter().map(|r| r.into()).collect();
        // KJB: TODO: not storing here in case txn fails
        // self.cache.set_permissions(id, &permissions).await;
        Ok(permissions)
    }

    #[tracing::instrument(skip(self, ctx, permission))]
    pub async fn add(&self, ctx: &BoscaContext, permission: &Permission) -> Result<(), Error> {
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
        self.on_collection_changed(ctx, &permission.entity_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, txn, permission))]
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

    #[tracing::instrument(skip(self, ctx, permission))]
    pub async fn delete(&self, ctx: &BoscaContext, permission: &Permission) -> Result<(), Error> {
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
        self.on_collection_changed(ctx, &permission.entity_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, collection, principal, action))]
    pub async fn has_supplementary_permission(
        &self,
        collection: &Collection,
        principal: &Principal,
        groups: &Vec<Uuid>,
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
        let eval = Evaluator::new(collection.id, self.get(&collection.id).await?);
        Ok(eval.evaluate(principal, groups, &action))
    }
}
