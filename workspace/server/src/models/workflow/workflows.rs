use async_graphql::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_postgres::Row;
use crate::models::workflow::activities::WorkflowActivityInput;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: String,
    pub name: String,
    pub description: String,
    pub queue: String,
    pub configuration: Value,
}

#[derive(InputObject)]
pub struct WorkflowInput {
    pub id: String,
    pub name: String,
    pub description: String,
    pub queue: String,
    pub configuration: Value,
    pub activities: Vec<WorkflowActivityInput>,
}

impl From<Row> for Workflow {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            queue: row.get("queue"),
            description: row.get("description"),
            configuration: row.get("configuration"),
        }
    }
}
