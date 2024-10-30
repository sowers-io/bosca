use crate::graphql::security::group::GroupObject;
use crate::models::security::permission::{Permission, PermissionAction};
use async_graphql::{Context, Error, Object};
use crate::context::BoscaContext;

pub struct PermissionObject {
    permission: Permission,
}

impl PermissionObject {
    pub fn new(permission: Permission) -> Self {
        Self { permission }
    }
}

#[Object(name = "Permission")]
impl PermissionObject {
    async fn group_id(&self) -> String {
        self.permission.group_id.to_string()
    }
    async fn group(&self, ctx: &Context<'_>) -> Result<GroupObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let group = ctx.security.get_group(&self.permission.group_id).await?;
        Ok(GroupObject::new(group))
    }
    async fn action(&self) -> PermissionAction {
        self.permission.action
    }
}

impl From<Permission> for PermissionObject {
    fn from(permission: Permission) -> Self {
        Self::new(permission)
    }
}
