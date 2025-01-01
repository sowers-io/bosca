use std::sync::Arc;
use async_graphql::Error;
use deadpool_postgres::Transaction;
use meilisearch_sdk::client::Client;
use uuid::Uuid;
use crate::datastores::content::ContentDataStore;
use crate::datastores::persisted_queries::PersistedQueriesDataStore;
use crate::datastores::security::SecurityDataStore;
use crate::datastores::workflow::WorkflowDataStore;
use crate::graphql::content::storage::ObjectStorage;
use crate::models::content::collection::Collection;
use crate::models::content::metadata::Metadata;
use crate::models::security::permission::PermissionAction;
use crate::models::security::principal::Principal;
use crate::queue::message_queues::MessageQueues;

use redis::Client as RedisClient;

#[derive(Clone)]
pub struct BoscaContext {
    pub content: ContentDataStore,
    pub security: SecurityDataStore,
    pub storage: ObjectStorage,
    pub workflow: WorkflowDataStore,
    pub queries: PersistedQueriesDataStore,
    pub redis: Arc<RedisClient>,
    pub search: Arc<Client>,
    pub principal: Principal,
    pub messages: MessageQueues,
}

impl BoscaContext {
    pub async fn check_metadata_action_principal(&self, principal: &Principal, id: &Uuid, action: PermissionAction) -> Result<Metadata, Error> {
        match self.content.get_metadata(id).await? {
            Some(metadata) => {
                if !self.content
                    .has_metadata_permission(&metadata, principal, action)
                    .await?
                {
                    let admin = self.security.get_administrators_group().await?;
                    if !self.principal.has_group(&admin.id) {
                        return Err(Error::new("invalid permissions"));
                    }
                }
                Ok(metadata)
            }
            None => Err(Error::new(format!(
                "metadata not found: {}",
                id
            ))),
        }
    }

    pub async fn check_metadata_action(&self, id: &Uuid, action: PermissionAction) -> Result<Metadata, Error> {
        match self.content.get_metadata(id).await? {
            Some(metadata) => {
                if !self.content
                    .has_metadata_permission(&metadata, &self.principal, action)
                    .await?
                {
                    let admin = self.security.get_administrators_group().await?;
                    if !self.principal.has_group(&admin.id) {
                        return Err(Error::new("invalid permissions"));
                    }
                }
                Ok(metadata)
            }
            None => Err(Error::new(format!(
                "metadata not found: {}",
                id
            ))),
        }
    }

    pub async fn check_metadata_version_action(&self, id: &Uuid, version: i32, action: PermissionAction) -> Result<Metadata, Error> {
        match self.content.get_metadata_by_version(id, version).await? {
            Some(metadata) => {
                if !self.content
                    .has_metadata_permission(&metadata, &self.principal, action)
                    .await?
                {
                    let admin = self.security.get_administrators_group().await?;
                    if !self.principal.has_group(&admin.id) {
                        return Err(Error::new("invalid permissions"));
                    }
                }
                Ok(metadata)
            }
            None => Err(Error::new(format!(
                "metadata not found: {} / {}",
                id, version
            ))),
        }
    }

    pub async fn check_collection_action_txn(&self, txn: &Transaction<'_>, id: &Uuid, action: PermissionAction) -> Result<Collection, Error> {
        match self.content.get_collection(id).await? {
            Some(collection) => {
                if !self.content
                    .has_collection_permission_txn(txn, &collection, &self.principal, action)
                    .await?
                {
                    let admin = self.security.get_administrators_group().await?;
                    if !self.principal.has_group(&admin.id) {
                        return Err(Error::new("invalid permissions"));
                    }
                }
                Ok(collection)
            }
            None => Err(Error::new(format!(
                "collection not found: {}",
                id
            ))),
        }
    }

    pub async fn check_collection_action(&self, id: &Uuid, action: PermissionAction) -> Result<Collection, Error> {
        match self.content.get_collection(id).await? {
            Some(collection) => {
                if !self.content
                    .has_collection_permission(&collection, &self.principal, action)
                    .await?
                {
                    let admin = self.security.get_administrators_group().await?;
                    if !self.principal.has_group(&admin.id) {
                        return Err(Error::new("invalid permissions"));
                    }
                }
                Ok(collection)
            }
            None => Err(Error::new(format!(
                "collection not found: {}",
                id
            ))),
        }
    }

    pub async fn check_has_service_account(&self) -> Result<(), Error> {
        let sa = self.security.get_service_account_group().await?;
        if !self.principal.has_group(&sa.id) {
            let admin = self.security.get_administrators_group().await?;
            if !self.principal.has_group(&admin.id) {
                return Err(Error::new("invalid permissions"));
            }
        }
        Ok(())
    }
}
