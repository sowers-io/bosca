use crate::context::BoscaContext;
use crate::graphql::profiles::profile::ProfileObject;
use crate::models::profiles::profile::ProfileInput;
use crate::models::profiles::profile_attribute_type::ProfileAttributeTypeInput;
use async_graphql::*;
use uuid::Uuid;
use crate::graphql::profiles::profile_mutation::ProfileMutationObject;

pub struct ProfilesMutationObject {}

#[Object(name = "ProfilesMutation")]
impl ProfilesMutationObject {
    async fn add(
        &self,
        ctx: &Context<'_>,
        profile: ProfileInput,
    ) -> Result<Option<ProfileObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        let id = ctx.profile.add(ctx, None, &profile, None).await?;
        Ok(ctx
            .profile
            .get_by_id(&id)
            .await?
            .map(ProfileObject::new))
    }

    async fn edit(
        &self,
        ctx: &Context<'_>,
        id: Option<String>,
        profile: ProfileInput,
    ) -> Result<Option<ProfileObject>> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("not authorized"));
        }
        if let Some(id) = id {
            ctx.check_has_admin_account().await?;
            let id = Uuid::parse_str(&id)?;
            ctx.profile.edit_by_id(ctx, &id, &profile).await?;
            Ok(ctx
                .profile
                .get_by_id(&id)
                .await?
                .map(ProfileObject::new))
        } else {
            let principal_id = ctx.principal.id;
            ctx.profile.edit_by_principal(ctx, &principal_id, &profile).await?;
            Ok(ctx
                .profile
                .get_by_principal(&principal_id)
                .await?
                .map(ProfileObject::new))
        }
    }

    async fn delete_attribute(
        &self,
        ctx: &Context<'_>,
        id: Option<String>,
        attribute_id: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if let Some(id) = id {
            ctx.check_has_service_account().await?;
            let id = Uuid::parse_str(&id)?;
            let attribute_id = Uuid::parse_str(&attribute_id)?;
            ctx.profile.delete_profile_attribute(&id, &attribute_id).await?;
        } else {
            let Some(profile) = ctx.profile.get_by_principal(&ctx.principal.id).await? else {
                return Err(Error::new("profile not found"));
            };
            let attribute_id = Uuid::parse_str(&attribute_id)?;
            ctx.profile.delete_profile_attribute(&profile.id, &attribute_id).await?;
        }
        Ok(true)
    }

    async fn add_attribute_type(
        &self,
        ctx: &Context<'_>,
        attribute: ProfileAttributeTypeInput,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        ctx.profile.add_profile_attribute_type(&attribute).await?;
        Ok(true)
    }

    async fn edit_attribute_type(
        &self,
        ctx: &Context<'_>,
        attribute: ProfileAttributeTypeInput,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        ctx.profile.edit_profile_attribute_type(&attribute).await?;
        Ok(true)
    }

    async fn delete_attribute_type(
        &self,
        ctx: &Context<'_>,
        attribute_id: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        ctx.profile.delete_profile_attribute_type(&attribute_id).await?;
        Ok(true)
    }

    async fn profile(&self, ctx: &Context<'_>) -> Result<ProfileMutationObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("unauthorized"));
        }
        let Some(profile) = ctx.profile.get_by_principal(&ctx.principal.id).await? else {
            return Err(Error::new("profile not found"));
        };
        Ok(ProfileMutationObject::new(profile))
    }
}
