use chrono::{DateTime, Utc};
use tokio_postgres::Row;

#[derive(Clone)]
pub struct DocumentCollaboration {
    pub content: Vec<u8>,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
}

impl From<&Row> for DocumentCollaboration {
    fn from(row: &Row) -> Self {
        Self {
            content: row.get("content"),
            created: row.get("created"),
            modified: row.get("modified"),
        }
    }
}