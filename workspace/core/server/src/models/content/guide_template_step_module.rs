use async_graphql::InputObject;
use serde::Serialize;
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Clone)]
pub struct GuideTemplateStepModule {
    pub template_metadata_id: Uuid,
    pub template_metadata_version: i32,
}

#[derive(InputObject, Clone, Serialize)]
pub struct GuideTemplateStepModuleInput {
    pub template_metadata_id: String,
    pub template_metadata_version: i32,
    pub configuration: Option<Value>,
}

impl From<&Row> for GuideTemplateStepModule {
    fn from(row: &Row) -> Self {
        Self {
            template_metadata_id: row.get("template_metadata_id"),
            template_metadata_version: row.get("template_metadata_version"),
        }
    }
}
