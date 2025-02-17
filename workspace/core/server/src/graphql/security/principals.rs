use crate::context::BoscaContext;
use crate::graphql::security::principal::PrincipalObject;
use async_graphql::{Context, Error, Object};

pub struct PrincipalsObject {
}

#[Object(name = "Principals")]
impl PrincipalsObject {
    async fn all(
        &self,
        ctx: &Context<'_>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<PrincipalObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        Ok(ctx
            .security
            .get_principals(offset, limit)
            .await?
            .into_iter()
            .map(PrincipalObject::new)
            .collect())
    }

    async fn current(&self, ctx: &Context<'_>) -> async_graphql::Result<PrincipalObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(PrincipalObject::new(ctx.principal.clone()))
    }
}
