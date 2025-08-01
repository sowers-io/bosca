use crate::graphql::content::metadata::MetadataObject;
use crate::models::content::metadata::Metadata;
use crate::models::content::metadata_relationship::MetadataRelationship;
use async_graphql::Object;
use serde_json::Value;

pub struct MetadataRelationshipObject {
    relationship: MetadataRelationship,
    metadata: Metadata,
}

impl MetadataRelationshipObject {
    pub fn new(relationship: MetadataRelationship, metadata: Metadata) -> Self {
        Self {
            relationship,
            metadata,
        }
    }
}

#[Object(name = "MetadataRelationship")]
impl MetadataRelationshipObject {
    async fn id(&self) -> String {
        self.relationship.id2.to_string()
    }
    async fn metadata(&self) -> MetadataObject {
        MetadataObject::new(self.metadata.clone())
    }
    async fn relationship(&self) -> &String {
        &self.relationship.relationship
    }
    async fn attributes(&self) -> &Option<Value> {
        &self.relationship.attributes
    }
}
