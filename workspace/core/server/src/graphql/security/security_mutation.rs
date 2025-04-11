use crate::context::BoscaContext;
use crate::graphql::security::login_mutation::LoginMutationObject;
use crate::graphql::security::signup_mutation::SignupMutationObject;
use async_graphql::*;
use uuid::Uuid;
use crate::graphql::security::groups_mutation::GroupsMutation;
use crate::graphql::security::principal_mutation::PrincipalMutation;

pub struct SecurityMutationObject {}

#[Object(name = "SecurityMutation")]
impl SecurityMutationObject {

    async fn login(&self) -> LoginMutationObject {
        LoginMutationObject {}
    }

    async fn signup(&self) -> SignupMutationObject {
        SignupMutationObject {}
    }

    async fn groups(&self) -> GroupsMutation {
        GroupsMutation {}
    }

    async fn principal(&self, ctx: &Context<'_>, id: Option<String>) -> Result<PrincipalMutation, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("unauthenticated"));
        }
        if !ctx.principal.verified {
            return Err(Error::new("not verified"));
        }
        let principal = if let Some(id) = id {
            let id = Uuid::parse_str(&id)?;
            ctx.check_has_service_account().await?;
            ctx.security.get_principal_by_id(&id).await?
        } else {
            ctx.principal.clone()
        };
        Ok(PrincipalMutation::new(principal))
    }

    async fn expire_refresh_tokens(&self, ctx: &Context<'_>) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.security.expire_refresh_tokens().await?;
        Ok(true)
    }

    async fn add_principal_group(
        &self,
        ctx: &Context<'_>,
        principal_id: String,
        group_id: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        let id = Uuid::parse_str(principal_id.as_str())?;
        let group_id = Uuid::parse_str(group_id.as_str())?;
        ctx.security.add_principal_group(&id, &group_id).await?;
        Ok(true)
    }

    async fn remove_principal_group(
        &self,
        ctx: &Context<'_>,
        principal_id: String,
        group_id: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        let id = Uuid::parse_str(principal_id.as_str())?;
        let group_id = Uuid::parse_str(group_id.as_str())?;
        ctx.security.remove_principal_group(&id, &group_id).await?;
        Ok(true)
    }
}
