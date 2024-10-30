use async_graphql::InputObject;
use chrono::{DateTime, Utc};
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Clone)]
pub struct MetadataSupplementary {
    pub key: String,
    pub name: String,
    pub content_type: String,
    pub content_length: Option<i64>,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub source_id: Option<Uuid>,
    pub source_identifier: Option<String>,
    pub uploaded: Option<DateTime<Utc>>,
}

#[derive(InputObject)]
pub struct MetadataSupplementaryInput {
    pub metadata_id: String,
    pub key: String,
    pub name: String,
    pub content_type: String,
    pub content_length: Option<i64>,
    pub source_id: Option<String>,
    pub source_identifier: Option<String>,
}

impl From<&Row> for MetadataSupplementary {
    fn from(row: &Row) -> Self {
        Self {
            key: row.get("key"),
            name: row.get("name"),
            content_type: row.get("content_type"),
            content_length: row.get("content_length"),
            created: row.get("created"),
            modified: row.get("modified"),
            source_id: row.get("source_id"),
            source_identifier: row.get("source_identifier"),
            uploaded: row.get("uploaded"),
        }
    }
}
