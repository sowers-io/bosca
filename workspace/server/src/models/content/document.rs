use async_graphql::InputObject;
use tokio_postgres::Row;
use uuid::Uuid;
use crate::models::content::document_block::DocumentBlockInput;

#[derive(Clone)]
pub struct Document {
    pub template_metadata_id : Option<Uuid>,
    pub template_metadata_version : Option<i32>,
    pub title: String,
    pub allow_user_defined_blocks: bool,
}

#[derive(InputObject, Clone)]
pub struct DocumentInput {
    pub template_metadata_id : Option<String>,
    pub template_metadata_version : Option<i32>,
    pub title: String,
    pub allow_user_defined_blocks: bool,
    pub blocks: Vec<DocumentBlockInput>,
}

impl From<&Row> for Document {
    fn from(row: &Row) -> Self {
        Self {
            template_metadata_id: row.get("template_metadata_id"),
            template_metadata_version: row.get("template_metadata_version"),
            title: row.get("title"),
            allow_user_defined_blocks: row.get("allow_user_defined_blocks"),
        }
    }
}