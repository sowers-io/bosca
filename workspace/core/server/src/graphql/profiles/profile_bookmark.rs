use crate::context::BoscaContext;
use crate::graphql::content::collection::CollectionObject;
use crate::graphql::content::metadata::MetadataObject;
use crate::models::security::permission::PermissionAction;
use async_graphql::{Context, Error, Object};
use chrono::{DateTime, Utc};
use crate::models::profiles::profile_bookmark::ProfileBookmark;

pub struct ProfileBookmarkObject {
    bookmark: ProfileBookmark,
}

impl ProfileBookmarkObject {
    pub fn new(bookmark: ProfileBookmark) -> Self {
        Self { bookmark }
    }
}

#[Object(name = "ProfileBookmark")]
impl ProfileBookmarkObject {
    async fn id(&self) -> i64 {
        self.bookmark.id
    }

    async fn metadata(&self, ctx: &Context<'_>) -> Result<Option<MetadataObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if let Some(metadata_id) = &self.bookmark.metadata_id {
            if let Some(version) = &self.bookmark.metadata_version {
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
        if let Some(collection_id) = &self.bookmark.collection_id {
            let collection = ctx
                .check_collection_action(collection_id, PermissionAction::View)
                .await?;
            return Ok(Some(CollectionObject::new(collection)));
        }
        Ok(None)
    }

    async fn created_at(&self) -> &DateTime<Utc> {
        &self.bookmark.created
    }
}
