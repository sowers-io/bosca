use crate::graphql::content::metadata::MetadataObject;
use crate::models::content::collection_metadata_relationship::CollectionMetadataRelationship;
use crate::models::content::metadata::Metadata;
use async_graphql::{Error, Object};
use serde_json::Value;

pub struct CollectionMetadataRelationshipObject {
    relationship: CollectionMetadataRelationship,
    metadata: Metadata,
}

impl CollectionMetadataRelationshipObject {
    pub fn new(relationship: CollectionMetadataRelationship, metadata: Metadata) -> Self {
        Self {
            relationship,
            metadata,
        }
    }
}

#[Object(name = "CollectionMetadataRelationship")]
impl CollectionMetadataRelationshipObject {
    async fn metadata(&self) -> Result<MetadataObject, Error> {
        Ok(MetadataObject::new(self.metadata.clone()))
    }
    async fn relationship(&self) -> &Option<String> {
        &self.relationship.relationship
    }
    async fn attributes(&self) -> &Option<Value> {
        &self.relationship.attributes
    }
}
