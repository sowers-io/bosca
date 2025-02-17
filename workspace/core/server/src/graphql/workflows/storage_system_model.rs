use crate::models::workflow::storage_system_models::StorageSystemModel;
use async_graphql::{Context, Error, Object};
use serde_json::Value;
use crate::context::BoscaContext;
use crate::graphql::workflows::model::ModelObject;

pub struct StorageSystemModelObject {
    model: StorageSystemModel,
}

impl StorageSystemModelObject {
    pub fn new(model: StorageSystemModel) -> Self {
        Self { model }
    }
}

#[Object(name = "StorageSystemModel")]
impl StorageSystemModelObject {
    async fn model_id(&self) -> String {
        self.model.model_id.to_string()
    }

    async fn model(&self, ctx: &Context<'_>) -> Result<Option<ModelObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let model = ctx.workflow.get_model(&self.model.model_id).await?;
        Ok(model.map(|m| m.into()))
    }

    async fn configuration(&self) -> &Value {
        &self.model.configuration
    }
}

impl From<StorageSystemModel> for StorageSystemModelObject {
    fn from(model: StorageSystemModel) -> Self {
        Self::new(model)
    }
}
