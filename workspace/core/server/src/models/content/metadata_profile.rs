use async_graphql::*;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

pub struct MetadataProfile {
    pub profile_id: Uuid,
    pub relationship: String,
}

#[derive(InputObject, Clone, Serialize, Deserialize)]
pub struct MetadataProfileInput {
    pub profile_id: String,
    pub relationship: String,
}

impl From<&Row> for MetadataProfile {
    fn from(row: &Row) -> Self {
        Self {
            profile_id: row.get("profile_id"),
            relationship: row.get("relationship"),
        }
    }
}
