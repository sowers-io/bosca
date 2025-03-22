use crate::context::BoscaContext;
use crate::datastores::notifier::Notifier;
use crate::models::content::metadata::Metadata;
use crate::models::security::permission::{Permission, PermissionAction};
use crate::models::security::principal::Principal;
use crate::security::evaluator::Evaluator;
use async_graphql::*;
use deadpool_postgres::{GenericClient, Pool, Transaction};
use log::error;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct MetadataPermissionsDataStore {
    pool: Arc<Pool>,
    notifier: Arc<Notifier>,
}

impl MetadataPermissionsDataStore {
    pub fn new(pool: Arc<Pool>, notifier: Arc<Notifier>) -> Self {
        Self { pool, notifier }
    }

    async fn on_metadata_changed(&self, id: &Uuid) -> Result<(), Error> {
        if let Err(e) = self.notifier.metadata_changed(id).await {
            error!("Failed to notify metadata changes: {:?}", e);
        }
        Ok(())
    }

    pub async fn get_metadata_permissions(&self, id: &Uuid) -> Result<Vec<Permission>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select metadata_id as entity_id, group_id, action from metadata_permissions where metadata_id = $1").await?;
        let rows = connection.query(&stmt, &[id]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn has_metadata_permission(
        &self,
        metadata: &Metadata,
        principal: &Principal,
        action: PermissionAction,
    ) -> Result<bool, Error> {
        if metadata.deleted {
            return Ok(false);
        }
        if action == PermissionAction::View
            && metadata.public
            && metadata.workflow_state_id == "published"
            && !metadata.deleted
        {
            return Ok(true);
        }
        let eval = Evaluator::new(self.get_metadata_permissions(&metadata.id).await?);
        Ok(eval.evaluate(principal, &action))
    }

    pub async fn has_metadata_content_permission(
        &self,
        metadata: &Metadata,
        principal: &Principal,
        action: PermissionAction,
    ) -> Result<bool, Error> {
        if metadata.deleted {
            return Ok(false);
        }
        if action == PermissionAction::View
            && metadata.public_content
            && metadata.workflow_state_id == "published"
            && !metadata.deleted
        {
            return Ok(true);
        }
        let eval = Evaluator::new(self.get_metadata_permissions(&metadata.id).await?);
        Ok(eval.evaluate(principal, &action))
    }

    pub async fn has_supplementary_permission(
        &self,
        metadata: &Metadata,
        principal: &Principal,
        action: PermissionAction,
    ) -> Result<bool, Error> {
        if metadata.deleted {
            return Ok(false);
        }
        if (action == PermissionAction::View || action == PermissionAction::List)
            && metadata.public_supplementary
            && metadata.workflow_state_id == "published"
            && !metadata.deleted
        {
            return Ok(true);
        }
        let eval = Evaluator::new(self.get_metadata_permissions(&metadata.id).await?);
        Ok(eval.evaluate(principal, &action))
    }

    pub async fn has_metadata_version_permission(
        &self,
        metadata: &Metadata,
        principal: &Principal,
        action: PermissionAction,
    ) -> Result<bool, Error> {
        if metadata.deleted {
            return Ok(false);
        }
        let eval = Evaluator::new(self.get_metadata_permissions(&metadata.id).await?);
        Ok(eval.evaluate(principal, &action))
    }

    pub async fn add_metadata_permission(&self, permission: &Permission) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("insert into metadata_permissions (metadata_id, group_id, action) values ($1, $2, $3)").await?;
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
        self.on_metadata_changed(&permission.entity_id).await?;
        Ok(())
    }

    pub async fn add_inherited_metadata_permissions_txn(
        &self,
        ctx: &BoscaContext,
        txn: &Transaction<'_>,
        parent_collection_id: &Uuid,
        id: &Uuid,
    ) -> Result<(), Error> {
        let permissions = ctx
            .content
            .collection_permissions
            .get_txn(txn, parent_collection_id)
            .await?;
        self.add_metadata_permissions_txn(txn, id, &permissions)
            .await?;
        Ok(())
    }

    pub async fn add_metadata_permissions_txn(
        &self,
        txn: &Transaction<'_>,
        id: &Uuid,
        permissions: &[Permission],
    ) -> Result<(), Error> {
        for permission in permissions.iter() {
            let metadata_permission = Permission {
                entity_id: *id,
                group_id: permission.group_id,
                action: permission.action,
            };
            self.add_metadata_permission_txn(txn, &metadata_permission)
                .await?
        }
        Ok(())
    }

    pub async fn add_metadata_permission_txn(
        &self,
        txn: &Transaction<'_>,
        permission: &Permission,
    ) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into metadata_permissions (metadata_id, group_id, action) values ($1, $2, $3)").await?;
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

    pub async fn delete_metadata_permission(&self, permission: &Permission) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("delete from metadata_permissions where metadata_id = $1 and group_id = $2 and action = $3").await?;
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
        self.on_metadata_changed(&permission.entity_id).await?;
        Ok(())
    }
}
