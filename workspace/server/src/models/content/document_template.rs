use crate::models::content::document_template_block::DocumentTemplateBlockInput;
use crate::models::content::document_template_attributes::DocumentTemplateAttributeInput;
use async_graphql::InputObject;
use chrono::{DateTime, Utc};
use serde_json::Value;
use tokio_postgres::Row;

#[derive(Clone)]
pub struct DocumentTemplate {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub allow_user_defined_blocks: bool,
    pub configuration: Value,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
}

#[derive(InputObject, Clone)]
pub struct DocumentTemplateInput {
    pub name: String,
    pub description: String,
    pub allow_user_defined_blocks: bool,
    pub configuration: Value,
    pub attributes: Vec<DocumentTemplateAttributeInput>,
    pub blocks: Vec<DocumentTemplateBlockInput>,
}

impl From<&Row> for DocumentTemplate {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            configuration: row.get("configuration"),
            allow_user_defined_blocks: row.get("allow_user_defined_blocks"),
            created: row.get("created"),
            modified: row.get("modified"),
        }
    }
}