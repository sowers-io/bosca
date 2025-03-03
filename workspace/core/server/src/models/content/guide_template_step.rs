use async_graphql::InputObject;
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;
use crate::models::content::guide_template_step_module::GuideTemplateStepModuleInput;
use crate::models::content::template_attribute::TemplateAttributeInput;

#[derive(Clone)]
pub struct GuideTemplateStep {
    pub metadata_id: Uuid,
    pub version: i32,
    pub id: i64,
    pub name: String,
    pub description: String,
    pub configuration: Option<Value>,
}

#[derive(InputObject, Clone)]
pub struct GuideTemplateStepInput {
    pub name: String,
    pub description: String,
    pub attributes: Vec<TemplateAttributeInput>,
    pub configuration: Option<Value>,
    pub modules: Vec<GuideTemplateStepModuleInput>,
}

impl From<&Row> for GuideTemplateStep {
    fn from(row: &Row) -> Self {
        Self {
            metadata_id: row.get("metadata_id"),
            version: row.get("version"),
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            configuration: row.get("configuration"),
        }
    }
}
