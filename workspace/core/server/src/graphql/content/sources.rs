use async_graphql::{Context, Error, Object};
use uuid::Uuid;
use crate::context::BoscaContext;
use crate::graphql::content::source::SourceObject;

pub struct SourcesObject {
}

#[Object(name = "Sources")]
impl SourcesObject {

    async fn sources(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<SourceObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .content
            .get_sources()
            .await?
            .into_iter()
            .map(SourceObject::new)
            .collect())
    }

    async fn source(&self, ctx: &Context<'_>, id: String) -> async_graphql::Result<Option<SourceObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(match Uuid::parse_str(id.as_str()) {
            Ok(id) => ctx.content.get_source_by_id(&id).await?,
            Err(_) => ctx.content.get_source_by_name(&id).await?,
        }
            .map(|s| s.into()))
    }
}
