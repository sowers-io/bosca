use crate::models::content::document_block_type::DocumentBlockType;
use async_graphql::InputObject;
use serde_json::Value;
use tokio_postgres::Row;

pub struct DocumentTemplateBlock {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub block_type: DocumentBlockType,
    pub configuration: Value,
    pub validation: Option<Value>,
    pub content: Value,
    pub required: bool,
}

#[derive(InputObject, Clone)]
pub struct DocumentTemplateBlockInput {
    pub name: String,
    pub description: String,
    #[graphql(name = "type")]
    pub block_type: DocumentBlockType,
    pub configuration: Value,
    pub validation: Option<Value>,
    pub content: Value,
    pub required: bool,
}

impl From<&Row> for DocumentTemplateBlock {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            block_type: row.get("type"),
            configuration: row.get("configuration"),
            validation: row.get("validation"),
            content: row.get("content"),
            required: row.get("required"),
        }
    }
}