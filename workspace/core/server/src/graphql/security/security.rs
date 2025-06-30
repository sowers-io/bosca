use crate::context::BoscaContext;
use crate::graphql::security::groups::GroupsObject;
use crate::graphql::security::login::LoginObject;
use crate::graphql::security::principal::PrincipalObject;
use crate::models::security::permission::PermissionAction;
use async_graphql::*;
use crate::graphql::security::principals::PrincipalsObject;

pub struct SecurityObject {}

#[Object(name = "Security")]
impl SecurityObject {
    async fn principals(&self) -> PrincipalsObject {
        PrincipalsObject {}
    }

    async fn principal(&self, ctx: &Context<'_>) -> Result<PrincipalObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(PrincipalObject::new(ctx.principal.clone()))
    }

    async fn actions(&self) -> Result<Vec<String>, Error> {
        Ok([PermissionAction::View,
            PermissionAction::Edit,
            PermissionAction::Delete,
            PermissionAction::Manage,
            PermissionAction::List,
            PermissionAction::Execute]
        .iter()
        .map(|id| format!("{id:?}"))
        .collect())
    }

    async fn login(&self) -> LoginObject {
        LoginObject {}
    }

    async fn groups(&self) -> GroupsObject {
        GroupsObject {}
    }
}
