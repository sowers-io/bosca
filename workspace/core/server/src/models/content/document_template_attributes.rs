use async_graphql::*;
use tokio_postgres::Row;
use crate::models::content::attribute_type::AttributeType;
use crate::models::content::attribute_ui_type::AttributeUiType;
use crate::models::content::document_template_attribute_workflow::DocumentTemplateAttributeWorkflowInput;

#[derive(Clone)]
pub struct DocumentTemplateAttribute {
    pub key: String,
    pub name: String,
    pub description: String,
    pub configuration: Option<serde_json::Value>,
    pub attribute_type: AttributeType,
    pub supplementary_key: Option<String>,
    pub ui: AttributeUiType,
    pub list: bool,
}

#[derive(InputObject, Clone)]
pub struct DocumentTemplateAttributeInput {
    pub key: String,
    pub name: String,
    pub description: String,
    pub configuration: Option<serde_json::Value>,
    #[graphql(name = "type")]
    pub attribute_type: AttributeType,
    pub supplementary_key: Option<String>,
    pub ui: AttributeUiType,
    pub list: bool,
    pub workflow_ids: Vec<DocumentTemplateAttributeWorkflowInput>
}


impl From<&Row> for DocumentTemplateAttribute {
    fn from(row: &Row) -> Self {
        Self {
            key: row.get("key"),
            name: row.get("name"),
            description: row.get("description"),
            configuration: row.get("configuration"),
            attribute_type: row.get("type"),
            supplementary_key: row.get("supplementary_key"),
            ui: row.get("ui"),
            list: row.get("list"),
        }
    }
}
