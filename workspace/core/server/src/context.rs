use crate::datastores::configurations::ConfigurationDataStore;
use crate::datastores::content::content::ContentDataStore;
use crate::datastores::content::workflow_schedules::WorkflowScheduleDataStore;
use crate::datastores::persisted_queries::PersistedQueriesDataStore;
use crate::datastores::security::SecurityDataStore;
use crate::datastores::workflow::WorkflowDataStore;
use crate::graphql::content::storage::ObjectStorage;
use crate::models::content::collection::Collection;
use crate::models::content::metadata::Metadata;
use crate::models::security::permission::PermissionAction;
use crate::models::security::principal::Principal;
use async_graphql::Error;
use bosca_database::build_pool;
use deadpool_postgres::Transaction;
use meilisearch_sdk::client::Client;
use std::env;
use std::sync::Arc;
use uuid::Uuid;

use crate::datastores::notifier::Notifier;
use crate::datastores::profile::ProfileDataStore;
use crate::initialization::jwt::new_jwt;
use crate::initialization::object_storage::new_object_storage;
use crate::initialization::redis::new_redis_client;
use crate::initialization::search::new_search_client;
use crate::models::profiles::profile::Profile;
use crate::models::profiles::profile_visibility::ProfileVisibility;
use crate::security::authorization_extension::get_anonymous_principal;
use crate::workflow::queue::JobQueues;

#[derive(Clone)]
pub struct BoscaContext {
    pub content: ContentDataStore,
    pub profile: ProfileDataStore,
    pub security: SecurityDataStore,
    pub storage: ObjectStorage,
    pub workflow: WorkflowDataStore,
    pub workflow_schedule: WorkflowScheduleDataStore,
    pub queries: PersistedQueriesDataStore,
    pub configuration: ConfigurationDataStore,
    pub notifier: Arc<Notifier>,
    pub search: Arc<Client>,
    pub principal: Principal,
}

impl BoscaContext {
    pub async fn new() -> Result<BoscaContext, Error> {
        let bosca_pool = build_pool("DATABASE_URL");
        let url_secret_key = match env::var("URL_SECRET_KEY") {
            Ok(url_secret_key) => url_secret_key,
            _ => {
                println!(
                    "Environment variable URL_SECRET_KEY could not be read, generating a random value"
                );
                Uuid::new_v4().to_string()
            }
        };
        let configuration_secret_key = match env::var("CONFIGURATION_SECRET_KEY") {
            Ok(url_secret_key) => url_secret_key,
            _ => {
                return Err(Error::new(
                    "Environment variable CONFIGURATION_SECRET_KEY could not be read",
                ))
            }
        };
        let redis_jobs_queue_client = new_redis_client("REDIS_JOBS_QUEUE_URL").await?;
        let redis_notifier_client = new_redis_client("REDIS_NOTIFIER_PUBSUB_URL").await?;
        let notifier = Arc::new(Notifier::new(redis_notifier_client.clone()));
        let jobs = JobQueues::new(
            Arc::clone(&bosca_pool),
            redis_jobs_queue_client.clone(),
            Arc::clone(&notifier),
        );
        let search = new_search_client()?;
        Ok(BoscaContext {
            security: SecurityDataStore::new(Arc::clone(&bosca_pool), new_jwt(), url_secret_key),
            workflow: WorkflowDataStore::new(
                Arc::clone(&bosca_pool),
                jobs.clone(),
                Arc::clone(&notifier),
                Arc::clone(&search),
            ),
            workflow_schedule: WorkflowScheduleDataStore::new(
                Arc::clone(&bosca_pool),
                Arc::clone(&notifier),
            ),
            configuration: ConfigurationDataStore::new(
                Arc::clone(&bosca_pool),
                configuration_secret_key,
                Arc::clone(&notifier),
            ),
            profile: ProfileDataStore::new(Arc::clone(&bosca_pool)),
            queries: PersistedQueriesDataStore::new(Arc::clone(&bosca_pool)).await,
            content: ContentDataStore::new(bosca_pool, Arc::clone(&notifier)),
            notifier,
            search,
            storage: new_object_storage(),
            principal: get_anonymous_principal(),
        })
    }

    pub async fn check_metadata_action_principal(
        &self,
        principal: &Principal,
        id: &Uuid,
        action: PermissionAction,
    ) -> Result<Metadata, Error> {
        match self.content.metadata.get(id).await? {
            Some(metadata) => {
                if !self
                    .content
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
            None => Err(Error::new(format!("metadata not found: {}", id))),
        }
    }

    pub async fn check_metadata_content_action_principal(
        &self,
        principal: &Principal,
        id: &Uuid,
        action: PermissionAction,
    ) -> Result<Metadata, Error> {
        match self.content.metadata.get(id).await? {
            Some(metadata) => {
                if !self
                    .content
                    .metadata_permissions
                    .has_metadata_content_permission(&metadata, principal, action)
                    .await?
                {
                    let admin = self.security.get_administrators_group().await?;
                    if !self.principal.has_group(&admin.id) {
                        return Err(Error::new("invalid permissions"));
                    }
                }
                Ok(metadata)
            }
            None => Err(Error::new(format!("metadata not found: {}", id))),
        }
    }

    pub async fn check_metadata_action(
        &self,
        id: &Uuid,
        action: PermissionAction,
    ) -> Result<Metadata, Error> {
        match self.content.metadata.get(id).await? {
            Some(metadata) => {
                if !self
                    .content
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
            None => Err(Error::new(format!("metadata not found: {}", id))),
        }
    }

    pub async fn check_metadata_version_action(
        &self,
        id: &Uuid,
        version: i32,
        action: PermissionAction,
    ) -> Result<Metadata, Error> {
        let metadata = self.check_metadata_action(id, action).await?;
        if metadata.version == version {
            return Ok(metadata);
        }
        match self.content.metadata.get_by_version(id, version).await? {
            Some(metadata) => {
                if !self
                    .content
                    .metadata_permissions
                    .has_metadata_version_permission(&metadata, &self.principal, action)
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

    pub async fn check_collection_action_txn(
        &self,
        txn: &Transaction<'_>,
        id: &Uuid,
        action: PermissionAction,
    ) -> Result<Collection, Error> {
        match self.content.collections.get(id).await? {
            Some(collection) => {
                if !self
                    .content
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
            None => Err(Error::new(format!("collection not found: {}", id))),
        }
    }

    pub async fn check_collection_action(
        &self,
        id: &Uuid,
        action: PermissionAction,
    ) -> Result<Collection, Error> {
        match self.content.collections.get(id).await? {
            Some(collection) => {
                if !self
                    .content
                    .collection_permissions
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
            None => Err(Error::new(format!("collection not found: {}", id))),
        }
    }

    pub async fn check_profile_action(
        &self,
        id: &Uuid,
        action: PermissionAction,
    ) -> Result<Profile, Error> {
        match self.profile.get_by_id(id).await? {
            Some(profile) => {
                if profile.principal == Some(self.principal.id) {
                    return Ok(profile);
                }
                if action == PermissionAction::View
                    && profile.visibility != ProfileVisibility::Public
                {
                    let admin = self.security.get_administrators_group().await?;
                    if !self.principal.has_group(&admin.id) {
                        return Err(Error::new("invalid permissions"));
                    }
                }
                Ok(profile)
            }
            None => Err(Error::new(format!("profile not found: {}", id))),
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
