use chrono::{DateTime, Utc};
use tokio_postgres::Row;
use uuid::Uuid;


pub struct ProfileBookmark {
    pub id: i64,
    // pub profile_id: Uuid,
    pub metadata_id: Option<Uuid>,
    pub metadata_version: Option<i32>,
    pub collection_id: Option<Uuid>,
    pub created: DateTime<Utc>,
}

impl From<&Row> for ProfileBookmark {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            // profile_id: row.get("profile_id"),
            metadata_id: row.get("metadata_id"),
            metadata_version: row.get("metadata_version"),
            collection_id: row.get("collection_id"),
            created: row.get("created"),
        }
    }
}
