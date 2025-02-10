use crate::context::BoscaContext;
use crate::graphql::content::document_block_metadata::DocumentBlockMetadataObject;
use crate::models::content::document_block::DocumentBlock;
use crate::models::content::document_block_type::DocumentBlockType;
use async_graphql::{Context, Error, Object};
use serde_json::Value;
use uuid::Uuid;

pub struct DocumentBlockObject {
    pub metadata_id: Uuid,
    pub version: i32,
    pub block: DocumentBlock,
}

impl DocumentBlockObject {
    pub fn new(metadata_id: Uuid, version: i32, block: DocumentBlock) -> Self {
        Self {
            metadata_id,
            version,
            block,
        }
    }
}

#[Object(name = "DocumentBlock")]
impl DocumentBlockObject {
    #[graphql(name = "type")]
    pub async fn block_type(&self) -> DocumentBlockType {
        self.block.block_type
    }

    pub async fn content(&self) -> &Value {
        &self.block.content
    }

    pub async fn metadata(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<DocumentBlockMetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata = ctx
            .content
            .documents
            .get_metadata(&self.metadata_id, self.version, self.block.id)
            .await?;
        Ok(metadata
            .into_iter()
            .map(DocumentBlockMetadataObject::new)
            .collect())
    }
}
