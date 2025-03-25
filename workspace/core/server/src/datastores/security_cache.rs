use crate::datastores::cache::manager::BoscaCacheManager;
use crate::models::security::group::Group;
use crate::models::security::principal::Principal;
use uuid::Uuid;
use crate::datastores::cache::cache::{BoscaCache, BoscaCacheInterface};

const CACHE_SECURITY_PRINCIPAL_ID: &str = "security::principal::id";
const CACHE_SECURITY_GROUP_ID: &str = "security::group::id";
const CACHE_SECURITY_GROUP_NAME: &str = "security::group::name";

#[derive(Clone)]
pub struct SecurityCache {
    principal_id: BoscaCache<Uuid, Principal>,
    group_id: BoscaCache<Uuid, Group>,
    group_name: BoscaCache<String, Group>,
}

impl SecurityCache {
    pub async fn new(cache: &mut BoscaCacheManager) -> Self {
        Self {
            principal_id: cache.new_cache(CACHE_SECURITY_PRINCIPAL_ID, 5000).await,
            group_id: cache.new_cache(CACHE_SECURITY_GROUP_ID, 5000).await,
            group_name: cache.new_cache(CACHE_SECURITY_GROUP_NAME, 5000).await,
        }
    }

    pub async fn get_principal_by_id(&self, id: &Uuid) -> Option<Principal> {
        self.principal_id.get(id).await
    }

    pub async fn cache_principal(&self, principal: &Principal) {
        self.principal_id.set(&principal.id, principal).await;
    }

    pub async fn evict_principal(&self, principal_id: &Uuid) {
        self.principal_id.remove(principal_id).await;
    }

    pub async fn get_group_by_name(&self, name: &str) -> Option<Group> {
        let name_lower = name.to_lowercase();
        self.group_name.get(&name_lower).await
    }

    pub async fn get_group_by_id(&self, id: &Uuid) -> Option<Group> {
        self.group_id.get(id).await
    }

    pub async fn cache_group(&self, group: &Group) {
        let name_lower = group.name.to_lowercase();
        self.group_id.set(&group.id, group).await;
        self.group_name.set(&name_lower, group).await;
    }

    // pub async fn evict_group(&self, group_id: &Uuid) {
    //     let Some(group): Option<Group> = self.group_id.get(group_id).await else {
    //         return;
    //     };
    //     let lower_name = group.name.to_lowercase();
    //     self.group_id.remove(group_id).await;
    //     self.group_name.remove(&lower_name).await;
    // }
}
