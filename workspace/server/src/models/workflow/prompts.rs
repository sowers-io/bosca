use async_graphql::*;
use tokio_postgres::Row;
use uuid::Uuid;
use yaml_rust2::Yaml;

pub struct Prompt {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub system_prompt: String,
    pub user_prompt: String,
    pub input_type: String,
    pub output_type: String,
}

#[derive(InputObject)]
pub struct PromptInput {
    pub name: String,
    pub description: String,
    pub system_prompt: String,
    pub user_prompt: String,
    pub input_type: String,
    pub output_type: String,
}

impl From<&Row> for Prompt {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            system_prompt: row.get("system_prompt"),
            user_prompt: row.get("user_prompt"),
            input_type: row.get("input_type"),
            output_type: row.get("output_type"),
        }
    }
}

impl From<&Yaml> for Prompt {
    fn from(yaml: &Yaml) -> Self {
        Self {
            id: if yaml["id"].is_null() || yaml["id"].is_badvalue() {
                Uuid::nil()
            } else {
                Uuid::parse_str(yaml["id"].as_str().unwrap()).unwrap()
            },
            name: yaml["name"].as_str().unwrap().to_string(),
            description: yaml["description"].as_str().unwrap().to_string(),
            system_prompt: yaml["systemPrompt"].as_str().unwrap().to_string(),
            user_prompt: yaml["userPrompt"].as_str().unwrap().to_string(),
            input_type: yaml["inputType"].as_str().unwrap().to_string(),
            output_type: yaml["outputType"].as_str().unwrap().to_string(),
        }
    }
}

impl From<&Yaml> for PromptInput {
    fn from(yaml: &Yaml) -> Self {
        Self {
            name: yaml["name"].as_str().unwrap_or("").to_string(),
            description: yaml["description"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            system_prompt: yaml["systemPrompt"].as_str().unwrap().to_string(),
            user_prompt: yaml["userPrompt"].as_str().unwrap().to_string(),
            input_type: yaml["inputType"]
                .as_str()
                .unwrap_or("text/plain")
                .to_string(),
            output_type: yaml["outputType"]
                .as_str()
                .unwrap_or("text/plain")
                .to_string(),
        }
    }
}
