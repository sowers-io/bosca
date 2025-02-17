use crate::worklfow::yaml::into;
use async_graphql::*;
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;
use yaml_rust2::Yaml;

pub struct Model {
    pub id: Uuid,
    pub model_type: String,
    pub name: String,
    pub description: String,
    pub configuration: Value,
}

#[derive(InputObject)]
pub struct ModelInput {
    #[graphql(name = "type")]
    pub model_type: String,
    pub name: String,
    pub description: String,
    pub configuration: Value,
}

impl From<Row> for Model {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            model_type: row.get("type"),
            description: row.get("description"),
            configuration: row.get("configuration"),
        }
    }
}

impl From<&Yaml> for Model {
    fn from(yaml: &Yaml) -> Self {
        Self {
            id: if yaml["id"].is_null() || yaml["id"].is_badvalue() {
                Uuid::nil()
            } else {
                Uuid::parse_str(yaml["id"].as_str().unwrap()).unwrap()
            },
            name: yaml["name"].as_str().unwrap().to_string(),
            model_type: yaml["type"].as_str().unwrap().to_string(),
            description: yaml["description"].as_str().unwrap().to_string(),
            configuration: into(&yaml["configuration"]),
        }
    }
}

impl From<&Yaml> for ModelInput {
    fn from(yaml: &Yaml) -> Self {
        Self {
            name: yaml["name"].as_str().unwrap().to_string(),
            model_type: yaml["type"].as_str().unwrap().to_string(),
            description: yaml["description"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            configuration: into(&yaml["configuration"]),
        }
    }
}
