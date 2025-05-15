use crate::models::bible::bible_language::BibleLanguageInput;
use crate::models::bible::book::BookInput;
use async_graphql::InputObject;
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;
use crate::models::bible::components::style::{Style, StyleInput};

pub struct Bible {
    pub metadata_id: Uuid,
    pub version: i32,
    pub variant: String,
    pub default_variant: bool,
    pub system_id: String,
    pub name: String,
    pub name_local: String,
    pub description: String,
    pub abbreviation: String,
    pub abbreviation_local: String,
    pub styles: Vec<Style>
}

#[derive(InputObject)]
pub struct BibleInput {
    pub system_id: String,
    pub variant: String,
    pub default_variant: bool,
    pub name: String,
    pub name_local: String,
    pub description: String,
    pub abbreviation: String,
    pub abbreviation_local: String,
    pub language: BibleLanguageInput,
    pub books: Vec<BookInput>,
    pub styles: Vec<StyleInput>
}

impl From<&Row> for Bible {
    fn from(row: &Row) -> Self {
        let styles: Value = row.get("styles");
        let styles: Vec<Style> = serde_json::from_value(styles).unwrap();
        Self {
            metadata_id: row.get("metadata_id"),
            version: row.get("version"),
            variant: row.get("variant"),
            default_variant: row.get("default_variant"),
            system_id: row.get("system_id"),
            name: row.get("name"),
            name_local: row.get("name_local"),
            description: row.get("description"),
            abbreviation: row.get("abbreviation"),
            abbreviation_local: row.get("abbreviation_local"),
            styles
        }
    }
}