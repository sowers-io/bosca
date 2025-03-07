use crate::context::BoscaContext;
use crate::graphql::security::login_mutation::LoginMutationObject;
use crate::graphql::security::signup_mutation::SignupMutationObject;
use async_graphql::*;
use uuid::Uuid;

pub struct SecurityMutationObject {}

#[Object(name = "SecurityMutation")]
impl SecurityMutationObject {
    async fn login(&self) -> LoginMutationObject {
        LoginMutationObject {}
    }

    async fn signup(&self) -> SignupMutationObject {
        SignupMutationObject {}
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
        let admin_group = ctx.security.get_administrators_group().await?;
        if !ctx.principal.has_group(&admin_group.id) {
            return Err(Error::new("invalid permissions"));
        }
        let id = Uuid::parse_str(principal_id.as_str())?;
        let group_id = Uuid::parse_str(group_id.as_str())?;
        ctx.security.add_principal_group(&id, &group_id).await?;
        Ok(true)
    }
}
