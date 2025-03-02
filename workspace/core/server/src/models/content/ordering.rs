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
    pub path: Vec<String>,
    pub order: Order,
    #[graphql(name = "type")]
    #[serde(rename = "type")]
    pub attribute_type: AttributeType,
}

#[derive(InputObject, Clone, Serialize, Deserialize, Debug)]
pub struct OrderingInput {
    pub path: Vec<String>,
    pub order: Order,
    #[graphql(name = "type")]
    #[serde(rename = "type")]
    pub attribute_type: AttributeType,
}
