use crate::models::profiles::profile_visibility::ProfileVisibility;
use async_graphql::InputObject;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;
use crate::models::content::comment_status::CommentStatus;

#[derive(Clone)]
pub struct Comment {
    pub parent_id: Option<i64>,
    pub id: i64,
    pub metadata_id: Option<Uuid>,
    pub version: Option<i32>,
    pub collection_id: Option<Uuid>,
    pub profile_id: Uuid,
    pub visibility: ProfileVisibility,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub status: CommentStatus,
    pub content: String,
    pub attributes: Option<Value>,
    pub system_attributes: Option<Value>,
    pub deleted: bool,
}

#[derive(InputObject, Clone, Serialize, Deserialize)]
pub struct CommentInput {
    pub parent_id: Option<i64>,
    pub visibility: ProfileVisibility,
    pub content: String,
    pub attributes: Option<Value>,
    pub system_attributes: Option<Value>,
}

impl From<&Row> for Comment {
    fn from(row: &Row) -> Self {
        Self {
            parent_id: row.get("parent_id"),
            id: row.get("id"),
            metadata_id: row.get("metadata_id"),
            version: row.get("version"),
            collection_id: row.get("collection_id"),
            profile_id: row.get("profile_id"),
            visibility: row.get("visibility"),
            created: row.get("created"),
            modified: row.get("modified"),
            status: row.get("status"),
            content: row.get("content"),
            attributes: row.get("attributes"),
            system_attributes: row.get("system_attributes"),
            deleted: row.get("deleted"),
        }
    }
}
