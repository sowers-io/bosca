use crate::models::bible::components::component::{Component, ComponentInput};
use crate::models::bible::reference::{Reference, ReferenceInput};
use async_graphql::InputObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_postgres::Row;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chapter {
    pub reference: Reference,
    pub component: Component,
}

#[derive(InputObject)]
pub struct ChapterInput {
    pub reference: ReferenceInput,
    pub component: ComponentInput,
}

impl From<&Row> for Chapter {
    fn from(row: &Row) -> Self {
        let j: Value = row.get("components");
        let c: Component = serde_json::from_value(j).unwrap();
        Self {
            reference: Reference {
                usfm: row.get("reference"),
            },
            component: c,
        }
    }
}
