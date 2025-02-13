use crate::models::content::document_block_type::DocumentBlockType;
use async_graphql::InputObject;
use serde_json::Value;
use tokio_postgres::Row;

pub struct DocumentTemplateBlock {
    pub name: String,
    pub description: String,
    pub block_type: DocumentBlockType,
    pub validation: Option<Value>,
    pub content: Value,
}

#[derive(InputObject, Clone)]
pub struct DocumentTemplateBlockInput {
    pub name: String,
    pub description: String,
    #[graphql(name = "type")]
    pub block_type: DocumentBlockType,
    pub validation: Option<Value>,
    pub content: Value,
}

impl From<&Row> for DocumentTemplateBlock {
    fn from(row: &Row) -> Self {
        Self {
            name: row.get("name"),
            description: row.get("description"),
            block_type: row.get("type"),
            validation: row.get("validation"),
            content: row.get("content"),
        }
    }
}