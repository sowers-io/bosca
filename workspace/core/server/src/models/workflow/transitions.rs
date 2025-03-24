use async_graphql::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use yaml_rust2::Yaml;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transition {
    pub from_state_id: String,
    pub to_state_id: String,
    pub description: String,
}

#[derive(InputObject)]
pub struct TransitionInput {
    pub from_state_id: String,
    pub to_state_id: String,
    pub description: String,
}

#[derive(InputObject, Clone)]
pub struct BeginTransitionInput {
    pub collection_id: Option<String>,
    pub metadata_id: Option<String>,
    pub version: Option<i32>,
    pub state_id: String,
    pub state_valid: Option<DateTime<Utc>>,
    pub status: String,
    pub supplementary_id: Option<String>,
    pub wait_for_completion: Option<bool>,
    pub restart: Option<bool>
}

impl From<Row> for Transition {
    fn from(row: Row) -> Self {
        Self {
            from_state_id: row.get("from_state_id"),
            to_state_id: row.get("to_state_id"),
            description: row.get("description"),
        }
    }
}

impl From<&Yaml> for Transition {
    fn from(yaml: &Yaml) -> Self {
        Self {
            from_state_id: yaml["from"].as_str().unwrap().to_string(),
            to_state_id: yaml["to"].as_str().unwrap().to_string(),
            description: yaml["description"].as_str().unwrap().to_string(),
        }
    }
}

impl From<&Yaml> for TransitionInput {
    fn from(yaml: &Yaml) -> Self {
        Self {
            from_state_id: yaml["from"].as_str().unwrap().to_string(),
            to_state_id: yaml["to"].as_str().unwrap().to_string(),
            description: yaml["description"].as_str().unwrap().to_string(),
        }
    }
}
