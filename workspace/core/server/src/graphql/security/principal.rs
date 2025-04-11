use crate::context::BoscaContext;
use crate::graphql::security::group::GroupObject;
use crate::models::security::principal::Principal;
use async_graphql::{Context, Object};
use crate::graphql::security::principal_credential::PrincipalCredentialObject;

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

    async fn credentials(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<PrincipalCredentialObject>, async_graphql::Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let credentials = ctx
            .security
            .get_principal_credentials(&ctx.principal.id)
            .await?;
        Ok(credentials
            .into_iter()
            .map(|c| PrincipalCredentialObject::new(c.identifier(), c.get_type()))
            .collect())
    }
}
