use async_graphql::*;
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;

pub struct MetadataRelationship {
    #[allow(dead_code)]
    pub id1: Uuid,
    pub id2: Uuid,
    pub relationship: String,
    pub attributes: Option<Value>,
}

#[derive(InputObject)]
pub struct MetadataRelationshipInput {
    pub id1: String,
    pub id2: String,
    pub relationship: Option<String>,
    pub attributes: Option<Value>,
}

impl From<&Row> for MetadataRelationship {
    fn from(row: &Row) -> Self {
        Self {
            id1: row.get("metadata1_id"),
            id2: row.get("metadata2_id"),
            relationship: row.get("relationship"),
            attributes: row.get("attributes"),
        }
    }
}
