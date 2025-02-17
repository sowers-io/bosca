use async_graphql::InputObject;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Clone)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
}

#[derive(InputObject, Clone)]
pub struct CategoryInput {
    pub name: String
}

impl From<&Row> for Category {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
        }
    }
}
