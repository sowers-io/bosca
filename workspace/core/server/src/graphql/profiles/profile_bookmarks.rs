use crate::context::BoscaContext;
use crate::graphql::profiles::profile_bookmark::ProfileBookmarkObject;
use crate::models::profiles::profile::Profile;
use async_graphql::{Context, Error, Object};
use uuid::Uuid;

pub struct ProfileBookmarksObject {
    profile: Profile,
}

impl ProfileBookmarksObject {
    pub fn new(profile: Profile) -> Self {
        Self { profile }
    }
}

#[Object(name = "ProfileBookmarks")]
impl ProfileBookmarksObject {
    pub async fn count(&self, ctx: &Context<'_>) -> Result<i64, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.profile.get_bookmarks_count(&self.profile.id).await
    }

    pub async fn bookmarks(&self, ctx: &Context<'_>) -> Result<Vec<ProfileBookmarkObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let bookmarks = ctx.profile.get_bookmarks(&self.profile.id).await?;
        Ok(bookmarks
            .into_iter()
            .map(ProfileBookmarkObject::new)
            .collect())
    }

    pub async fn bookmark(
        &self,
        ctx: &Context<'_>,
        metadata_id: Option<String>,
        metadata_version: Option<i32>,
        collection_id: Option<String>,
    ) -> Result<Option<ProfileBookmarkObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .profile
            .get_bookmark(
                &self.profile.id,
                metadata_id.map(|m| Uuid::parse_str(&m).unwrap()),
                metadata_version,
                collection_id.map(|c| Uuid::parse_str(&c).unwrap()),
            )
            .await?
            .map(ProfileBookmarkObject::new))
    }
}
