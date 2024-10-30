use async_graphql::*;
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;

pub struct Source {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub configuration: Value,
}

#[derive(InputObject)]
pub struct SourceInput {
    pub name: String,
    pub description: String,
    pub configuration: Value,
}

impl From<&Row> for Source {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            configuration: row.get("configuration"),
        }
    }
}
