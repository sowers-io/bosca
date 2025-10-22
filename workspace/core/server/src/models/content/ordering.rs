use async_graphql::{Enum, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use crate::models::content::attribute_type::AttributeType;
use crate::models::content::attribute_location::AttributeLocation;

#[derive(Enum, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum Order {
    #[serde(alias = "ASCENDING")]
    #[serde(alias = "ascending")]
    Ascending,
    #[default]
    #[serde(alias = "DESCENDING")]
    #[serde(alias = "descending")]
    Descending,
}


#[derive(SimpleObject, Clone, Serialize, Deserialize, Debug, Default)]
pub struct Ordering {
    pub field: Option<String>,
    pub path: Option<Vec<String>>,
    pub order: Order,
    #[graphql(name = "type")]
    #[serde(rename = "type")]
    pub attribute_type: Option<AttributeType>,
    #[graphql(name = "location")]
    #[serde(rename = "location")]
    pub attribute_location: Option<AttributeLocation>,
}

impl Ordering {
    pub fn get_field(&self) -> Option<&String> {
        if let Some(field) = &self.field {
            if field != "created" && field != "modified" && field != "slug" && field != "name" {
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
    #[graphql(name = "location")]
    #[serde(rename = "location")]
    pub attribute_location: Option<AttributeLocation>,
}
