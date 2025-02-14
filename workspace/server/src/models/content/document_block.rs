use crate::models::content::document_block_type::DocumentBlockType;
use async_graphql::InputObject;
use serde_json::Value;
use tokio_postgres::Row;
use crate::models::content::document_block_metadata::DocumentBlockMetadataInput;

#[derive(Clone)]
pub struct DocumentBlock {
    pub id: i64,
    pub template_block_id: Option<i64>,
    pub block_type: DocumentBlockType,
    pub content: Value,
}

#[derive(InputObject, Clone)]
pub struct DocumentBlockInput {
    pub template_block_id: Option<i64>,
    pub references: Vec<DocumentBlockMetadataInput>,
    #[graphql(name = "type")]
    pub block_type: DocumentBlockType,
    pub content: Value,
}

impl From<&Row> for DocumentBlock {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            template_block_id: row.get("template_block_id"),
            block_type: row.get("type"),
            content: row.get("content"),
        }
    }
}