use async_graphql::*;
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;

pub struct CollectionMetadataRelationship {
    #[allow(dead_code)]
    pub id: Uuid,
    pub metadata_id: Uuid,
    pub relationship: Option<String>,
    pub attributes: Option<Value>,
}

#[derive(InputObject)]
pub struct CollectionMetadataRelationshipInput {
    pub id: String,
    pub metadata_id: String,
    pub relationship: Option<String>,
    pub attributes: Option<Value>,
}

impl From<&Row> for CollectionMetadataRelationship {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("collection_id"),
            metadata_id: row.get("metadata_id"),
            relationship: row.get("relationship"),
            attributes: row.get("attributes"),
        }
    }
}
