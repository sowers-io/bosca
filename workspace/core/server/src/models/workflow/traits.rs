use async_graphql::*;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trait {
    pub id: String,
    pub name: String,
    pub description: String,
    pub delete_workflow_id: Option<String>,
    pub workflow_ids: Vec<String>,
    pub content_types: Vec<String>,
}

#[derive(InputObject)]
pub struct TraitInput {
    pub id: String,
    pub name: String,
    pub description: String,
    pub workflow_ids: Vec<String>,
    pub delete_workflow_id: Option<String>,
    pub content_types: Vec<String>,
}

impl From<&Row> for Trait {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            delete_workflow_id: row.get("delete_workflow_id"),
            workflow_ids: Vec::new(),
            content_types: Vec::new(),
        }
    }
}
