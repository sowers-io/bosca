use crate::context::BoscaContext;
use crate::graphql::security::groups::GroupsObject;
use crate::graphql::security::login::LoginObject;
use crate::graphql::security::principals::PrincipalObject;
use crate::models::security::permission::PermissionAction;
use async_graphql::*;

pub struct SecurityObject {}

#[Object(name = "Security")]
impl SecurityObject {
    async fn principal(&self, ctx: &Context<'_>) -> Result<PrincipalObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(PrincipalObject::new(ctx.principal.clone()))
    }

    async fn actions(&self) -> Result<Vec<String>, Error> {
        Ok(vec![
            PermissionAction::View,
            PermissionAction::Edit,
            PermissionAction::Delete,
            PermissionAction::Manage,
            PermissionAction::List,
        ]
        .iter()
        .map(|id| format!("{:?}", id))
        .collect())
    }

    async fn login(&self) -> LoginObject {
        LoginObject {}
    }

    async fn groups(&self) -> GroupsObject {
        GroupsObject {}
    }
}
