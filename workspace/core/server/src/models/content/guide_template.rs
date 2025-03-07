use async_graphql::InputObject;
use rrule::RRuleSet;
use serde::Serialize;
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;
use crate::models::content::guide_template_step::GuideTemplateStepInput;
use crate::models::content::guide_type::GuideType;
use crate::models::content::template_attribute::TemplateAttributeInput;

#[derive(Clone)]
pub struct GuideTemplate {
    pub metadata_id: Uuid,
    pub version: i32,
    pub rrule: Option<RRuleSet>,
    pub guide_type: GuideType,
    pub default_attributes: Option<Value>,
}

#[derive(InputObject, Clone, Serialize)]
pub struct GuideTemplateInput {
    pub rrule: String,
    #[graphql(name = "type")]
    pub guide_type: GuideType,
    pub attributes: Vec<TemplateAttributeInput>,
    pub default_attributes: Option<Value>,
    pub steps: Vec<GuideTemplateStepInput>
}

impl From<&Row> for GuideTemplate {
    fn from(row: &Row) -> Self {
        let rrule: Option<String> = row.get("rrule");
        Self {
            metadata_id: row.get("metadata_id"),
            version: row.get("version"),
            rrule: rrule.map(|r| r.parse().unwrap()),
            guide_type: row.get("type"),
            default_attributes: row.get("default_attributes"),
        }
    }
}
