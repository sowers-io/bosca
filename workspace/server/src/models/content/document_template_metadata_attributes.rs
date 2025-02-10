use async_graphql::*;
use tokio_postgres::Row;
use crate::models::content::document_metadata_attribute_type::DocumentMetadataAttributeType;

#[derive(Clone)]
pub struct DocumentTemplateMetadataAttribute {
    pub id: i64,
    pub key: String,
    pub name: String,
    pub description: String,
    pub attribute_type: DocumentMetadataAttributeType
}

#[derive(Clone)]
pub struct DocumentTemplateMetadataAttributeWorkflow {
    pub workflow_id: String,
    pub auto_run: bool
}

#[derive(InputObject)]
pub struct DocumentTemplateMetadataAttributeInput {
    pub key: String,
    pub name: String,
    pub description: String,
    #[graphql(name = "type")]
    pub attribute_type: DocumentMetadataAttributeType,
    pub workflow_ids: Vec<DocumentTemplateMetadataAttributeWorkflowInput>
}

#[derive(InputObject)]
pub struct DocumentTemplateMetadataAttributeWorkflowInput {
    pub workflow_id: String,
    pub auto_run: bool
}

impl From<&Row> for DocumentTemplateMetadataAttribute {
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

impl From<&Row> for DocumentTemplateMetadataAttributeWorkflow {
    fn from(row: &Row) -> Self {
        Self {
            workflow_id: row.get("workflow_id"),
            auto_run: row.get("auto_run"),
        }
    }
}