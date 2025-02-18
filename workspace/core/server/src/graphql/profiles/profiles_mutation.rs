use crate::context::BoscaContext;
use crate::graphql::profiles::profile::ProfileObject;
use crate::models::profiles::profile::ProfileInput;
use crate::models::profiles::profile_attribute_type::ProfileAttributeTypeInput;
use async_graphql::*;

pub struct ProfilesMutationObject {}

#[Object(name = "ProfilesMutation")]
impl ProfilesMutationObject {
    async fn edit_profile(
        &self,
        ctx: &Context<'_>,
        profile: ProfileInput,
    ) -> Result<Option<ProfileObject>> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("not authorized"));
        }
        let principal_id = ctx.principal.id;
        ctx.profile.edit(ctx, &principal_id, &profile).await?;
        Ok(ctx
            .profile
            .get_by_principal(&principal_id)
            .await?
            .map(ProfileObject::new))
    }

    async fn add_profile_attribute_type(
        &self,
        ctx: &Context<'_>,
        attribute: ProfileAttributeTypeInput,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        ctx.profile.add_profile_attribute_type(&attribute).await?;
        Ok(true)
    }

    async fn edit_profile_attribute_type(
        &self,
        ctx: &Context<'_>,
        attribute: ProfileAttributeTypeInput,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        ctx.profile.edit_profile_attribute_type(&attribute).await?;
        Ok(true)
    }

    async fn delete_profile_attribute_type(
        &self,
        ctx: &Context<'_>,
        attribute_id: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        ctx.profile.delete_profile_attribute_type(&attribute_id).await?;
        Ok(true)
    }
}
