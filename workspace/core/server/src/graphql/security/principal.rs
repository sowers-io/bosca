use crate::graphql::security::group::GroupObject;
use crate::models::security::principal::Principal;
use async_graphql::Object;

pub struct PrincipalObject {
    principal: Principal,
}

impl PrincipalObject {
    pub fn new(principal: Principal) -> Self {
        Self { principal }
    }
}

#[Object(name = "Principal")]
impl PrincipalObject {
    async fn id(&self) -> String {
        self.principal.id.to_string()
    }

    async fn verified(&self) -> bool {
        self.principal.verified
    }

    async fn groups(&self) -> Vec<GroupObject> {
        match &self.principal.get_groups() {
            Some(groups) => groups.iter().map(|g| GroupObject::new(g.clone())).collect(),
            None => Vec::new(),
        }
    }
}
