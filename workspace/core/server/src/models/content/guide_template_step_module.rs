use async_graphql::InputObject;
use serde::Serialize;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Clone)]
pub struct GuideTemplateStepModule {
    pub id: i64,
    pub template_metadata_id: Uuid,
    pub template_metadata_version: i32,
}

#[derive(InputObject, Clone, Serialize)]
pub struct GuideTemplateStepModuleInput {
    pub template_metadata_id: String,
    pub template_metadata_version: i32,
}

impl From<&Row> for GuideTemplateStepModule {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            template_metadata_id: row.get("template_metadata_id"),
            template_metadata_version: row.get("template_metadata_version"),
        }
    }
}
