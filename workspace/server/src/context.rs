use std::sync::Arc;
use async_graphql::Error;
use deadpool_postgres::Transaction;
use meilisearch_sdk::client::Client;
use uuid::Uuid;
use crate::datastores::content::content::ContentDataStore;
use crate::datastores::persisted_queries::PersistedQueriesDataStore;
use crate::datastores::security::SecurityDataStore;
use crate::datastores::workflow::WorkflowDataStore;
use crate::graphql::content::storage::ObjectStorage;
use crate::models::content::collection::Collection;
use crate::models::content::metadata::Metadata;
use crate::models::security::permission::PermissionAction;
use crate::models::security::principal::Principal;

use crate::datastores::notifier::Notifier;
use crate::datastores::profile::ProfileDataStore;
use crate::models::profiles::profile::Profile;
use crate::models::profiles::profile_visibility::ProfileVisibility;

#[derive(Clone)]
pub struct BoscaContext {
    pub content: ContentDataStore,
    pub profile: ProfileDataStore,
    pub security: SecurityDataStore,
    pub storage: ObjectStorage,
    pub workflow: WorkflowDataStore,
    pub queries: PersistedQueriesDataStore,
    pub notifier: Arc<Notifier>,
    pub search: Arc<Client>,
    pub principal: Principal,
}

impl BoscaContext {
    pub async fn check_metadata_action_principal(&self, principal: &Principal, id: &Uuid, action: PermissionAction) -> Result<Metadata, Error> {
        match self.content.metadata.get(id).await? {
            Some(metadata) => {
                if !self.content
                    .metadata_permissions
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
        match self.content.metadata.get(id).await? {
            Some(metadata) => {
                if !self.content
                    .metadata_permissions
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
        match self.content.metadata.get_by_version(id, version).await? {
            Some(metadata) => {
                if !self.content
                    .metadata_permissions
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
        match self.content.collections.get(id).await? {
            Some(collection) => {
                if !self.content
                    .collection_permissions
                    .has_txn(txn, &collection, &self.principal, action)
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
        match self.content.collections.get(id).await? {
            Some(collection) => {
                if !self.content.collection_permissions
                    .has(&collection, &self.principal, action)
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

    pub async fn check_profile_action(&self, id: &Uuid, action: PermissionAction) -> Result<Profile, Error> {
        match self.profile.get_by_id(id).await? {
            Some(profile) => {
                if profile.principal == self.principal.id {
                    return Ok(profile)
                }
                if action == PermissionAction::View && profile.visibility != ProfileVisibility::Public {
                    let admin = self.security.get_administrators_group().await?;
                    if !self.principal.has_group(&admin.id) {
                        return Err(Error::new("invalid permissions"));
                    }
                }
                Ok(profile)
            }
            None => Err(Error::new(format!(
                "profile not found: {}",
                id
            ))),
        }
    }

    pub async fn check_has_admin_account(&self) -> Result<(), Error> {
        if !self.has_admin_account().await? {
            return Err(Error::new("invalid permissions"));
        }
        Ok(())
    }

    pub async fn has_admin_account(&self) -> Result<bool, Error> {
        let admin = self.security.get_administrators_group().await?;
        Ok(self.principal.has_group(&admin.id))
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
