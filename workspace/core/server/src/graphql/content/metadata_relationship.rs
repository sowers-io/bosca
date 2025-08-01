use crate::context::{BoscaContext, PermissionCheck};
use crate::graphql::content::metadata::MetadataObject;
use crate::models::content::metadata_relationship::MetadataRelationship;
use crate::models::security::permission::PermissionAction;
use async_graphql::{Context, Error, Object};
use serde_json::Value;

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
        let check =
            PermissionCheck::new_with_metadata_id(self.relationship.id2, PermissionAction::View);
        let metadata = ctx.metadata_permission_check(check).await?;
        let check = PermissionCheck::new_with_metadata_advertised(metadata, PermissionAction::View);
        let metadata = ctx.metadata_permission_check(check).await?;
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
