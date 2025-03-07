use async_graphql::{Context, Error, Object};
use crate::context::BoscaContext;
use crate::graphql::security::group::GroupObject;

pub struct GroupsObject {
}

#[Object(name = "Groups")]
impl GroupsObject {
    async fn all(&self, ctx: &Context<'_>, offset: i64, limit: i64) -> Result<Vec<GroupObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        Ok(ctx.security.get_groups(offset, limit).await?.into_iter().map(|g| g.into()).collect())
    }
}
