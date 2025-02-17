use async_graphql::{Context, Error, Object};
use crate::context::BoscaContext;
use crate::graphql::content::content_trait::TraitObject;

pub struct TraitsObject {
}

#[Object(name = "Traits")]
impl TraitsObject {

    async fn all(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<TraitObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx.workflow
            .get_traits()
            .await?
            .into_iter()
            .map(TraitObject::new)
            .collect())
    }

    #[graphql(name = "trait")]
    async fn trait_(&self, ctx: &Context<'_>, id: String) -> async_graphql::Result<Option<TraitObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx.workflow.get_trait(&id).await?.map(TraitObject::new))
    }
}