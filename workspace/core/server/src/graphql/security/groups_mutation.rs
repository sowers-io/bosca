use crate::context::BoscaContext;
use crate::graphql::security::group::GroupObject;
use crate::models::security::group_type::GroupType;
use async_graphql::{Context, Error, Object};

pub struct GroupsMutation {}

#[Object(name = "GroupsMutation")]
impl GroupsMutation {
    async fn add_group(
        &self,
        ctx: &Context<'_>,
        name: String,
        description: String,
        group_type: GroupType,
    ) -> Result<GroupObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        let group = ctx
            .security
            .add_group(&name, &description, group_type)
            .await?;
        Ok(GroupObject::from(group))
    }
}
