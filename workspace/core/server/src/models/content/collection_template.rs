use async_graphql::InputObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;
use crate::models::content::template_attribute::TemplateAttributeInput;
use crate::models::content::find_query::{FindQueries, FindQueriesInput};

#[derive(Clone)]
pub struct CollectionTemplate {
    pub metadata_id: Uuid,
    pub version: i32,
    pub default_attributes: Option<Value>,
    pub configuration: Option<Value>,
    pub metadata_filter: Option<FindQueries>,
    pub collection_filter: Option<FindQueries>,
}

#[derive(InputObject, Clone, Serialize, Deserialize)]
pub struct CollectionTemplateInput {
    pub attributes: Vec<TemplateAttributeInput>,
    pub default_attributes: Option<Value>,
    pub metadata_filter: Option<FindQueriesInput>,
    pub collection_filter: Option<FindQueriesInput>,
    pub configuration: Option<Value>,
}

impl From<&Row> for CollectionTemplate {
    fn from(row: &Row) -> Self {
        let collection_filter: Option<Value> = row.get("collection_filter");
        let collection_filter: Option<FindQueries> = collection_filter.map(|f| serde_json::from_value(f).unwrap());
        let metadata_filter: Option<Value> = row.get("metadata_filter");
        let metadata_filter: Option<FindQueries> = metadata_filter.map(|f| serde_json::from_value(f).unwrap());
        Self {
            metadata_id: row.get("metadata_id"),
            version: row.get("version"),
            default_attributes: row.get("default_attributes"),
            configuration: row.get("configuration"),
            collection_filter,
            metadata_filter,
        }
    }
}