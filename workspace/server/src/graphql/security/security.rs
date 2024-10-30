use crate::graphql::security::login::LoginObject;
use crate::graphql::security::principals::PrincipalObject;
use async_graphql::*;
use crate::context::BoscaContext;

pub struct SecurityObject {}

#[Object(name = "Security")]
impl SecurityObject {
    async fn principal(&self, ctx: &Context<'_>) -> Result<PrincipalObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(PrincipalObject::new(ctx.principal.clone()))
    }

    async fn login(&self) -> LoginObject {
        LoginObject {}
    }
}
