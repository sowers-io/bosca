use crate::context::BoscaContext;
use crate::graphql::profile::profile::ProfileObject;
use async_graphql::*;
use crate::graphql::profile::profile_attribute_types::ProfileAttributeTypesObject;

pub struct ProfilesObject {}

#[Object(name = "Profiles")]
impl ProfilesObject {

    async fn profile(&self, ctx: &Context<'_>) -> Result<Option<ProfileObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let principal_id = ctx.principal.id;
        Ok(ctx.profile.get_profile_by_principal(&principal_id).await?.map(ProfileObject::new))
    }

    async fn attribute_types(&self) -> ProfileAttributeTypesObject {
        ProfileAttributeTypesObject {}
    }
}
