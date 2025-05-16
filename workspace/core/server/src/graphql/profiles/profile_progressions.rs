use crate::context::BoscaContext;
use crate::graphql::profiles::profile_guide_progress::ProfileGuideProgressObject;
use crate::models::profiles::profile::Profile;
use async_graphql::{Context, Error, Object};
use uuid::Uuid;

pub struct ProfileGuideProgressionsObject {
    profile: Profile
}

impl ProfileGuideProgressionsObject {
    pub fn new(profile: Profile) -> Self {
        Self { profile }
    }
}

#[Object(name = "ProfileGuideProgressions")]
impl ProfileGuideProgressionsObject {
    async fn all(&self, ctx: &Context<'_>, offset: i64, limit: i64) -> Result<Vec<ProfileGuideProgressObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let progressions = ctx.profile.get_progressions(&self.profile.id, offset, limit).await?;
        Ok(progressions.into_iter().map(ProfileGuideProgressObject::new).collect())
    }

    async fn count(&self, ctx: &Context<'_>) -> Result<i64, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let count = ctx.profile.get_progression_count(&self.profile.id).await?;
        Ok(count)
    }

    async fn progress(&self, ctx: &Context<'_>, id: String, version: i32) -> Result<Option<ProfileGuideProgressObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(&id)?;
        let progression = ctx.profile.get_progress(&self.profile.id, &id, version).await?;
        Ok(progression.map(ProfileGuideProgressObject::new))
    }
}
