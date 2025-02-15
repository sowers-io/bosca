use crate::models::content::document_template_attributes::DocumentTemplateAttributeInput;
use async_graphql::InputObject;
use serde_json::Value;
use tokio_postgres::Row;

#[derive(Clone)]
pub struct DocumentTemplate {
    pub configuration: Option<Value>,
    pub schema: Option<Value>,
    pub content: Value,
}

#[derive(InputObject, Clone)]
pub struct DocumentTemplateInput {
    pub attributes: Vec<DocumentTemplateAttributeInput>,
    pub configuration: Option<Value>,
    pub schema: Option<Value>,
    pub content: Value,
}

impl From<&Row> for DocumentTemplate {
    fn from(row: &Row) -> Self {
        Self {
            configuration: row.get("configuration"),
            schema: row.get("schema"),
            content: row.get("content"),
        }
    }
}