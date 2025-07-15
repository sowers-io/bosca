use crate::context::BoscaContext;
use crate::models::profiles::profile::Profile;
use async_graphql::{Context, Error, Object};
use uuid::Uuid;
use crate::graphql::profiles::profile_mark::ProfileMarkObject;

pub struct ProfileMarksObject {
    profile: Profile,
}

impl ProfileMarksObject {
    pub fn new(profile: Profile) -> Self {
        Self { profile }
    }
}

#[Object(name = "ProfileMarks")]
impl ProfileMarksObject {
    pub async fn count(&self, ctx: &Context<'_>) -> Result<i64, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if self.profile.principal.is_none() || self.profile.principal != Some(ctx.principal.id) {
            return Ok(0);
        }
        ctx.profile_marks.get_count(&self.profile.id).await
    }

    pub async fn marks(
        &self,
        ctx: &Context<'_>,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<Vec<ProfileMarkObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if self.profile.principal.is_none() || self.profile.principal != Some(ctx.principal.id) {
            return Ok(Vec::new());
        }
        let marks = ctx
            .profile_marks
            .get_all(&self.profile.id, offset.unwrap_or(0), limit.unwrap_or(25))
            .await?;
        Ok(marks
            .into_iter()
            .map(ProfileMarkObject::new)
            .collect())
    }

    pub async fn mark(
        &self,
        ctx: &Context<'_>,
        metadata_id: Option<String>,
        metadata_version: Option<i32>,
        collection_id: Option<String>,
    ) -> Result<Option<ProfileMarkObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if self.profile.principal.is_none() || self.profile.principal != Some(ctx.principal.id) {
            return Ok(None);
        }
        Ok(ctx
            .profile_marks
            .get(
                &self.profile.id,
                metadata_id.map(|m| Uuid::parse_str(&m).unwrap()),
                metadata_version,
                collection_id.map(|c| Uuid::parse_str(&c).unwrap()),
            )
            .await?
            .map(ProfileMarkObject::new))
    }
}
