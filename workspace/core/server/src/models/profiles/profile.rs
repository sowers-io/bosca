use crate::models::profiles::profile_attribute::ProfileAttributeInput;
use crate::models::profiles::profile_visibility::ProfileVisibility;
use async_graphql::InputObject;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Profile {
    pub id: Uuid,
    pub principal: Option<Uuid>,
    pub name: String,
    pub visibility: ProfileVisibility,
    pub collection_id: Option<Uuid>,
}

#[derive(InputObject, Debug, Clone, PartialEq, Eq)]
pub struct ProfileInput {
    pub slug: Option<String>,
    pub name: String,
    pub visibility: ProfileVisibility,
    pub attributes: Vec<ProfileAttributeInput>,
}

impl From<&Row> for Profile {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            principal: row.get("principal"),
            name: row.get("name"),
            visibility: row.get("visibility"),
            collection_id: row.get("collection_id"),
        }
    }
}
