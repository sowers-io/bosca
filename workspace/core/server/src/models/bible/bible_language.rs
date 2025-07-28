use async_graphql::InputObject;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BibleLanguage {
    pub iso: String,
    pub name: String,
    pub name_local: String,
    pub script: String,
    pub script_code: String,
    pub script_direction: String,
}

#[derive(InputObject)]
pub struct BibleLanguageInput {
    pub iso: String,
    pub name: String,
    pub name_local: String,
    pub script: String,
    pub script_code: String,
    pub script_direction: String,
}

impl From<&Row> for BibleLanguage {
    fn from(row: &Row) -> Self {
        Self {
            iso: row.get("iso"),
            name: row.get("name"),
            name_local: row.get("name_local"),
            script: row.get("script"),
            script_code: row.get("script_code"),
            script_direction: row.get("script_direction"),
        }
    }
}