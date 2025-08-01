use crate::context::{BoscaContext, PermissionCheck};
use crate::graphql::content::metadata::MetadataObject;
use crate::models::content::collection_metadata_relationship::CollectionMetadataRelationship;
use crate::models::security::permission::PermissionAction;
use async_graphql::{Context, Error, Object};
use serde_json::Value;

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
        let check = PermissionCheck::new_with_metadata_id_advertised(
            self.relationship.metadata_id,
            PermissionAction::View,
        );
        let metadata = ctx.metadata_permission_check(check).await?;
        Ok(MetadataObject::new(metadata))
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
