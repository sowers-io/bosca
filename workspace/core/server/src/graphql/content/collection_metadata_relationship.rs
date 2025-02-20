use async_graphql::{Context, Error, Object};
use serde_json::Value;
use crate::context::BoscaContext;
use crate::graphql::content::metadata::MetadataObject;
use crate::models::content::collection_metadata_relationship::CollectionMetadataRelationship;

pub struct CollectionMetadataRelationshipObject {
    relationship: CollectionMetadataRelationship,
}

impl CollectionMetadataRelationshipObject {
    pub fn new(relationship: CollectionMetadataRelationship) -> Self {
        Self { relationship }
    }
}

#[Object(name = "CollectionMetadataRelationship")]
impl CollectionMetadataRelationshipObject {
    async fn metadata(&self, ctx: &Context<'_>) -> Result<MetadataObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata = ctx.content.metadata.get(&self.relationship.metadata_id).await?;
        if metadata.is_none() {
            return Err(Error::new("missing metadata"));
        }
        Ok(MetadataObject::new(metadata.unwrap()))
    }
    async fn relationship(&self) -> &Option<String> {
        &self.relationship.relationship
    }
    async fn attributes(&self) -> &Option<Value> {
        &self.relationship.attributes
    }
}

impl From<CollectionMetadataRelationship> for CollectionMetadataRelationshipObject {
    fn from(relationship: CollectionMetadataRelationship) -> Self {
        Self::new(relationship)
    }
}
