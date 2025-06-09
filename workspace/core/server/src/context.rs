use crate::datastores::cache::manager::BoscaCacheManager;
use crate::datastores::configurations::ConfigurationDataStore;
use crate::datastores::content::content::ContentDataStore;
use crate::datastores::content::workflow_schedules::WorkflowScheduleDataStore;
use crate::datastores::notifier::Notifier;
use crate::datastores::persisted_queries::PersistedQueriesDataStore;
use crate::datastores::profile::ProfileDataStore;
use crate::datastores::security::SecurityDataStore;
use crate::datastores::security_oauth2::SecurityOAuth2;
use crate::datastores::workflow::workflow::WorkflowDataStore;
use crate::graphql::content::storage::ObjectStorage;
use crate::initialization::cache::new_cache_client;
use crate::initialization::jwt::new_jwt;
use crate::initialization::object_storage::new_object_storage;
use crate::initialization::redis::new_redis_client;
use crate::initialization::search::new_search_client;
use crate::models::content::collection::Collection;
use crate::models::content::collection_supplementary::CollectionSupplementary;
use crate::models::content::metadata::Metadata;
use crate::models::content::metadata_supplementary::MetadataSupplementary;
use crate::models::profiles::profile::Profile;
use crate::models::profiles::profile_visibility::ProfileVisibility;
use crate::models::security::permission::PermissionAction;
use crate::models::security::principal::Principal;
use crate::security::authorization_extension::get_anonymous_principal;
use crate::workflow::queue::JobQueues;
use async_graphql::{Context, Error};
use bosca_database::build_pool;
use deadpool_postgres::Transaction;
use log::info;
use meilisearch_sdk::client::Client;
use std::env;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct BoscaContext {
    pub content: ContentDataStore,
    pub profile: ProfileDataStore,
    pub security: SecurityDataStore,
    pub security_oauth2: SecurityOAuth2,
    pub storage: ObjectStorage,
    pub workflow: WorkflowDataStore,
    pub workflow_schedule: WorkflowScheduleDataStore,
    pub queries: PersistedQueriesDataStore,
    pub configuration: ConfigurationDataStore,
    pub notifier: Arc<Notifier>,
    pub search: Arc<Client>,
    pub principal: Principal,
    pub principal_groups: Vec<Uuid>,
    pub cache: BoscaCacheManager,
}

impl BoscaContext {
    pub async fn new() -> Result<BoscaContext, Error> {
        info!("Connecting to Database");
        let bosca_pool = build_pool("DATABASE_URL")?;
        let url_secret_key = match env::var("URL_SECRET_KEY") {
            Ok(url_secret_key) => url_secret_key,
            _ => {
                println!(
                    "Environment variable URL_SECRET_KEY could not be read, generating a random value"
                );
                Uuid::new_v4().to_string()
            }
        };
        let auto_verify = match env::var("AUTO_VERIFY_SIGNUP") {
            Ok(auto_verify) => {
                let v = auto_verify.to_lowercase() == "true";
                if v {
                    println!("Environment variable AUTO_VERIFY_SIGNUP is true");
                }
                v
            },
            _ => {
                println!(
                    "Environment variable AUTO_VERIFY_SIGNUP could not be read, falling back to false"
                );
                false
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
        info!("Connecting to Redis");
        let redis_jobs_queue_client = new_redis_client("REDIS_JOBS_QUEUE").await?;
        let redis_notifier_client = new_redis_client("REDIS_NOTIFIER_PUBSUB").await?;
        let notifier = Arc::new(Notifier::new(redis_notifier_client.clone()));
        let jobs = JobQueues::new(
            bosca_pool.clone(),
            redis_jobs_queue_client.clone(),
            Arc::clone(&notifier),
        );
        info!("Connecting to Search");
        let search = new_search_client()?;
        info!("Connecting to Cache");
        let cache_client = new_cache_client().await?;
        info!("Building Context");
        let mut cache = BoscaCacheManager::new(cache_client.clone());
        let configuration = ConfigurationDataStore::new(
            bosca_pool.clone(),
            configuration_secret_key,
            cache_client.clone(),
            Arc::clone(&notifier),
        );
        let ctx = BoscaContext {
            security: SecurityDataStore::new(
                configuration.clone(),
                &mut cache,
                bosca_pool.clone(),
                new_jwt(),
                url_secret_key,
                auto_verify,
            )
            .await?,
            security_oauth2: SecurityOAuth2::new()?,
            workflow: WorkflowDataStore::new(
                bosca_pool.clone(),
                &mut cache,
                jobs.clone(),
                Arc::clone(&notifier),
            )
            .await?,
            workflow_schedule: WorkflowScheduleDataStore::new(
                bosca_pool.clone(),
                Arc::clone(&notifier),
            ),
            configuration,
            profile: ProfileDataStore::new(bosca_pool.clone()),
            queries: PersistedQueriesDataStore::new(bosca_pool.clone()).await,
            content: ContentDataStore::new(
                bosca_pool,
                &mut cache,
                Arc::clone(&notifier),
                redis_jobs_queue_client.clone()
            )
            .await?,
            notifier,
            search,
            storage: new_object_storage(),
            principal: get_anonymous_principal(),
            principal_groups: vec![],
            cache,
        };
        info!("Context built");
        Ok(ctx)
    }

    #[tracing::instrument(skip(self, principal, id, action))]
    pub async fn check_metadata_action_principal(
        &self,
        principal: &Principal,
        groups: &Vec<Uuid>,
        id: &Uuid,
        action: PermissionAction,
    ) -> Result<Metadata, Error> {
        match self.content.metadata.get(id).await? {
            Some(metadata) => {
                if !self
                    .content
                    .metadata_permissions
                    .has(&metadata, principal, groups, action)
                    .await?
                {
                    let admin = self.security.get_administrators_group().await?;
                    if !self.principal_groups.contains(&admin.id) {
                        return Err(Error::new("invalid permissions"));
                    }
                }
                Ok(metadata)
            }
            None => Err(Error::new(format!("metadata not found: {}", id))),
        }
    }

    #[tracing::instrument(skip(self, principal, id, action))]
    pub async fn check_metadata_content_action_principal(
        &self,
        principal: &Principal,
        groups: &Vec<Uuid>,
        id: &Uuid,
        action: PermissionAction,
    ) -> Result<Metadata, Error> {
        match self.content.metadata.get(id).await? {
            Some(metadata) => {
                if !self
                    .content
                    .metadata_permissions
                    .has_metadata_content_permission(&metadata, principal, groups, action)
                    .await?
                {
                    let admin = self.security.get_administrators_group().await?;
                    if !self.principal_groups.contains(&admin.id) {
                        return Err(Error::new("invalid permissions"));
                    }
                }
                Ok(metadata)
            }
            None => Err(Error::new(format!("metadata not found: {}", id))),
        }
    }

    #[tracing::instrument(skip(self, metadata, action))]
    pub async fn check_metadata_supplementary_action(
        &self,
        metadata: &Metadata,
        action: PermissionAction,
    ) -> Result<(), Error> {
        if !self
            .content
            .metadata_permissions
            .has_supplementary_permission(metadata, &self.principal, &self.principal_groups, action)
            .await?
        {
            let admin = self.security.get_administrators_group().await?;
            if !self.principal_groups.contains(&admin.id) {
                return Err(Error::new("invalid permissions"));
            }
        }
        Ok(())
    }

    #[tracing::instrument(skip(self, principal, supplementary_id, action))]
    pub async fn check_metadata_supplementary_action_principal(
        &self,
        principal: &Principal,
        groups: &Vec<Uuid>,
        supplementary_id: &Uuid,
        action: PermissionAction,
    ) -> Result<(Metadata, MetadataSupplementary), Error> {
        match self
            .content
            .metadata_supplementary
            .get_supplementary(supplementary_id)
            .await?
        {
            Some(supplementary) => {
                let metadata = self
                    .check_metadata_action(&supplementary.metadata_id, PermissionAction::View)
                    .await?;
                if !self
                    .content
                    .metadata_permissions
                    .has_supplementary_permission(&metadata, principal, groups, action)
                    .await?
                {
                    let admin = self.security.get_administrators_group().await?;
                    if !groups.contains(&admin.id) {
                        return Err(Error::new("invalid permissions"));
                    }
                }
                Ok((metadata, supplementary))
            }
            None => Err(Error::new(format!(
                "supplementary not found: {}",
                supplementary_id
            ))),
        }
    }

    #[tracing::instrument(skip(self, principal, supplementary_id, action))]
    pub async fn check_collection_supplementary_action_principal(
        &self,
        principal: &Principal,
        groups: &Vec<Uuid>,
        supplementary_id: &Uuid,
        action: PermissionAction,
    ) -> Result<(Collection, CollectionSupplementary), Error> {
        let Some(supplementary) = self
            .content
            .collection_supplementary
            .get_supplementary(supplementary_id)
            .await?
        else {
            return Err(Error::new(format!(
                "collection supplementary not found: {}",
                supplementary_id
            )));
        };
        let Some(collection) = self
            .content
            .collections
            .get(&supplementary.collection_id)
            .await?
        else {
            return Err(Error::new(format!(
                "collection not found: {}",
                supplementary_id
            )));
        };
        if !self
            .content
            .collection_permissions
            .has_supplementary_permission(&collection, principal, groups, action)
            .await?
        {
            let admin = self.security.get_administrators_group().await?;
            if !groups.contains(&admin.id) {
                return Err(Error::new("invalid permissions"));
            }
        }
        Ok((collection, supplementary))
    }

    #[tracing::instrument(skip(self, collection, action))]
    pub async fn check_collection_supplementary_action(
        &self,
        collection: &Collection,
        action: PermissionAction,
    ) -> Result<(), Error> {
        if !self
            .content
            .collection_permissions
            .has_supplementary_permission(
                collection,
                &self.principal,
                &self.principal_groups,
                action,
            )
            .await?
        {
            let admin = self.security.get_administrators_group().await?;
            if !self.principal_groups.contains(&admin.id) {
                return Err(Error::new("invalid permissions"));
            }
        }
        Ok(())
    }

    #[tracing::instrument(skip(self, id, action))]
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
                    .has(&metadata, &self.principal, &self.principal_groups, action)
                    .await?
                {
                    let admin = self.security.get_administrators_group().await?;
                    if !self.principal_groups.contains(&admin.id) {
                        return Err(Error::new("invalid permissions"));
                    }
                }
                Ok(metadata)
            }
            None => Err(Error::new(format!("metadata not found: {}", id))),
        }
    }

    #[tracing::instrument(skip(self, id, version, action))]
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
                    .has_metadata_version_permission(
                        &metadata,
                        &self.principal,
                        &self.principal_groups,
                        action,
                    )
                    .await?
                {
                    let admin = self.security.get_administrators_group().await?;
                    if !self.principal_groups.contains(&admin.id) {
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

    #[tracing::instrument(skip(self, id, version, action))]
    pub async fn check_metadata_version_principal_action(
        &self,
        principal: &Principal,
        groups: &Vec<Uuid>,
        id: &Uuid,
        version: i32,
        action: PermissionAction,
    ) -> Result<Metadata, Error> {
        let metadata = self.check_metadata_action_principal(principal, groups, id, action).await?;
        if metadata.version == version {
            return Ok(metadata);
        }
        match self.content.metadata.get_by_version(id, version).await? {
            Some(metadata) => {
                if !self
                    .content
                    .metadata_permissions
                    .has_metadata_version_permission(
                        &metadata,
                        &self.principal,
                        &self.principal_groups,
                        action,
                    )
                    .await?
                {
                    let admin = self.security.get_administrators_group().await?;
                    if !self.principal_groups.contains(&admin.id) {
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

    #[tracing::instrument(skip(self, txn, id, action))]
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
                    .has_txn(
                        txn,
                        &collection,
                        &self.principal,
                        &self.principal_groups,
                        action,
                    )
                    .await?
                {
                    let admin = self.security.get_administrators_group().await?;
                    if !self.principal_groups.contains(&admin.id) {
                        return Err(Error::new("invalid permissions"));
                    }
                }
                Ok(collection)
            }
            None => Err(Error::new(format!("collection not found: {}", id))),
        }
    }

    #[tracing::instrument(skip(self, id, action))]
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
                    .has(&collection, &self.principal, &self.principal_groups, action)
                    .await?
                {
                    let admin = self.security.get_administrators_group().await?;
                    if !self.principal_groups.contains(&admin.id) {
                        return Err(Error::new("invalid permissions"));
                    }
                }
                Ok(collection)
            }
            None => Err(Error::new(format!("collection not found: {}", id))),
        }
    }

    #[tracing::instrument(skip(self, id, action))]
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
                    if !self.principal_groups.contains(&admin.id) {
                        return Err(Error::new("invalid permissions"));
                    }
                }
                Ok(profile)
            }
            None => Err(Error::new(format!("profile not found: {}", id))),
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn check_has_admin_account(&self) -> Result<(), Error> {
        if !self.has_admin_account().await? {
            return Err(Error::new("invalid permissions"));
        }
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn has_admin_account(&self) -> Result<bool, Error> {
        let admin = self.security.get_administrators_group().await?;
        Ok(self.principal_groups.contains(&admin.id))
    }

    #[tracing::instrument(skip(self))]
    pub async fn has_service_account(&self) -> Result<bool, Error> {
        let sa = self.security.get_service_account_group().await?;
        if !self.principal_groups.contains(&sa.id) {
            self.has_admin_account().await
        } else {
            Ok(true)
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn check_has_service_account(&self) -> Result<(), Error> {
        let sa = self.security.get_service_account_group().await?;
        if !self.principal_groups.contains(&sa.id) {
            let admin = self.security.get_administrators_group().await?;
            if !self.principal_groups.contains(&admin.id) {
                return Err(Error::new("invalid permissions"));
            }
        }
        Ok(())
    }

    pub fn get<'a>(ctx: &Context<'a>) -> Result<&'a BoscaContext, Error> {
        ctx.data::<BoscaContext>()
    }
}
