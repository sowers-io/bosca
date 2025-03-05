use async_graphql::InputObject;
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;
use crate::models::content::document_template_container::DocumentTemplateContainerInput;
use crate::models::content::template_attribute::TemplateAttributeInput;

#[derive(Clone)]
pub struct DocumentTemplate {
    pub metadata_id: Uuid,
    pub version: i32,
    pub configuration: Option<Value>,
    pub schema: Option<Value>,
    pub default_attributes: Option<Value>,
    pub content: Value,
}

#[derive(InputObject, Clone)]
pub struct DocumentTemplateInput {
    pub attributes: Vec<TemplateAttributeInput>,
    pub configuration: Option<Value>,
    pub schema: Option<Value>,
    pub default_attributes: Option<Value>,
    pub containers: Option<Vec<DocumentTemplateContainerInput>>,
    pub content: Value,
}

impl From<&Row> for DocumentTemplate {
    fn from(row: &Row) -> Self {
        Self {
            metadata_id: row.get("metadata_id"),
            version: row.get("version"),
            configuration: row.get("configuration"),
            schema: row.get("schema"),
            default_attributes: row.get("default_attributes"),
            content: row.get("content"),
        }
    }
}