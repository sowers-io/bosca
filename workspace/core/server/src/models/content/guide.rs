use async_graphql::InputObject;
use rrule::RRuleSet;
use serde::Serialize;
use tokio_postgres::Row;
use uuid::Uuid;
use crate::models::content::guide_step::GuideStepInput;
use crate::models::content::guide_type::GuideType;

#[derive(Clone)]
pub struct Guide {
    pub metadata_id: Uuid,
    pub version: i32,
    pub template_metadata_id: Option<Uuid>,
    pub template_metadata_version: Option<i32>,
    pub rrule: Option<RRuleSet>,
    pub guide_type: GuideType,
}

#[derive(InputObject, Clone, Serialize)]
pub struct GuideInput {
    pub guide_type: GuideType,
    pub rrule: Option<String>,
    pub template_metadata_id: Option<String>,
    pub template_metadata_version: Option<i32>,
    pub steps: Option<GuideStepInput>,
}

impl From<&Row> for Guide {
    fn from(row: &Row) -> Self {
        let rrule: Option<String> = row.get("rrule");
        Self {
            metadata_id: row.get("metadata_id"),
            version: row.get("version"),
            template_metadata_id: row.get("template_metadata_id"),
            template_metadata_version: row.get("template_metadata_version"),
            rrule: rrule.map(|r| r.parse().unwrap()),
            guide_type: row.get("type"),
        }
    }
}
