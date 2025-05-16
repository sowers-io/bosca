use chrono::{DateTime, Utc};
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Clone)]
pub struct GuideHistory {
    pub metadata_id: Uuid,
    pub version: i32,
    pub attributes: Value,
    pub completed: Option<DateTime<Utc>>,
}

impl From<&Row> for GuideHistory {
    fn from(row: &Row) -> Self {
        Self {
            metadata_id: row.get("metadata_id"),
            version: row.get("version"),
            attributes: row.get("attributes"),
            completed: row.get("completed"),
        }
    }
}
