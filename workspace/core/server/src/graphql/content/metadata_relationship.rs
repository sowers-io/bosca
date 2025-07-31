use crate::models::content::metadata_relationship::MetadataRelationship;
use async_graphql::{Context, Error, Object};
use serde_json::Value;
use crate::context::BoscaContext;
use crate::graphql::content::metadata::MetadataObject;
use crate::models::security::permission::PermissionAction;

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
        let metadata = ctx.check_metadata_action(&self.relationship.id2, PermissionAction::View).await?;
        ctx.check_metadata_action_2(&metadata, PermissionAction::View).await?;
        Ok(MetadataObject::new(metadata))
    }
    async fn relationship(&self) -> &String {
        &self.relationship.relationship
    }
    async fn attributes(&self) -> &Option<Value> {
        &self.relationship.attributes
    }
}

impl From<MetadataRelationship> for MetadataRelationshipObject {
    fn from(relationship: MetadataRelationship) -> Self {
        Self::new(relationship)
    }
}
