use crate::graphql::workflows::storage_system::StorageSystemObject;
use crate::models::workflow::activities::WorkflowActivityStorageSystem;
use async_graphql::{Context, Error, Object};
use serde_json::Value;
use crate::context::BoscaContext;

pub struct WorkflowActivityStorageSystemObject {
    system: WorkflowActivityStorageSystem,
}

impl WorkflowActivityStorageSystemObject {
    pub fn new(system: WorkflowActivityStorageSystem) -> Self {
        Self { system }
    }
}

#[Object(name = "WorkflowActivityStorageSystem")]
impl WorkflowActivityStorageSystemObject {
    async fn configuration(&self) -> &Value {
        &self.system.configuration
    }
    async fn system(&self, ctx: &Context<'_>) -> Result<StorageSystemObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(StorageSystemObject::new(
            ctx.workflow
                .get_storage_system(&self.system.system_id)
                .await?
                .unwrap(),
        ))
    }
}

impl From<WorkflowActivityStorageSystem> for WorkflowActivityStorageSystemObject {
    fn from(system: WorkflowActivityStorageSystem) -> Self {
        Self::new(system)
    }
}
