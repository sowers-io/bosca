use async_graphql::InputObject;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;
use crate::models::content::guide_template_step_module::GuideTemplateStepModuleInput;

#[derive(Clone)]
pub struct GuideTemplateStep {
    pub id: i64,
    pub template_metadata_id: Uuid,
    pub template_metadata_version: i32,
}

#[derive(InputObject, Clone, Serialize, Deserialize)]
pub struct GuideTemplateStepInput {
    pub template_metadata_id: String,
    pub template_metadata_version: i32,
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
