use crate::models::content::metadata_relationship::MetadataRelationship;
use async_graphql::{Context, Error, Object};
use serde_json::Value;
use crate::context::BoscaContext;
use crate::graphql::content::metadata::MetadataObject;

pub struct MetadataRelationshipObject {
    relationship: MetadataRelationship,
}

impl MetadataRelationshipObject {
    pub fn new(relationship: MetadataRelationship) -> Self {
        Self { relationship }
    }
}

#[Object(name = "MetadataRelationship")]
impl MetadataRelationshipObject {
    async fn id(&self) -> String {
        self.relationship.id2.to_string()
    }
    async fn metadata(&self, ctx: &Context<'_>) -> Result<MetadataObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let metadata = ctx.content.metadata.get(&self.relationship.id2).await?;
        if metadata.is_none() {
            return Err(Error::new("missing metadata"));
        }
        Ok(MetadataObject::new(metadata.unwrap()))
    }
    async fn relationship(&self) -> &String {
        &self.relationship.relationship
    }
    async fn attributes(&self) -> &Value {
        &self.relationship.attributes
    }
}

impl From<MetadataRelationship> for MetadataRelationshipObject {
    fn from(relationship: MetadataRelationship) -> Self {
        Self::new(relationship)
    }
}
