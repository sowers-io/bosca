use async_graphql::{Enum, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use crate::models::content::attribute_type::AttributeType;

#[derive(Enum, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Order {
    Ascending,
    Descending,
}

#[derive(SimpleObject, Clone, Serialize, Deserialize, Debug)]
pub struct Ordering {
    pub field: Option<String>,
    pub path: Option<Vec<String>>,
    pub order: Order,
    #[graphql(name = "type")]
    #[serde(rename = "type")]
    pub attribute_type: Option<AttributeType>,
}

impl Ordering {
    pub fn get_field(&self) -> Option<&String> {
        if let Some(field) = &self.field {
            if field != "created" {
                return None;
            }
            return Some(field);
        }
        None
    }
}

#[derive(InputObject, Clone, Serialize, Deserialize, Debug)]
pub struct OrderingInput {
    pub field: Option<String>,
    pub path: Option<Vec<String>>,
    pub order: Order,
    #[graphql(name = "type")]
    #[serde(rename = "type")]
    pub attribute_type: Option<AttributeType>,
}
