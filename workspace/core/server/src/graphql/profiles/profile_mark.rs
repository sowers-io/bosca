use crate::context::BoscaContext;
use crate::graphql::content::collection::CollectionObject;
use crate::graphql::content::metadata::MetadataObject;
use crate::models::security::permission::PermissionAction;
use async_graphql::{Context, Error, Object};
use chrono::{DateTime, Utc};
use crate::models::profiles::profile_mark::ProfileMark;

pub struct ProfileMarkObject {
    mark: ProfileMark,
}

impl ProfileMarkObject {
    pub fn new(mark: ProfileMark) -> Self {
        Self { mark }
    }
}

#[Object(name = "ProfileMark")]
impl ProfileMarkObject {
    async fn id(&self) -> i64 {
        self.mark.id
    }

    async fn metadata(&self, ctx: &Context<'_>) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if let Some(metadata_id) = &self.mark.metadata_id {
            if let Some(version) = &self.mark.metadata_version {
                let metadata = ctx
                    .check_metadata_version_action(metadata_id, *version, PermissionAction::View)
                    .await?;
                return Ok(Some(MetadataObject::new(metadata)));
            }
        }
        Ok(None)
    }

    async fn collection(&self, ctx: &Context<'_>) -> Result<Option<CollectionObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if let Some(collection_id) = &self.mark.collection_id {
            let collection = ctx
                .check_collection_action(collection_id, PermissionAction::View)
                .await?;
            return Ok(Some(CollectionObject::new(collection)));
        }
        Ok(None)
    }

    async fn attributes(&self) -> &Option<serde_json::Value> {
        &self.mark.attributes
    }

    async fn created(&self) -> &DateTime<Utc> {
        &self.mark.created
    }
}
