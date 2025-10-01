use crate::context::BoscaContext;
use crate::graphql::profiles::profile::ProfileObject;
use crate::graphql::profiles::profile_attribute_types::ProfileAttributeTypesObject;
use async_graphql::*;
use uuid::Uuid;

pub struct ProfilesObject {}

#[Object(name = "Profiles")]
impl ProfilesObject {
    async fn all(
        &self,
        ctx: &Context<'_>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<ProfileObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        Ok(ctx
            .profile
            .get_all(offset, limit)
            .await?
            .into_iter()
            .map(ProfileObject::new).collect())
    }

    async fn find_profiles(&self, ctx: &Context<'_>, type_id: String, attribute: String, value: String) -> Result<Vec<ProfileObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        Ok(ctx
            .profile
            .find_by_attribute(&type_id, &attribute, &value)
            .await?
            .into_iter()
            .map(ProfileObject::new).collect())
    }

    async fn profile(&self, ctx: &Context<'_>, id: String) -> Result<Option<ProfileObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        let id = Uuid::parse_str(&id)?;
        Ok(ctx
            .profile
            .get_by_id(&id)
            .await?
            .map(ProfileObject::new))
    }

    async fn current(&self, ctx: &Context<'_>) -> Result<Option<ProfileObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let principal_id = ctx.principal.id;
        Ok(ctx
            .profile
            .get_by_principal(&principal_id)
            .await?
            .map(ProfileObject::new))
    }

    async fn attribute_types(&self) -> ProfileAttributeTypesObject {
        ProfileAttributeTypesObject {}
    }
}
