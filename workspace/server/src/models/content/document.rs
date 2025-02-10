use async_graphql::InputObject;
use tokio_postgres::Row;
use crate::models::content::document_block::DocumentBlockInput;

#[derive(Clone)]
pub struct Document {
    pub template_id : Option<i64>,
    pub title: String,
    pub allow_user_defined_blocks: bool,
}

#[derive(InputObject, Clone)]
pub struct DocumentInput {
    pub template_id : Option<i64>,
    pub title: String,
    pub allow_user_defined_blocks: bool,
    pub blocks: Vec<DocumentBlockInput>,
}

impl From<&Row> for Document {
    fn from(row: &Row) -> Self {
        Self {
            template_id: row.get("template_id"),
            title: row.get("title"),
            allow_user_defined_blocks: row.get("allow_user_defined_blocks"),
        }
    }
}