use crate::models::content::guide_template_step::GuideTemplateStepInput;
use crate::models::content::guide_type::GuideType;
use async_graphql::InputObject;
use rrule::RRuleSet;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;
use chrono::{self, Timelike};

#[derive(Clone)]
pub struct GuideTemplate {
    pub metadata_id: Uuid,
    pub version: i32,
    pub rrule: Option<RRuleSet>,
    pub guide_type: GuideType,
    pub default_attributes: Option<Value>,
    pub configuration: Option<Value>,
}

#[derive(InputObject, Clone, Serialize, Deserialize)]
pub struct GuideTemplateInput {
    pub rrule: String,
    #[graphql(name = "type")]
    pub guide_type: GuideType,
    pub steps: Vec<GuideTemplateStepInput>,
    pub default_attributes: Option<Value>,
    pub configuration: Option<Value>,
}

impl From<&Row> for GuideTemplate {
    fn from(row: &Row) -> Self {
        let rrule: Option<String> = row.get("rrule");
        Self {
            metadata_id: row.get("metadata_id"),
            version: row.get("version"),
            rrule: rrule.filter(|r| !r.is_empty()).map(|r| {
                // For guide templates, RRULE strings may not include DTSTART
                // Add a default start date if missing to make it parseable
                if r.contains("DTSTART") {
                    r.parse().unwrap()
                } else {
                    // Add a default DTSTART to make the RRULE parseable
                    let dtstart = chrono::Utc::now()
                        .with_hour(0).unwrap()
                        .with_minute(0).unwrap()
                        .with_second(0).unwrap()
                        .with_nanosecond(0).unwrap();
                    let rrule_with_dtstart = format!("DTSTART:{}\n{}", 
                        dtstart.format("%Y%m%dT%H%M%SZ"), r);
                    rrule_with_dtstart.parse().unwrap()
                }
            }),
            guide_type: row.get("type"),
            default_attributes: row.get("default_attributes"),
            configuration: row.get("configuration"),
        }
    }
}
