use crate::context::BoscaContext;
use crate::graphql::security::group::GroupObject;
use crate::models::security::permission::{Permission, PermissionAction};
use async_graphql::{Context, Error, Object};

pub struct ConfigurationPermissionObject {
    permission: Permission,
}

impl ConfigurationPermissionObject {
    pub fn new(permission: Permission) -> Self {
        Self { permission }
    }
}

#[Object(name = "ConfigurationPermission")]
impl ConfigurationPermissionObject {
    async fn group(&self, ctx: &Context<'_>) -> Result<GroupObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(GroupObject::new(ctx.security.get_group(&self.permission.group_id).await?))
    }

    async fn action(&self) -> PermissionAction {
        self.permission.action
    }
}
