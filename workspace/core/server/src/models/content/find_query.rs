use crate::models::content::collection::CollectionType;
use async_graphql::{Enum, InputObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::content::ordering::OrderingInput;

#[derive(Enum, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
pub enum ExtensionFilterType {
    Document,
    DocumentTemplate,
    Guide,
    GuideTemplate,
    CollectionTemplate,
}

#[derive(InputObject, Clone, Serialize, Deserialize)]
pub struct FindAttributeInput {
    pub key: String,
    pub value: String,
}

#[derive(InputObject, Clone, Serialize, Deserialize)]
pub struct FindAttributesInput {
    pub attributes: Vec<FindAttributeInput>,
}

#[derive(InputObject, Clone, Serialize, Deserialize, Default)]
pub struct FindQueryInput {
    pub attributes: Vec<FindAttributesInput>,
    pub content_types: Option<Vec<String>>,
    pub language_tags: Option<Vec<String>>,
    pub category_ids: Option<Vec<String>>,
    pub trait_ids: Option<Vec<String>>,
    pub extension_filter: Option<ExtensionFilterType>,
    pub collection_type: Option<CollectionType>,
    pub ordering: Option<Vec<OrderingInput>>, // TODO: remove once we have indexes
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl FindQueryInput {
    pub fn get_category_ids(&self) -> Option<Vec<Uuid>> {
        self.category_ids.clone().map(|category_ids| {
            category_ids
                .iter()
                .map(|id| Uuid::parse_str(id).unwrap())
                .collect()
        })
    }
}