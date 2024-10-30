use crate::models::workflow::storage_system_models::StorageSystemModel;
use async_graphql::Object;
use serde_json::Value;

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

    async fn configuration(&self) -> &Value {
        &self.model.configuration
    }
}

impl From<StorageSystemModel> for StorageSystemModelObject {
    fn from(model: StorageSystemModel) -> Self {
        Self::new(model)
    }
}
