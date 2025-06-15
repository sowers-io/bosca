use crate::datastores::cache::cache::BoscaCache;
use crate::datastores::cache::manager::BoscaCacheManager;
use crate::models::security::group::Group;
use crate::models::security::principal::Principal;
use async_graphql::Error;
use uuid::Uuid;

const CACHE_SECURITY_PRINCIPAL_ID: &str = "security_principal_id";
const CACHE_SECURITY_PRINCIPAL_GROUP_ID: &str = "security_principal_group_id";
const CACHE_SECURITY_GROUP_ID: &str = "security_group_id";
const CACHE_SECURITY_GROUP_NAME: &str = "security_group_name";

#[derive(Clone, Debug)]
pub struct SecurityCache {
    principal_id: BoscaCache<Principal>,
    principal_group_ids: BoscaCache<Vec<Uuid>>,
    group_id: BoscaCache<Group>,
    group_name: BoscaCache<Group>,
}

impl SecurityCache {
    pub async fn new(cache: &mut BoscaCacheManager) -> Result<Self, Error> {
        Ok(Self {
            principal_id: cache.new_id_tiered_cache(CACHE_SECURITY_PRINCIPAL_ID).await?,
            principal_group_ids: cache.new_id_tiered_cache(CACHE_SECURITY_PRINCIPAL_GROUP_ID).await?,
            group_id: cache.new_id_tiered_cache(CACHE_SECURITY_GROUP_ID).await?,
            group_name: cache.new_string_tiered_cache(CACHE_SECURITY_GROUP_NAME).await?,
        })
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_principal_by_id(&self, id: &Uuid) -> Option<Principal> {
        self.principal_id.get(id).await
    }

    #[tracing::instrument(skip(self, principal))]
    pub async fn cache_principal(&self, principal: &Principal) {
        self.principal_id.set(&principal.id, principal).await;
    }

    #[tracing::instrument(skip(self, principal_id))]
    pub async fn evict_principal(&self, principal_id: &Uuid) {
        self.principal_id.remove(principal_id).await;
        self.principal_group_ids.remove(principal_id).await;
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_principal_group_ids(&self, id: &Uuid) -> Option<Vec<Uuid>> {
        self.principal_group_ids.get(id).await
    }

    #[tracing::instrument(skip(self, id, group_ids))]
    pub async fn cache_principal_group_ids(&self, id: &Uuid, group_ids: Vec<Uuid>) {
        self.principal_group_ids.set(id, &group_ids).await;
    }

    #[tracing::instrument(skip(self, name))]
    pub async fn get_group_by_name(&self, name: &str) -> Option<Group> {
        let name_lower = name.to_lowercase();
        self.group_name.get(&name_lower).await
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_group_by_id(&self, id: &Uuid) -> Option<Group> {
        self.group_id.get(id).await
    }

    #[tracing::instrument(skip(self, group))]
    pub async fn cache_group(&self, group: &Group) {
        let name_lower = group.name.to_lowercase();
        self.group_id.set(&group.id, group).await;
        self.group_name.set(&name_lower, group).await;
    }

    pub async fn evict_group(&self, group_id: &Uuid) {
        let Some(group): Option<Group> = self.group_id.get(group_id).await else {
            return;
        };
        let lower_name = group.name.to_lowercase();
        self.group_id.remove(group_id).await;
        self.group_name.remove(&lower_name).await;
    }
}
