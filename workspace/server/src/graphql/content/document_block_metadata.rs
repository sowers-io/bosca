use crate::context::BoscaContext;
use crate::graphql::content::metadata::MetadataObject;
use crate::models::content::document_block_metadata::DocumentBlockMetadata;
use async_graphql::{Context, Error, Object};
use serde_json::Value;

pub struct DocumentBlockMetadataObject {
    pub metadata: DocumentBlockMetadata,
}

impl DocumentBlockMetadataObject {
    pub fn new(metadata: DocumentBlockMetadata) -> Self {
        Self { metadata }
    }
}

#[Object(name = "DocumentBlockMetadata")]
impl DocumentBlockMetadataObject {
    pub async fn metadata(&self, ctx: &Context<'_>) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata = ctx.content.metadata.get(&self.metadata.metadata_id).await?;
        Ok(metadata.map(MetadataObject::new))
    }

    pub async fn attributes(&self) -> &Option<Value> {
        &self.metadata.attributes
    }
}
