use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
}

impl Group {
    #[allow(dead_code)]
    pub fn new(id: Uuid, name: String) -> Self {
        Self { id, name }
    }
}

impl From<&Row> for Group {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
        }
    }
}
