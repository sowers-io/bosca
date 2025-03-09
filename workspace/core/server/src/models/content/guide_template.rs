use crate::models::content::guide_template_step::GuideTemplateStepInput;
use crate::models::content::guide_type::GuideType;
use async_graphql::InputObject;
use rrule::RRuleSet;
use serde::Serialize;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Clone)]
pub struct GuideTemplate {
    pub metadata_id: Uuid,
    pub version: i32,
    pub rrule: Option<RRuleSet>,
    pub guide_type: GuideType,
}

#[derive(InputObject, Clone, Serialize)]
pub struct GuideTemplateInput {
    pub rrule: String,
    #[graphql(name = "type")]
    pub guide_type: GuideType,
    pub steps: Vec<GuideTemplateStepInput>,
}

impl From<&Row> for GuideTemplate {
    fn from(row: &Row) -> Self {
        let rrule: Option<String> = row.get("rrule");
        Self {
            metadata_id: row.get("metadata_id"),
            version: row.get("version"),
            rrule: rrule.filter(|r| !r.is_empty()).map(|r| r.parse().unwrap()),
            guide_type: row.get("type"),
        }
    }
}
