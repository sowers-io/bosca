use async_graphql::InputObject;
use tokio_postgres::Row;
use uuid::Uuid;
use crate::models::content::guide_template_step_module::GuideTemplateStepModuleInput;
use crate::models::content::template_attribute::TemplateAttributeInput;

#[derive(Clone)]
pub struct GuideTemplateStep {
    pub metadata_id: Uuid,
    pub version: i32,
    pub id: i64,
    pub template_metadata_id: Option<Uuid>,
    pub template_metadata_version: Option<i32>,
}

#[derive(InputObject, Clone)]
pub struct GuideTemplateStepInput {
    pub template_metadata_id: Option<String>,
    pub template_metadata_version: Option<i32>,
    pub attributes: Vec<TemplateAttributeInput>,
    pub modules: Vec<GuideTemplateStepModuleInput>,
}

impl From<&Row> for GuideTemplateStep {
    fn from(row: &Row) -> Self {
        Self {
            metadata_id: row.get("metadata_id"),
            version: row.get("version"),
            id: row.get("id"),
            template_metadata_id: row.get("template_metadata_id"),
            template_metadata_version: row.get("template_metadata_version"),
        }
    }
}
