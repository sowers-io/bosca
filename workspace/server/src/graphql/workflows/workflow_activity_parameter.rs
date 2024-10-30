use crate::models::workflow::activities::WorkflowActivityParameter;
use async_graphql::Object;

pub struct WorkflowActivityParameterObject {
    parameter: WorkflowActivityParameter,
}

impl WorkflowActivityParameterObject {
    pub fn new(parameter: WorkflowActivityParameter) -> Self {
        Self { parameter }
    }
}

#[Object(name = "WorkflowActivityParameter")]
impl WorkflowActivityParameterObject {
    async fn name(&self) -> &String {
        &self.parameter.name
    }

    async fn value(&self) -> &String {
        &self.parameter.value
    }
}

impl From<WorkflowActivityParameter> for WorkflowActivityParameterObject {
    fn from(parameter: WorkflowActivityParameter) -> Self {
        Self::new(parameter)
    }
}
