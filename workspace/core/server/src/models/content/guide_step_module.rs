use async_graphql::InputObject;
use serde::Serialize;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Clone)]
pub struct GuideStepModule {
    pub module_metadata_id: Uuid,
    pub module_metadata_version: i32,
}

#[derive(InputObject, Clone, Serialize)]
pub struct GuideStepModuleInput {
    pub template_metadata_id: Option<String>,
    pub template_metadata_version: Option<i32>,
    pub template_step_id: Option<i64>,
    pub template_module_id: Option<i64>,
    pub module_metadata_id: String,
    pub module_metadata_version: i32,
}

impl From<&Row> for GuideStepModule {
    fn from(row: &Row) -> Self {
        Self {
            module_metadata_id: row.get("module_metadata_id"),
            module_metadata_version: row.get("module_metadata_version"),
        }
    }
}
