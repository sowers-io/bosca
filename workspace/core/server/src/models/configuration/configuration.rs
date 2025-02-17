use async_graphql::InputObject;
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;
use crate::models::security::permission::PermissionInput;

#[derive(Clone)]
pub struct Configuration {
    pub id: Uuid,
    pub key: String,
    pub description: String,
}

#[derive(InputObject, Clone)]
pub struct ConfigurationInput {
    pub key: String,
    pub description: String,
    pub value: Value,
    pub permissions: Vec<PermissionInput>,
}

impl From<&Row> for Configuration {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            key: row.get("key"),
            description: row.get("description"),
        }
    }
}
