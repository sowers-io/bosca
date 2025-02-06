use crate::models::profile::profile::Profile;
use crate::models::profile::profile_visibility::ProfileVisibility;
use async_graphql::{Context, Error, Object};
use crate::context::BoscaContext;
use crate::graphql::profile::profile_attribute::ProfileAttributeObject;

pub struct ProfileObject {
    profile: Profile,
}

impl ProfileObject {
    pub fn new(profile: Profile) -> Self {
        Self { profile }
    }
}

#[Object(name = "Profile")]
impl ProfileObject {
    async fn id(&self) -> String {
        self.profile.id.to_string()
    }

    async fn name(&self) -> &String {
        &self.profile.name
    }

    async fn visibility(&self) -> &ProfileVisibility {
        &self.profile.visibility
    }

    async fn attributes(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<ProfileAttributeObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let attributes = ctx.profile.get_profile_attributes(&self.profile.id).await?;
        // TODO: Filter things out based on who is looking at the profile
        Ok(attributes.into_iter().filter(|a| a.visibility != ProfileVisibility::System ).map(ProfileAttributeObject::new).collect())
    }
}
