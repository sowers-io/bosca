use async_graphql::*;
use tokio_postgres::Row;
use yaml_rust2::Yaml;

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

impl From<&Yaml> for Trait {
    fn from(yaml: &Yaml) -> Self {
        Self {
            id: yaml["id"].as_str().unwrap().to_string(),
            name: yaml["name"].as_str().unwrap().to_string(),
            description: yaml["description"].as_str().unwrap().to_string(),
            delete_workflow_id: None,
            workflow_ids: yaml["workflowIds"]
                .as_vec()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap().to_string())
                .collect(),
            content_types: Vec::new(),
        }
    }
}

impl From<&Yaml> for TraitInput {
    fn from(yaml: &Yaml) -> Self {
        Self {
            id: yaml["id"].as_str().unwrap_or("").to_string(),
            name: yaml["name"].as_str().unwrap_or("").to_string(),
            description: yaml["description"].as_str().unwrap_or("").to_string(),
            workflow_ids: if yaml["workflowIds"].is_null() || yaml["workflowIds"].is_badvalue() {
                Vec::new()
            } else {
                yaml["workflowIds"]
                    .as_vec()
                    .unwrap()
                    .iter()
                    .map(|x| x.as_str().unwrap().to_string())
                    .collect()
            },
            content_types: Vec::new(),
        }
    }
}
