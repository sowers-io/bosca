use async_graphql::{Enum, InputObject};
use uuid::Uuid;
use crate::models::content::collection::CollectionType;

#[derive(Enum, Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum ExtensionFilterType {
    Document,
    DocumentTemplate,
    Guide,
    GuideTemplate,
    CollectionTemplate,
}

#[derive(InputObject)]
pub struct FindAttribute {
    pub key: String,
    pub value: String,
}

#[derive(InputObject)]
pub struct FindAttributes {
    pub attributes: Vec<FindAttribute>
}

#[derive(InputObject)]
pub struct FindQuery {
    pub attributes: Vec<FindAttributes>,
    pub content_types: Option<Vec<String>>,
    pub category_ids: Option<Vec<String>>,
    pub extension_filter: Option<ExtensionFilterType>,
    pub collection_type: Option<CollectionType>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl FindQuery {
    pub fn get_category_ids(&self) -> Option<Vec<Uuid>> {
        self.category_ids.clone().map(|category_ids| {
            category_ids
                .iter()
                .map(|id| Uuid::parse_str(id).unwrap())
                .collect()
        })
    }
}