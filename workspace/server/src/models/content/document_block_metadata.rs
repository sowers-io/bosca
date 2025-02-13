use async_graphql::InputObject;
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Clone)]
pub struct DocumentBlockMetadata {
    pub metadata_id: Uuid,
    pub attributes: Option<Value>,
}

#[derive(InputObject, Clone)]
pub struct DocumentBlockMetadataInput {
    pub metadata_id: String,
    pub attributes: Option<Value>,
}

impl From<&Row> for DocumentBlockMetadata {
    fn from(row: &Row) -> Self {
        Self {
            metadata_id: row.get("metadata_reference_id"),
            attributes: row.get("attributes"),
        }
    }
}