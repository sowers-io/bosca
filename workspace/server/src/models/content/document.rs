use async_graphql::InputObject;
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Clone)]
pub struct Document {
    pub template_metadata_id : Option<Uuid>,
    pub template_metadata_version : Option<i32>,
    pub title: String,
    pub content: Value
}

#[derive(InputObject, Clone)]
pub struct DocumentInput {
    pub template_metadata_id : Option<String>,
    pub template_metadata_version : Option<i32>,
    pub title: String,
    pub content: Value,
}

impl From<&Row> for Document {
    fn from(row: &Row) -> Self {
        Self {
            template_metadata_id: row.get("template_metadata_id"),
            template_metadata_version: row.get("template_metadata_version"),
            title: row.get("title"),
            content: row.get("content"),
        }
    }
}