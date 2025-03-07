use crate::models::profiles::profile_visibility::ProfileVisibility;
use async_graphql::InputObject;
use chrono::{DateTime, Utc};
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileAttribute {
    pub id: Uuid,
    pub type_id: String,
    pub visibility: ProfileVisibility,
    pub confidence: i32,
    pub priority: i32,
    pub source: String,
    pub attributes: Option<Value>,
    pub metadata_id: Option<Uuid>,
    pub expiration: Option<DateTime<Utc>>,
}

#[derive(InputObject, Debug, Clone, PartialEq, Eq)]
pub struct ProfileAttributeInput {
    pub id: Option<String>,
    pub type_id: String,
    pub visibility: ProfileVisibility,
    pub confidence: i32,
    pub priority: i32,
    pub source: String,
    pub attributes: Option<Value>,
    pub metadata_id: Option<String>,
    pub metadata_supplementary: Option<String>,
    pub expiration: Option<DateTime<Utc>>,
}

impl From<&Row> for ProfileAttribute {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            type_id: row.get("type_id"),
            visibility: row.get("visibility"),
            confidence: row.get("confidence"),
            priority: row.get("priority"),
            source: row.get("source"),
            metadata_id: row.get("metadata_id"),
            attributes: row.get("attributes"),
            expiration: row.get("expiration"),
        }
    }
}
