use crate::context::{BoscaContext, PermissionCheck};
use crate::graphql::content::collection::CollectionObject;
use crate::graphql::content::metadata::MetadataObject;
use crate::models::profiles::profile_mark::ProfileMark;
use crate::models::security::permission::PermissionAction;
use async_graphql::{Context, Error, Object};
use chrono::{DateTime, Utc};

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
                let check = PermissionCheck::new_with_metadata_id_with_version(
                    *metadata_id,
                    *version,
                    PermissionAction::View,
                );
                let metadata = ctx.metadata_permission_check(check).await?;
                return Ok(Some(MetadataObject::new(metadata)));
            }
        }
        Ok(None)
    }

    async fn collection(&self, ctx: &Context<'_>) -> Result<Option<CollectionObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if let Some(collection_id) = &self.mark.collection_id {
            let check = PermissionCheck::new_with_collection_id(
                *collection_id,
                PermissionAction::View,
            );
            let collection = ctx.collection_permission_check(check).await?;
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
