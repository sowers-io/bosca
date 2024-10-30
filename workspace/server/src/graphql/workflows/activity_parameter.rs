use crate::models::workflow::activities::{ActivityParameter, ActivityParameterType};
use async_graphql::Object;

pub struct ActivityParameterObject {
    parameter: ActivityParameter,
}

impl ActivityParameterObject {
    pub fn new(parameter: ActivityParameter) -> Self {
        Self { parameter }
    }
}

#[Object(name = "ActivityParameter")]
impl ActivityParameterObject {
    async fn name(&self) -> &String {
        &self.parameter.name
    }

    #[graphql(name = "type")]
    async fn parameter_type(&self) -> ActivityParameterType {
        self.parameter.parameter_type
    }
}
impl From<ActivityParameter> for ActivityParameterObject {
    fn from(parameter: ActivityParameter) -> Self {
        Self::new(parameter)
    }
}
