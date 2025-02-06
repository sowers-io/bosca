use crate::context::BoscaContext;
use crate::graphql::profiles::profile_attribute_type::ProfileAttributeTypeObject;
use crate::models::profiles::profile_visibility::ProfileVisibility;
use async_graphql::*;

pub struct ProfileAttributeTypesObject {}

#[Object(name = "ProfileAttributeTypes")]
impl ProfileAttributeTypesObject {
    async fn all(&self, ctx: &Context<'_>) -> Result<Vec<ProfileAttributeTypeObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .profile
            .get_profile_attribute_types()
            .await?
            .into_iter()
            .filter(|a| a.visibility != ProfileVisibility::System)
            .map(ProfileAttributeTypeObject::new)
            .collect())
    }
}
