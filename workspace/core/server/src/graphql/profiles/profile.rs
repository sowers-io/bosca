use crate::context::BoscaContext;
use crate::graphql::content::collection::CollectionObject;
use crate::graphql::profiles::profile_attribute::ProfileAttributeObject;
use crate::graphql::profiles::profile_bookmarks::ProfileBookmarksObject;
use crate::graphql::profiles::profile_guides::ProfileGuidesObject;
use crate::graphql::security::principal::PrincipalObject;
use crate::models::profiles::profile::Profile;
use crate::models::profiles::profile_visibility::ProfileVisibility;
use crate::models::security::permission::PermissionAction;
use async_graphql::{Context, Error, Object};
use crate::graphql::profiles::profile_marks::ProfileMarksObject;

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
    async fn id(&self, ctx: &Context<'_>) -> Result<String, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(
            if self.profile.principal == Some(ctx.principal.id) || ctx.has_admin_account().await? {
                self.profile.id.to_string()
            } else {
                "00000000-0000-0000-0000-000000000000".to_string()
            },
        )
    }

    async fn principal(&self, ctx: &Context<'_>) -> Result<Option<PrincipalObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if let Some(principal_id) = &self.profile.principal {
            if *principal_id == ctx.principal.id || ctx.has_admin_account().await? {
                let principal = ctx.security.get_principal_by_id(principal_id).await?;
                return Ok(Some(PrincipalObject::new(principal)));
            }
        }
        Ok(None)
    }

    async fn collection(&self, ctx: &Context<'_>) -> Result<Option<CollectionObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if let Some(collection_id) = &self.profile.collection_id {
            let Ok(collection) = ctx
                .check_collection_action(collection_id, PermissionAction::View)
                .await
            else {
                return Ok(None);
            };
            return Ok(Some(CollectionObject::new(collection)));
        }
        Ok(None)
    }

    async fn slug(&self, ctx: &Context<'_>) -> Result<Option<String>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(
            if self.profile.principal == Some(ctx.principal.id)
                || self.profile.visibility == ProfileVisibility::Public
                || ctx.has_admin_account().await?
            {
                ctx.profile.get_slug(&self.profile.id).await?
            } else {
                None
            },
        )
    }

    async fn name(&self, ctx: &Context<'_>) -> Result<String, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        // TODO: Filter things out based on who is looking at the profiles
        Ok(
            if self.profile.principal == Some(ctx.principal.id)
                || self.profile.visibility == ProfileVisibility::Public
                || ctx.has_admin_account().await?
            {
                self.profile.name.clone()
            } else {
                "".to_string()
            },
        )
    }

    async fn visibility(&self) -> &ProfileVisibility {
        &self.profile.visibility
    }

    async fn attributes(&self, ctx: &Context<'_>) -> Result<Vec<ProfileAttributeObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let attributes = ctx.profile.get_attributes(&self.profile.id).await?;
        if self.profile.principal == Some(ctx.principal.id) {
            Ok(attributes
                .into_iter()
                .filter(|a| a.visibility != ProfileVisibility::System)
                .map(ProfileAttributeObject::new)
                .collect())
        } else if ctx.has_service_account().await? || ctx.has_admin_account().await? {
            Ok(attributes
                .into_iter()
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

    async fn guides(&self, ctx: &Context<'_>) -> Result<ProfileGuidesObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let Some(principal) = self.profile.principal else {
            return Err(Error::new("No principal"));
        };
        if ctx.principal.id != principal {
            ctx.check_has_admin_account().await?;
        }
        Ok(ProfileGuidesObject::new(self.profile.clone()))
    }

    async fn bookmarks(&self) -> ProfileBookmarksObject {
        ProfileBookmarksObject::new(self.profile.clone())
    }

    async fn marks(&self) -> ProfileMarksObject {
        ProfileMarksObject::new(self.profile.clone())
    }
}

impl From<Profile> for ProfileObject {
    fn from(profile: Profile) -> Self {
        Self::new(profile)
    }
}
