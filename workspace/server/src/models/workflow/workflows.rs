use crate::worklfow::yaml::into;
use async_graphql::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_postgres::Row;
use yaml_rust2::Yaml;

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

impl From<&Yaml> for WorkflowInput {
    fn from(yaml: &Yaml) -> Self {
        Self {
            id: yaml["id"].as_str().unwrap_or("").to_string(),
            name: yaml["name"].as_str().unwrap_or("").to_string(),
            queue: yaml["queue"].as_str().unwrap().to_string(),
            description: yaml["description"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            configuration: into(&yaml["configuration"]),
        }
    }
}
