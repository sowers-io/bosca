use async_graphql::InputObject;
use chrono::{DateTime, Utc};
use tokio_postgres::Row;

#[derive(Clone)]
pub struct DocumentTemplate {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub allow_user_defined_blocks: bool,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
}

#[derive(InputObject)]
pub struct DocumentTemplateInput {
    pub name: String,
    pub description: String,
    pub allow_user_defined_blocks: bool,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
}

impl From<&Row> for DocumentTemplate {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("system_attributes"),
            allow_user_defined_blocks: row.get("allow_user_defined_blocks"),
            created: row.get("created"),
            modified: row.get("modified"),
        }
    }
}