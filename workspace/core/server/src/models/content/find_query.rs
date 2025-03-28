use crate::models::content::collection::CollectionType;
use async_graphql::{Enum, InputObject, SimpleObject};
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

#[derive(InputObject, Clone, Serialize, Deserialize)]
pub struct FindQueryInput {
    pub attributes: Vec<FindAttributesInput>,
    pub content_types: Option<Vec<String>>,
    pub category_ids: Option<Vec<String>>,
    pub extension_filter: Option<ExtensionFilterType>,
    pub collection_type: Option<CollectionType>,
    pub ordering: Option<Vec<OrderingInput>>, // TODO: remove once we have indexes
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(InputObject, Clone, Serialize, Deserialize)]
pub struct FindQueriesInput {
    pub options: Vec<FindQueryOptionInput>,
}

#[derive(InputObject, Clone, Serialize, Deserialize)]
pub struct FindQueryOptionInput {
    pub name: String,
    pub query: FindQueryInput,
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

#[derive(SimpleObject, Serialize, Deserialize, Clone)]
pub struct FindQueries {
    pub options: Vec<FindQueryOption>,
}

#[derive(SimpleObject, Serialize, Deserialize, Clone)]
pub struct FindQueryOption {
    pub name: String,
    pub query: FindQuery,
}

#[derive(SimpleObject, Serialize, Deserialize, Clone)]
pub struct FindQuery {
    pub attributes: Vec<FindAttributes>,
    pub content_types: Option<Vec<String>>,
    pub category_ids: Option<Vec<String>>,
    pub extension_filter: Option<ExtensionFilterType>,
    pub collection_type: Option<CollectionType>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(SimpleObject, Serialize, Deserialize, Clone)]
pub struct FindAttribute {
    pub key: String,
    pub value: String,
}

#[derive(SimpleObject, Serialize, Deserialize, Clone)]
pub struct FindAttributes {
    pub attributes: Vec<FindAttribute>,
}
