use crate::datastores::cache::manager::BoscaCacheManager;
use crate::datastores::configurations::ConfigurationDataStore;
use crate::datastores::content::content::ContentDataStore;
use crate::datastores::content::workflow_schedules::WorkflowScheduleDataStore;
use crate::datastores::notifier::Notifier;
use crate::datastores::persisted_queries::PersistedQueriesDataStore;
use crate::datastores::profile::profile::ProfileDataStore;
use crate::datastores::profile::profile_bookmarks::ProfileBookmarksDataStore;
use crate::datastores::profile::profile_marks::ProfileMarksDataStore;
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
use crate::search::search::SearchClient;
use crate::security::authorization_extension::get_anonymous_principal;
use crate::workflow::queue::JobQueues;
use async_graphql::{Context, Error};
use bosca_database::build_pool;
use deadpool_postgres::Transaction;
use log::info;
use std::env;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct BoscaContext {
    pub content: ContentDataStore,
    pub profile: ProfileDataStore,
    pub profile_bookmarks: ProfileBookmarksDataStore,
    pub profile_marks: ProfileMarksDataStore,
    pub security: SecurityDataStore,
    pub security_oauth2: SecurityOAuth2,
    pub storage: ObjectStorage,
    pub workflow: WorkflowDataStore,
    pub workflow_schedule: WorkflowScheduleDataStore,
    pub queries: PersistedQueriesDataStore,
    pub configuration: ConfigurationDataStore,
    pub notifier: Arc<Notifier>,
    pub search: Arc<SearchClient>,
    pub principal: Principal,
    pub principal_groups: Vec<Uuid>,
    pub cache: BoscaCacheManager,
}

#[derive(Default)]
pub struct PermissionCheck {
    principal: Option<Principal>,
    groups: Option<Vec<Uuid>>,
    collection_id: Option<Uuid>,
    collection: Option<Collection>,
    supplementary_id: Option<Uuid>,
    metadata_id: Option<Uuid>,
    version: Option<i32>,
    metadata: Option<Metadata>,
    action: PermissionAction,
    supplementary: bool,
    content: bool,
    enable_advertised: bool,
}

impl PermissionCheck {
    pub fn new_with_metadata(metadata: Metadata, action: PermissionAction) -> Self {
        Self {
            metadata: Some(metadata),
            action,
            ..Default::default()
        }
    }
    pub fn new_with_metadata_supplementary(metadata: Metadata, action: PermissionAction) -> Self {
        Self {
            metadata: Some(metadata),
            supplementary: true,
            action,
            ..Default::default()
        }
    }
    pub fn new_with_metadata_content(metadata: Metadata, action: PermissionAction) -> Self {
        Self {
            metadata: Some(metadata),
            action,
            content: true,
            ..Default::default()
        }
    }
    pub fn new_with_metadata_id(metadata_id: Uuid, action: PermissionAction) -> Self {
        Self {
            metadata_id: Some(metadata_id),
            action,
            ..Default::default()
        }
    }
    pub fn new_with_metadata_id_with_version(
        metadata_id: Uuid,
        version: i32,
        action: PermissionAction,
    ) -> Self {
        Self {
            metadata_id: Some(metadata_id),
            version: Some(version),
            action,
            ..Default::default()
        }
    }
    pub fn new_with_metadata_supplementary_id(
        supplementary_id: Uuid,
        action: PermissionAction,
    ) -> Self {
        Self {
            supplementary_id: Some(supplementary_id),
            action,
            supplementary: true,
            ..Default::default()
        }
    }
    pub fn new_with_metadata_advertised(metadata: Metadata, action: PermissionAction) -> Self {
        Self {
            metadata: Some(metadata),
            action,
            enable_advertised: true,
            ..Default::default()
        }
    }
    pub fn new_with_metadata_id_advertised(metadata_id: Uuid, action: PermissionAction) -> Self {
        Self {
            metadata_id: Some(metadata_id),
            action,
            enable_advertised: true,
            ..Default::default()
        }
    }
    pub fn new_with_principal_and_metadata_id(
        principal: Principal,
        groups: Vec<Uuid>,
        metadata_id: Uuid,
        action: PermissionAction,
    ) -> Self {
        Self {
            metadata_id: Some(metadata_id),
            principal: Some(principal),
            groups: Some(groups),
            action,
            ..Default::default()
        }
    }
    pub fn new_with_principal_and_metadata_supplementary_id(
        principal: Principal,
        groups: Vec<Uuid>,
        metadata_id: Uuid,
        supplementary_id: Uuid,
        action: PermissionAction,
    ) -> Self {
        Self {
            metadata_id: Some(metadata_id),
            supplementary_id: Some(supplementary_id),
            principal: Some(principal),
            groups: Some(groups),
            supplementary: true,
            action,
            ..Default::default()
        }
    }
    pub fn new_with_principal_and_metadata_id_with_version(
        principal: Principal,
        groups: Vec<Uuid>,
        metadata_id: Uuid,
        version: i32,
        action: PermissionAction,
    ) -> Self {
        Self {
            metadata_id: Some(metadata_id),
            version: Some(version),
            principal: Some(principal),
            groups: Some(groups),
            action,
            ..Default::default()
        }
    }
    pub fn new_with_principal_and_collection_supplementary_id(
        principal: Principal,
        groups: Vec<Uuid>,
        supplementary_id: Uuid,
        action: PermissionAction,
    ) -> Self {
        Self {
            supplementary_id: Some(supplementary_id),
            principal: Some(principal),
            groups: Some(groups),
            action,
            ..Default::default()
        }
    }
    pub fn new_with_collection(collection: Collection, action: PermissionAction) -> Self {
        Self {
            collection: Some(collection),
            action,
            ..Default::default()
        }
    }
    pub fn new_with_collection_id(collection_id: Uuid, action: PermissionAction) -> Self {
        Self {
            collection_id: Some(collection_id),
            action,
            ..Default::default()
        }
    }
    pub fn new_with_collection_id_supplementary(
        collection_id: Uuid,
        action: PermissionAction,
    ) -> Self {
        Self {
            collection_id: Some(collection_id),
            supplementary: true,
            action,
            ..Default::default()
        }
    }
    pub fn new_with_collection_supplementary_id(
        supplementary_id: Uuid,
        action: PermissionAction,
    ) -> Self {
        Self {
            supplementary_id: Some(supplementary_id),
            action,
            supplementary: true,
            ..Default::default()
        }
    }
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
            }
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
            profile_bookmarks: ProfileBookmarksDataStore::new(bosca_pool.clone()),
            profile_marks: ProfileMarksDataStore::new(bosca_pool.clone()),
            queries: PersistedQueriesDataStore::new(bosca_pool.clone()).await,
            content: ContentDataStore::new(
                bosca_pool,
                &mut cache,
                Arc::clone(&notifier),
                redis_jobs_queue_client.clone(),
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

    #[tracing::instrument(skip(self, groups))]
    async fn check_principal_groups(&self, groups: &[Uuid]) -> Result<(), Error> {
        let sa = self.security.get_service_account_group().await?;
        if !groups.contains(&sa.id) {
            let admin = self.security.get_administrators_group().await?;
            if !groups.contains(&admin.id) {
                return Err(Error::new("invalid permissions"));
            }
        }
        Ok(())
    }

    #[tracing::instrument(skip(self, check))]
    pub async fn metadata_permission_check(
        &self,
        check: PermissionCheck,
    ) -> Result<Metadata, Error> {
        let metadata = if let Some(metadata) = check.metadata {
            metadata
        } else {
            if let Some(metadata_id) = check.metadata_id {
                if let Some(metadata) = self.content.metadata.get(&metadata_id).await? {
                    if let Some(version) = check.version {
                        if metadata.version == version {
                            metadata
                        } else {
                            if let Some(metadata) = self
                                .content
                                .metadata
                                .get_by_version(&metadata.id, version)
                                .await?
                            {
                                metadata
                            } else {
                                return Err(Error::new(format!(
                                    "metadata not found: {metadata_id}"
                                )));
                            }
                        }
                    } else {
                        metadata
                    }
                } else {
                    return Err(Error::new(format!("metadata not found: {metadata_id}")));
                }
            } else {
                return Err(Error::new("invalid permission check"));
            }
        };
        let principal = if let Some(ref principal) = check.principal {
            principal
        } else {
            &self.principal
        };
        let groups = if let Some(ref groups) = check.groups {
            groups
        } else {
            &self.principal_groups
        };
        if check.supplementary {
            if !self
                .content
                .metadata_permissions
                .has_supplementary_permission(&metadata, principal, groups, check.action)
                .await?
            {
                self.check_principal_groups(groups).await?;
            }
        } else if check.content {
            if !self
                .content
                .metadata_permissions
                .has_metadata_content_permission(&metadata, principal, groups, check.action)
                .await?
            {
                self.check_principal_groups(groups).await?;
            }
        } else {
            if !self
                .content
                .metadata_permissions
                .has(
                    &metadata,
                    principal,
                    groups,
                    check.action,
                    check.enable_advertised,
                )
                .await?
            {
                self.check_principal_groups(groups).await?;
            }
        }
        Ok(metadata)
    }

    #[tracing::instrument(skip(self, check))]
    pub async fn metadata_supplementary_permission_check(
        &self,
        check: PermissionCheck,
    ) -> Result<(Metadata, MetadataSupplementary), Error> {
        let supplementary = if let Some(supplementary_id) = check.supplementary_id {
            if let Some(supplementary) = self
                .content
                .metadata_supplementary
                .get_supplementary(&supplementary_id)
                .await?
            {
                supplementary
            } else {
                return Err(Error::new(format!(
                    "supplementary not found: {supplementary_id}"
                )));
            }
        } else {
            return Err(Error::new("invalid permission check"));
        };
        let principal = if let Some(ref principal) = check.principal {
            principal
        } else {
            &self.principal
        };
        let groups = if let Some(ref groups) = check.groups {
            groups
        } else {
            &self.principal_groups
        };
        if let Some(metadata) = self
            .content
            .metadata
            .get(&supplementary.metadata_id)
            .await?
        {
            if !self
                .content
                .metadata_permissions
                .has_supplementary_permission(&metadata, principal, groups, check.action)
                .await?
            {
                self.check_principal_groups(groups).await?;
            }
            Ok((metadata, supplementary))
        } else {
            Err(Error::new(format!(
                "metadata not found: {}",
                check.supplementary_id.unwrap()
            )))
        }
    }

    #[tracing::instrument(skip(self, check))]
    pub async fn collection_permission_check(
        &self,
        check: PermissionCheck,
    ) -> Result<Collection, Error> {
        let collection = if let Some(collection) = check.collection {
            collection
        } else {
            if let Some(collection_id) = check.collection_id {
                if let Some(collection) = self.content.collections.get(&collection_id).await? {
                    collection
                } else {
                    return Err(Error::new(format!("collection not found: {collection_id}")));
                }
            } else {
                return Err(Error::new("invalid permission check"));
            }
        };
        let principal = if let Some(ref principal) = check.principal {
            principal
        } else {
            &self.principal
        };
        let groups = if let Some(ref groups) = check.groups {
            groups
        } else {
            &self.principal_groups
        };
        if check.supplementary {
            if !self
                .content
                .collection_permissions
                .has_supplementary_permission(&collection, principal, groups, check.action)
                .await?
            {
                self.check_principal_groups(groups).await?;
            }
        } else {
            if !self
                .content
                .collection_permissions
                .has(&collection, principal, groups, check.action)
                .await?
            {
                self.check_principal_groups(groups).await?;
            }
        }
        Ok(collection)
    }

    #[tracing::instrument(skip(self, check))]
    pub async fn collection_supplementary_permission_check(
        &self,
        check: PermissionCheck,
    ) -> Result<(Collection, CollectionSupplementary), Error> {
        let supplementary = if let Some(supplementary_id) = check.supplementary_id {
            if let Some(supplementary) = self
                .content
                .collection_supplementary
                .get_supplementary(&supplementary_id)
                .await?
            {
                supplementary
            } else {
                return Err(Error::new(format!(
                    "supplementary not found: {supplementary_id}"
                )));
            }
        } else {
            return Err(Error::new("invalid permission check"));
        };
        let principal = if let Some(ref principal) = check.principal {
            principal
        } else {
            &self.principal
        };
        let groups = if let Some(ref groups) = check.groups {
            groups
        } else {
            &self.principal_groups
        };
        if let Some(collection) = self
            .content
            .collections
            .get(&supplementary.collection_id)
            .await?
        {
            if !self
                .content
                .collection_permissions
                .has_supplementary_permission(&collection, principal, groups, check.action)
                .await?
            {
                self.check_principal_groups(groups).await?;
            }
            Ok((collection, supplementary))
        } else {
            Err(Error::new(format!(
                "collection not found: {}",
                check.supplementary_id.unwrap()
            )))
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
                    self.check_principal_groups(&self.principal_groups).await?;
                }
                Ok(collection)
            }
            None => Err(Error::new(format!("collection not found: {id}"))),
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
                    self.check_principal_groups(&self.principal_groups).await?;
                }
                Ok(profile)
            }
            None => Err(Error::new(format!("profile not found: {id}"))),
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
            self.check_principal_groups(&self.principal_groups).await?;
        }
        Ok(())
    }

    pub fn get<'a>(ctx: &Context<'a>) -> Result<&'a BoscaContext, Error> {
        ctx.data::<BoscaContext>()
    }
}
