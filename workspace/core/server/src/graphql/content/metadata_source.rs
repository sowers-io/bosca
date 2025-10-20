use async_graphql::{Context, Error, Object};
use crate::context::BoscaContext;
use crate::graphql::content::source::SourceObject;
use crate::models::content::metadata::Metadata;

pub struct MetadataSourceObject {
    pub metadata: Metadata,
}

#[Object(name = "MetadataSource")]
impl MetadataSourceObject {
    async fn id(&self) -> Option<String> {
        self.metadata.source_id.map(|s| s.to_string())
    }

    async fn identifier(&self) -> &Option<String> {
        &self.metadata.source_identifier
    }

    async fn source_url(&self) -> &Option<String> {
        &self.metadata.source_url
    }

    async fn source(&self, ctx: &Context<'_>) -> Result<Option<SourceObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if let Some(id) = &self.metadata.source_id {
            Ok(if let Some(source) = ctx.content.sources.get_source_by_id(id).await? {
                Some(SourceObject::new(source))
            } else {
                None
            })
        } else {
            Ok(None)
        }
    }
}
