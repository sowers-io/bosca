use crate::models::workflow::models::Model;
use async_graphql::{Error, Object};
use serde_json::Value;

pub struct ModelObject {
    model: Model,
}

impl ModelObject {
    pub fn new(model: Model) -> Self {
        Self { model }
    }
}

#[Object(name = "Model")]
impl ModelObject {
    async fn id(&self) -> Result<String, Error> {
        Ok(self.model.id.to_string())
    }

    #[graphql(name = "type")]
    async fn type_name(&self) -> &String {
        &self.model.model_type
    }

    async fn name(&self) -> &String {
        &self.model.name
    }

    async fn description(&self) -> &String {
        &self.model.description
    }

    async fn configuration(&self) -> &Value {
        &self.model.configuration
    }
}

impl From<Model> for ModelObject {
    fn from(model: Model) -> Self {
        Self::new(model)
    }
}
