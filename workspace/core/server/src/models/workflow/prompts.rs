use async_graphql::*;
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;

pub struct Prompt {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub system_prompt: String,
    pub user_prompt: String,
    pub input_type: String,
    pub output_type: String,
    pub schema: Option<Value>
}

#[derive(InputObject)]
pub struct PromptInput {
    pub name: String,
    pub description: String,
    pub system_prompt: String,
    pub user_prompt: String,
    pub input_type: String,
    pub output_type: String,
    pub schema: Option<Value>,
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
            schema: row.get("schema"),
        }
    }
}
