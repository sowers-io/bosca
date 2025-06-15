use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;
use crate::models::security::group_type::GroupType;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub group_type: GroupType,
}

impl Group {
    #[allow(dead_code)]
    pub fn new(id: Uuid, name: String, description: String, group_type: GroupType) -> Self {
        Self { id, name, description, group_type }
    }
}

impl From<&Row> for Group {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            group_type: row.get("type"),
        }
    }
}
