use crate::models::bible::bible_language::BibleLanguageInput;
use crate::models::bible::book::BookInput;
use crate::models::bible::components::style::{Style, Style2, StyleInput};
use async_graphql::InputObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    pub styles: Vec<Style>,
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
    pub styles: Vec<StyleInput>,
}

impl From<&Row> for Bible {
    fn from(row: &Row) -> Self {
        let styles: Value = row.get("styles");
        let result = serde_json::from_value::<Vec<Style>>(styles.clone());
        let styles = match result {
            Ok(result) => result,
            Err(_) => {
                let style2 = serde_json::from_value::<Vec<Style2>>(styles).unwrap();
                style2
                    .into_iter()
                    .map(|style| match style {
                        Style2::Declared(style) => Style::Declared(style),
                        Style2::Referenced(style) => Style::Referenced(style),
                    })
                    .collect()
            }
        };
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
            styles,
        }
    }
}
