use async_graphql::InputObject;
use serde::Serialize;
use tokio_postgres::Row;
use uuid::Uuid;
use crate::models::content::guide_template_step_module::GuideTemplateStepModuleInput;

#[derive(Clone)]
pub struct GuideTemplateStep {
    pub id: i64,
    pub template_metadata_id: Option<Uuid>,
    pub template_metadata_version: Option<i32>,
}

#[derive(InputObject, Clone, Serialize)]
pub struct GuideTemplateStepInput {
    pub template_metadata_id: Option<String>,
    pub template_metadata_version: Option<i32>,
    pub modules: Vec<GuideTemplateStepModuleInput>,
}

impl From<&Row> for GuideTemplateStep {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            template_metadata_id: row.get("template_metadata_id"),
            template_metadata_version: row.get("template_metadata_version"),
        }
    }
}
