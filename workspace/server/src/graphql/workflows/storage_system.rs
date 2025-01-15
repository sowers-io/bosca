use crate::graphql::workflows::storage_system_model::StorageSystemModelObject;
use crate::models::workflow::storage_systems::{StorageSystem, StorageSystemType};
use async_graphql::{Context, Error, Object};
use serde_json::Value;
use crate::context::BoscaContext;

pub struct StorageSystemObject {
    system: StorageSystem,
}

impl StorageSystemObject {
    pub fn new(system: StorageSystem) -> Self {
        Self { system }
    }
}

#[Object(name = "StorageSystem")]
impl StorageSystemObject {
    async fn id(&self) -> Result<String, Error> {
        Ok(self.system.id.to_string())
    }

    #[graphql(name = "type")]
    async fn system_type(&self) -> StorageSystemType {
        self.system.system_type
    }

    async fn name(&self) -> &String {
        &self.system.name
    }

    async fn description(&self) -> &String {
        &self.system.description
    }

    async fn configuration(&self) -> &Option<Value> {
        &self.system.configuration
    }

    async fn models(&self, ctx: &Context<'_>) -> Result<Vec<StorageSystemModelObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx.workflow
            .get_storage_system_models(&self.system.id)
            .await?
            .into_iter()
            .map(StorageSystemModelObject::from)
            .collect())
    }
}

impl From<StorageSystem> for StorageSystemObject {
    fn from(system: StorageSystem) -> Self {
        Self::new(system)
    }
}
