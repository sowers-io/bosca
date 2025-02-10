use async_graphql::*;
use tokio_postgres::Row;
use crate::models::content::document_attribute_type::DocumentAttributeType;
use crate::models::content::document_template_attribute_workflow::DocumentTemplateAttributeWorkflowInput;

#[derive(Clone)]
pub struct DocumentTemplateAttribute {
    pub id: i64,
    pub key: String,
    pub name: String,
    pub description: String,
    pub attribute_type: DocumentAttributeType
}

#[derive(InputObject, Clone)]
pub struct DocumentTemplateAttributeInput {
    pub key: String,
    pub name: String,
    pub description: String,
    #[graphql(name = "type")]
    pub attribute_type: DocumentAttributeType,
    pub workflow_ids: Vec<DocumentTemplateAttributeWorkflowInput>
}


impl From<&Row> for DocumentTemplateAttribute {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            key: row.get("key"),
            name: row.get("name"),
            description: row.get("description"),
            attribute_type: row.get("type"),
        }
    }
}
