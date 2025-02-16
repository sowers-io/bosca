use crate::context::BoscaContext;
use crate::graphql::profiles::profile_attribute::ProfileAttributeObject;
use crate::models::profiles::profile::Profile;
use crate::models::profiles::profile_visibility::ProfileVisibility;
use async_graphql::{Context, Error, Object};

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
    async fn id(&self, ctx: &Context<'_>) -> Result<Option<String>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(if self.profile.principal == ctx.principal.id || ctx.has_admin_account().await? {
            Some(self.profile.id.to_string())
        } else {
            None
        })
    }

    async fn slug(&self, ctx: &Context<'_>) -> Result<Option<String>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(
            if self.profile.principal == ctx.principal.id
                || self.profile.visibility == ProfileVisibility::Public
                || ctx.has_admin_account().await?
            {
                ctx.profile.get_slug(&self.profile.id).await?
            } else {
                None
            },
        )
    }

    async fn name(&self, ctx: &Context<'_>) -> Result<Option<String>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        // TODO: Filter things out based on who is looking at the profiles
        Ok(
            if self.profile.principal == ctx.principal.id
                || self.profile.visibility == ProfileVisibility::Public
                || ctx.has_admin_account().await?
            {
                Some(self.profile.name.clone())
            } else {
                None
            },
        )
    }

    async fn visibility(&self) -> &ProfileVisibility {
        &self.profile.visibility
    }

    async fn attributes(&self, ctx: &Context<'_>) -> Result<Vec<ProfileAttributeObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let attributes = ctx.profile.get_attributes(&self.profile.id).await?;
        if self.profile.principal == ctx.principal.id || ctx.has_admin_account().await? {
            Ok(attributes
                .into_iter()
                .filter(|a| a.visibility != ProfileVisibility::System)
                .map(ProfileAttributeObject::new)
                .collect())
        } else {
            // TODO: Filter things out based on who is looking at the profiles
            Ok(attributes
                .into_iter()
                .filter(|a| a.visibility == ProfileVisibility::Public)
                .map(ProfileAttributeObject::new)
                .collect())
        }
    }
}

impl From<Profile> for ProfileObject {
    fn from(profile: Profile) -> Self {
        Self::new(profile)
    }
}
