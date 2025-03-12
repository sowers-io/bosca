use async_graphql::InputObject;
use serde::Serialize;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Clone)]
pub struct GuideStepModule {
    pub id: i64,
    pub module_metadata_id: Uuid,
    pub module_metadata_version: i32,
}

#[derive(InputObject, Clone, Default, Serialize)]
pub struct GuideStepModuleInput {
    pub module_metadata_id: String,
    pub module_metadata_version: i32,
}

impl From<&Row> for GuideStepModule {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            module_metadata_id: row.get("module_metadata_id"),
            module_metadata_version: row.get("module_metadata_version"),
        }
    }
}
