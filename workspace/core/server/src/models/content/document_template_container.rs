use crate::models::content::document_template_container_type::DocumentTemplateContainerType;
use crate::models::content::template_workflow::TemplateWorkflowInput;
use async_graphql::InputObject;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Clone)]
pub struct DocumentTemplateContainer {
    pub id: String,
    pub name: String,
    pub description: String,
    pub supplementary_key: Option<String>,
    pub container_type: DocumentTemplateContainerType,
}

#[derive(InputObject, Clone, Serialize, Deserialize)]
pub struct DocumentTemplateContainerInput {
    pub id: String,
    pub name: String,
    pub description: String,
    pub supplementary_key: Option<String>,
    pub workflows: Vec<TemplateWorkflowInput>,
    #[serde(rename = "type")]
    pub container_type: Option<DocumentTemplateContainerType>,
}

impl From<&Row> for DocumentTemplateContainer {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            supplementary_key: row.get("supplementary_key"),
            container_type: row.get("type")
        }
    }
}
