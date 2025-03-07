use crate::context::BoscaContext;
use crate::graphql::content::source::SourceObject;
use crate::models::content::source::SourceInput;
use async_graphql::{Context, Error, Object};

pub struct SourceMutationObject {}

#[Object(name = "SourceMutation")]
impl SourceMutationObject {
    async fn add(&self, ctx: &Context<'_>, source: SourceInput) -> Result<Option<SourceObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = ctx.content.sources.add(&source).await?;
        let source = ctx.content.sources.get_source_by_id(&id).await?;
        Ok(source.map(|s| s.into()))
    }
}
