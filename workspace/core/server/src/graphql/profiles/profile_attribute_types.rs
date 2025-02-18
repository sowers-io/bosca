use crate::context::BoscaContext;
use crate::graphql::profiles::profile_attribute_type::ProfileAttributeTypeObject;
use crate::models::profiles::profile_visibility::ProfileVisibility;
use async_graphql::*;

pub struct ProfileAttributeTypesObject {}

#[Object(name = "ProfileAttributeTypes")]
impl ProfileAttributeTypesObject {

    async fn all(&self, ctx: &Context<'_>) -> Result<Vec<ProfileAttributeTypeObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let is_admin = ctx.has_admin_account().await?;
        Ok(ctx
            .profile
            .get_attribute_types()
            .await?
            .into_iter()
            .filter(|a| {
                if is_admin {
                    true
                } else {
                    a.visibility == ProfileVisibility::Public
                }
            })
            .map(ProfileAttributeTypeObject::new)
            .collect())
    }
}
