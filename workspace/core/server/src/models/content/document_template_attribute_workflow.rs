use async_graphql::*;
use tokio_postgres::Row;

#[derive(Clone)]
pub struct DocumentTemplateAttributeWorkflow {
    pub workflow_id: String,
    pub auto_run: bool
}

#[derive(InputObject, Clone)]
pub struct DocumentTemplateAttributeWorkflowInput {
    pub workflow_id: String,
    pub auto_run: bool
}

impl From<&Row> for DocumentTemplateAttributeWorkflow {
    fn from(row: &Row) -> Self {
        Self {
            workflow_id: row.get("workflow_id"),
            auto_run: row.get("auto_run"),
        }
    }
}