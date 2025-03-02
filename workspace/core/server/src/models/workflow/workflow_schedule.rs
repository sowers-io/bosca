use async_graphql::{Error, InputObject};
use chrono::{DateTime, Utc};
use rrule::RRuleSet;
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;

pub struct WorkflowSchedule {
    pub id: Uuid,
    pub metadata_id: Option<Uuid>,
    pub collection_id: Option<Uuid>,
    pub workflow_id: String,
    pub attributes: Option<Value>,
    pub configuration: Option<Value>,
    pub rrule: RRuleSet,
    pub starts: DateTime<Utc>,
    pub ends: Option<DateTime<Utc>>,
    pub last_run: Option<DateTime<Utc>>,
    pub next_run: Option<DateTime<Utc>>,
    pub last_scheduled: Option<DateTime<Utc>>,
    pub enabled: bool,
}

#[derive(InputObject)]
pub struct WorkflowScheduleInput {
    pub workflow_id: String,
    pub attributes: Option<Value>,
    pub configuration: Option<Value>,
    pub rrule: String,
    pub ends: Option<DateTime<Utc>>,
    pub enabled: bool,
}

impl WorkflowScheduleInput {
    pub fn create_schedule(&self, metadata_id: Option<Uuid>, collection_id: Option<Uuid>) -> Result<WorkflowSchedule, Error> {
        let rrule: RRuleSet = self.rrule.parse()?;
        let starts = rrule.get_dt_start().to_utc();
        Ok(WorkflowSchedule {
            id: Uuid::nil(),
            metadata_id,
            collection_id,
            workflow_id: self.workflow_id.clone(),
            attributes: self.attributes.clone(),
            configuration: self.configuration.clone(),
            rrule,
            starts,
            ends: self.ends,
            last_run: None,
            next_run: Some(starts),
            last_scheduled: Some(Utc::now()),
            enabled: self.enabled,
        })
    }
}

impl From<&Row> for WorkflowSchedule {
    fn from(row: &Row) -> Self {
        let rrule: String = row.get("rrule");
        Self {
            id: row.get("id"),
            metadata_id: row.get("metadata_id"),
            collection_id: row.get("collection_id"),
            workflow_id: row.get("workflow_id"),
            attributes: row.get("attributes"),
            configuration: row.get("configuration"),
            rrule: rrule.parse().unwrap(),
            starts: row.get("starts"),
            ends: row.get("ends"),
            last_run: row.get("last_run"),
            next_run: row.get("next_run"),
            last_scheduled: row.get("last_scheduled"),
            enabled: row.get("enabled"),
        }
    }
}

