use crate::models::content::document_block_type::DocumentBlockType;
use async_graphql::InputObject;
use tokio_postgres::Row;

pub struct DocumentTemplateBlock {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub block_type: DocumentBlockType,
    pub sort: i32
}

#[derive(InputObject)]
pub struct DocumentTemplateBlockInput {
    pub name: String,
    pub description: String,
    #[graphql(name = "type")]
    pub block_type: DocumentBlockType,
    pub sort: i32
}

impl From<&Row> for DocumentTemplateBlock {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            block_type: row.get("type"),
            sort: row.get("sort"),
        }
    }
}