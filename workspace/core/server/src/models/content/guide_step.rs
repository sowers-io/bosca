use async_graphql::InputObject;
use serde::Serialize;
use tokio_postgres::Row;
use uuid::Uuid;
use crate::models::content::guide_step_module::GuideStepModuleInput;

#[derive(Clone)]
pub struct GuideStep {
    pub metadata_id: Uuid,
    pub metadata_version: i32,
    pub id: i64,
    pub step_metadata_id: Option<Uuid>,
    pub step_metadata_version: Option<i32>,
}

#[derive(InputObject, Clone, Serialize)]
pub struct GuideStepInput {
    pub step_metadata_id: Option<String>,
    pub step_metadata_version: Option<i32>,
    pub modules: Vec<GuideStepModuleInput>,
}

impl From<&Row> for GuideStep {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            metadata_id: row.get("metadata_id"),
            metadata_version: row.get("metadata_version"),
            step_metadata_id: row.get("step_metadata_id"),
            step_metadata_version: row.get("step_metadata_version"),
        }
    }
}
