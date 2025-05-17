use chrono::{DateTime, Utc};
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Clone)]
pub struct GuideProgress {
    pub metadata_id: Uuid,
    pub version: i32,
    pub attributes: Value,
    pub started: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub completed_step_ids: Vec<i64>,
}

impl From<&Row> for GuideProgress {
    fn from(row: &Row) -> Self {
        Self {
            metadata_id: row.get("metadata_id"),
            version: row.get("version"),
            attributes: row.get("attributes"),
            started: row.get("started"),
            modified: row.get("modified"),
            completed_step_ids: row.get("completed_step_ids"),
        }
    }
}
