use crate::models::content::document_block_type::DocumentBlockType;
use async_graphql::InputObject;
use tokio_postgres::Row;

pub struct DocumentTemplateBlock {
    pub name: String,
    pub description: String,
    pub block_type: DocumentBlockType,
}

#[derive(InputObject, Clone)]
pub struct DocumentTemplateBlockInput {
    pub name: String,
    pub description: String,
    #[graphql(name = "type")]
    pub block_type: DocumentBlockType,
}

impl From<&Row> for DocumentTemplateBlock {
    fn from(row: &Row) -> Self {
        Self {
            name: row.get("name"),
            description: row.get("description"),
            block_type: row.get("type"),
        }
    }
}