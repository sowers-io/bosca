use crate::context::BoscaContext;
use async_graphql::*;
use crate::graphql::profiles::profile::ProfileObject;
use crate::models::profiles::profile::ProfileInput;

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
        ctx.profile.edit(&principal_id, &profile).await?;
        Ok(ctx.profile.get_by_principal(&principal_id).await?.map(ProfileObject::new))
    }
}
