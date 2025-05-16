use crate::context::BoscaContext;
use crate::models::profiles::profile::Profile;
use async_graphql::{Context, Error, Object};
use uuid::Uuid;
use crate::graphql::profiles::profile_guide_history::ProfileGuideHistoryObject;

pub struct ProfileGuideHistoriesObject {
    profile: Profile,
}

impl ProfileGuideHistoriesObject {
    pub fn new(profile: Profile) -> Self {
        Self { profile }
    }
}

#[Object(name = "ProfileGuideHistories")]
impl ProfileGuideHistoriesObject {
    async fn all(&self, ctx: &Context<'_>, offset: i64, limit: i64) -> Result<Vec<ProfileGuideHistoryObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let histories = ctx.profile.get_histories(&self.profile.id, offset, limit).await?;
        Ok(histories.into_iter().map(ProfileGuideHistoryObject::new).collect())
    }

    async fn count(&self, ctx: &Context<'_>) -> Result<i64, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let count = ctx.profile.get_histories_count(&self.profile.id).await?;
        Ok(count)
    }

    async fn history(&self, ctx: &Context<'_>, id: String, version: i32, offset: i64, limit: i64) -> Result<Vec<ProfileGuideHistoryObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(&id)?;
        let history = ctx.profile.get_history(&self.profile.id, &id, version, offset, limit).await?;
        Ok(history.into_iter().map(ProfileGuideHistoryObject::new).collect())
    }

    async fn history_count(&self, ctx: &Context<'_>, id: String, version: i32) -> Result<i64, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(&id)?;
        let count = ctx.profile.get_history_count(&self.profile.id, &id, version).await?;
        Ok(count)
    }
}
