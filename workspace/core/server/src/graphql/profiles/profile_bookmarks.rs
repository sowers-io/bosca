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
        if self.profile.principal.is_none() || self.profile.principal != Some(ctx.principal.id) {
            return Ok(0);
        }
        ctx.profile_bookmarks.get_count(&self.profile.id).await
    }

    pub async fn bookmarks(
        &self,
        ctx: &Context<'_>,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<Vec<ProfileBookmarkObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if self.profile.principal.is_none() || self.profile.principal != Some(ctx.principal.id) {
            return Ok(Vec::new());
        }
        let bookmarks = ctx
            .profile_bookmarks
            .get_all(&self.profile.id, offset.unwrap_or(0), limit.unwrap_or(25))
            .await?;
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
        if self.profile.principal.is_none() || self.profile.principal != Some(ctx.principal.id) {
            return Ok(None);
        }
        Ok(ctx
            .profile_bookmarks
            .get(
                &self.profile.id,
                metadata_id.map(|m| Uuid::parse_str(&m).unwrap()),
                metadata_version,
                collection_id.map(|c| Uuid::parse_str(&c).unwrap()),
            )
            .await?
            .map(ProfileBookmarkObject::new))
    }
}
