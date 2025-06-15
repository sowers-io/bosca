use crate::context::BoscaContext;
use crate::graphql::security::group::GroupObject;
use crate::models::security::group_type::GroupType;
use async_graphql::{Context, Error, Object};
use uuid::Uuid;

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

    async fn edit_group(
        &self,
        ctx: &Context<'_>,
        id: String,
        name: String,
        description: String,
        group_type: GroupType,
    ) -> Result<GroupObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        let id = Uuid::parse_str(&id)?;
        let group = ctx
            .security
            .edit_group(&id, &name, &description, group_type)
            .await?;
        Ok(GroupObject::from(group))
    }

    async fn delete_group(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        let id = Uuid::parse_str(&id)?;
        ctx
            .security
            .delete_group(&id)
            .await?;
        Ok(true)
    }
}
