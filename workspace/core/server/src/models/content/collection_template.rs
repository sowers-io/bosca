use async_graphql::InputObject;
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;
use crate::models::content::collection_template_attributes::CollectionTemplateAttributeInput;

#[derive(Clone)]
pub struct CollectionTemplate {
    pub metadata_id: Uuid,
    pub version: i32,
    pub configuration: Option<Value>,
}

#[derive(InputObject, Clone)]
pub struct CollectionTemplateInput {
    pub attributes: Vec<CollectionTemplateAttributeInput>,
    pub configuration: Option<Value>,
}

impl From<&Row> for CollectionTemplate {
    fn from(row: &Row) -> Self {
        Self {
            metadata_id: row.get("metadata_id"),
            version: row.get("version"),
            configuration: row.get("configuration"),
        }
    }
}