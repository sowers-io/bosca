use crate::models::content::template_attribute::TemplateAttributeInput;
use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;
use crate::models::content::ordering::{Ordering, OrderingInput};

#[derive(Clone)]
pub struct CollectionTemplate {
    pub metadata_id: Uuid,
    pub version: i32,
    pub default_attributes: Option<Value>,
    pub configuration: Option<Value>,
    pub ordering: Option<Vec<Ordering>>,
    pub filters: Option<CollectionTemplateFilters>,
}

#[derive(SimpleObject, Clone, Serialize, Deserialize, Default)]
pub struct CollectionTemplateFilters {
    pub filters: Vec<CollectionTemplateFilter>
}

#[derive(SimpleObject, Clone, Serialize, Deserialize)]
pub struct CollectionTemplateFilter {
    pub name: String,
    pub filter: String,
}

#[derive(InputObject, Clone, Serialize, Deserialize)]
pub struct CollectionTemplateInput {
    pub attributes: Vec<TemplateAttributeInput>,
    pub default_attributes: Option<Value>,
    pub filters: Option<CollectionTemplateFiltersInput>,
    pub ordering: Option<Vec<OrderingInput>>,
    pub configuration: Option<Value>,
}

#[derive(InputObject, Clone, Serialize, Deserialize)]
pub struct CollectionTemplateFiltersInput {
    pub filters: Vec<CollectionTemplateFilterInput>
}

#[derive(InputObject, Clone, Serialize, Deserialize)]
pub struct CollectionTemplateFilterInput {
    pub name: String,
    pub filter: String,
}

impl From<&Row> for CollectionTemplate {
    fn from(row: &Row) -> Self {
        let ordering_values: Option<Value> = row.get("ordering");
        let filter_values: Option<Value> = row.get("filters");
        let filters: Option<CollectionTemplateFilters> = filter_values.map(|f| serde_json::from_value(f).unwrap_or_default());
        let ordering: Option<Vec<Ordering>> = ordering_values.map(|o| serde_json::from_value(o).unwrap_or_default());
        Self {
            metadata_id: row.get("metadata_id"),
            version: row.get("version"),
            default_attributes: row.get("default_attributes"),
            configuration: row.get("configuration"),
            ordering,
            filters,
        }
    }
}
