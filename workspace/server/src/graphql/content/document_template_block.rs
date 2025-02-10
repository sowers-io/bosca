use crate::models::content::document_block_type::DocumentBlockType;
use crate::models::content::document_template_block::DocumentTemplateBlock;
use async_graphql::Object;

pub struct DocumentTemplateBlockObject {
    pub block: DocumentTemplateBlock,
}

impl DocumentTemplateBlockObject {
    pub fn new(block: DocumentTemplateBlock) -> Self {
        Self { block }
    }
}

#[Object(name = "DocumentTemplateBlock")]
impl DocumentTemplateBlockObject {
    #[graphql(name = "type")]
    pub async fn block_type(&self) -> DocumentBlockType {
        self.block.block_type
    }

    pub async fn name(&self) -> &String {
        &self.block.name
    }

    pub async fn description(&self) -> &String {
        &self.block.description
    }
}
