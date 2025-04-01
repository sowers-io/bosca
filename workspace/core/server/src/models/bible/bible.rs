use crate::models::bible::bible_language::BibleLanguageInput;
use crate::models::bible::book::BookInput;
use async_graphql::InputObject;
use tokio_postgres::Row;
use uuid::Uuid;

pub struct Bible {
    pub metadata_id: Uuid,
    pub version: i32,
    pub system_id: String,
    pub name: String,
    pub name_local: String,
    pub description: String,
    pub abbreviation: String,
    pub abbreviation_local: String,
}

#[derive(InputObject)]
pub struct BibleInput {
    pub system_id: String,
    pub name: String,
    pub name_local: String,
    pub description: String,
    pub abbreviation: String,
    pub abbreviation_local: String,
    pub language: BibleLanguageInput,
    pub books: Vec<BookInput>,
}

impl From<&Row> for Bible {
    fn from(row: &Row) -> Self {
        Self {
            metadata_id: row.get("metadata_id"),
            version: row.get("version"),
            system_id: row.get("system_id"),
            name: row.get("name"),
            name_local: row.get("name_local"),
            description: row.get("description"),
            abbreviation: row.get("abbreviation"),
            abbreviation_local: row.get("abbreviation_local"),
        }
    }
}