use async_graphql::{Context, Error, Object};
use crate::context::BoscaContext;
use crate::graphql::profiles::profile::ProfileObject;
use crate::models::content::metadata_profile::MetadataProfile;

pub struct MetadataProfileObject {
    pub profile: MetadataProfile,
}

impl MetadataProfileObject {
    pub fn new(profile: MetadataProfile) -> Self {
        Self { profile }
    }
}

#[Object(name = "MetadataProfile")]
impl MetadataProfileObject {

    async fn relationship(&self) -> &String {
        &self.profile.relationship
    }

    async fn profile(&self, ctx: &Context<'_>) -> Result<Option<ProfileObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let profile = ctx.profile.get_by_id(&self.profile.profile_id).await?;
        Ok(profile.map(ProfileObject::new))
    }
}
