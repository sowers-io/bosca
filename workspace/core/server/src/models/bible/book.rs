use crate::models::bible::chapter::ChapterInput;
use crate::models::bible::reference::ReferenceInput;
use async_graphql::InputObject;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Book {
    pub metadata_id: Uuid,
    pub version: i32,
    pub usfm: String,
    pub name_short: String,
    pub name_long: String,
    pub abbreviation: String,
}

#[derive(InputObject)]
pub struct BookInput {
    pub reference: ReferenceInput,
    pub name_short: String,
    pub name_long: String,
    pub abbreviation: String,
    pub usx: String,
    pub chapters: Vec<ChapterInput>,
}

impl From<&Row> for Book {
    fn from(row: &Row) -> Self {
        Self {
            metadata_id: row.get("metadata_id"),
            version: row.get("version"),
            usfm: row.get("usfm"),
            name_short: row.get("name_short"),
            name_long: row.get("name_long"),
            abbreviation: row.get("abbreviation"),
        }
    }
}