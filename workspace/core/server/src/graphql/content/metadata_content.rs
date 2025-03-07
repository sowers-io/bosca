use async_graphql::{Context, Error, Object};
use serde_json::Value;
use crate::context::BoscaContext;
use crate::graphql::content::metadata_content_urls::MetadataContentUrls;
use crate::models::content::metadata::Metadata;

pub struct MetadataContentObject {
    pub metadata: Metadata,
}

#[Object(name = "MetadataContent")]
impl MetadataContentObject {
    #[graphql(name = "type")]
    async fn content_type(&self) -> &String {
        &self.metadata.content_type
    }

    async fn length(&self) -> Option<i64> {
        self.metadata.content_length
    }

    async fn urls(&self) -> MetadataContentUrls {
        MetadataContentUrls {
            metadata: self.metadata.clone(),
        }
    }

    async fn text(&self, ctx: &Context<'_>) -> Result<String, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let path = ctx.storage.get_metadata_path(&self.metadata, None).await?;
        Ok(ctx.storage.get(&path).await?)
    }

    async fn json(&self, ctx: &Context<'_>) -> Result<Value, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let path = ctx.storage.get_metadata_path(&self.metadata, None).await?;
        let text = ctx.storage.get(&path).await?;
        Ok(serde_json::from_str(text.as_str())?)
    }
}
