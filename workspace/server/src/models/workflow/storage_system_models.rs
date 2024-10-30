use crate::worklfow::yaml::into;
use async_graphql::*;
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;
use yaml_rust2::Yaml;

pub struct StorageSystemModel {
    pub model_id: Uuid,
    pub configuration: Value,
}

#[derive(InputObject)]
pub struct StorageSystemModelInput {
    pub model_id: String,
    pub configuration: Value,
}

impl From<Row> for StorageSystemModel {
    fn from(row: Row) -> Self {
        Self {
            model_id: row.get("model_id"),
            configuration: row.get("configuration"),
        }
    }
}

impl From<&Yaml> for StorageSystemModelInput {
    fn from(yaml: &Yaml) -> Self {
        Self {
            model_id: "".to_string(),
            configuration: into(&yaml["configuration"]),
        }
    }
}
