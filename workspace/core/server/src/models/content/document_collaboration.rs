use async_graphql::InputObject;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Clone)]
pub struct DocumentCollaboration {
    pub content: Vec<u8>,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
}

#[derive(InputObject, Clone, Serialize, Deserialize)]
pub struct DocumentCollaborationInput {
    pub content: Vec<u8>,
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