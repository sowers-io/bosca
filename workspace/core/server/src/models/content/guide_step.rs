use async_graphql::InputObject;
use serde::Serialize;
use tokio_postgres::Row;
use uuid::Uuid;
use crate::models::content::guide_step_module::GuideStepModuleInput;

#[derive(Clone)]
pub struct GuideStep {
    pub metadata_id: Uuid,
    pub version: i32,
    pub id: i64,
    pub step_metadata_id: Option<Uuid>,
    pub step_metadata_version: Option<i32>,
}

#[derive(InputObject, Clone, Serialize)]
pub struct GuideStepInput {
    pub template_metadata_id: Option<String>,
    pub template_metadata_version: Option<i32>,
    pub template_step_id: Option<i64>,
    pub step_metadata_id: Option<String>,
    pub step_metadata_version: Option<i32>,
    pub modules: Vec<GuideStepModuleInput>,
}

impl From<&Row> for GuideStep {
    fn from(row: &Row) -> Self {
        Self {
            metadata_id: row.get("metadata_id"),
            version: row.get("version"),
            id: row.get("id"),
            step_metadata_id: row.get("step_metadata_id"),
            step_metadata_version: row.get("step_metadata_version"),
        }
    }
}
