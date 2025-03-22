use async_graphql::InputObject;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;
use crate::models::content::metadata::MetadataInput;

#[derive(Clone)]
pub struct GuideStepModule {
    pub id: i64,
    pub module_metadata_id: Uuid,
    pub module_metadata_version: i32,
}

#[derive(InputObject, Clone, Default, Serialize, Deserialize)]
pub struct GuideStepModuleInput {
    pub module_metadata_id: Option<String>,
    pub module_metadata_version: Option<i32>,
    pub metadata: Option<MetadataInput>,
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
