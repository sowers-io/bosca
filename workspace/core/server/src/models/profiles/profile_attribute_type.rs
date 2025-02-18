use async_graphql::InputObject;
use crate::models::profiles::profile_visibility::ProfileVisibility;
use tokio_postgres::Row;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileAttributeType {
    pub id: String,
    pub name: String,
    pub description: String,
    pub visibility: ProfileVisibility,
}

#[derive(InputObject, Debug, Clone, PartialEq, Eq)]
pub struct ProfileAttributeTypeInput {
    pub id: String,
    pub name: String,
    pub description: String,
    pub visibility: ProfileVisibility,
}

impl From<&Row> for ProfileAttributeType {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            visibility: row.get("visibility"),
        }
    }
}
