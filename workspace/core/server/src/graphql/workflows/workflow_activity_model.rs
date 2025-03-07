use crate::graphql::workflows::model::ModelObject;
use crate::models::workflow::activities::WorkflowActivityModel;
use async_graphql::{Context, Error, Object};
use serde_json::Value;
use crate::context::BoscaContext;

pub struct WorkflowActivityModelObject {
    activity: WorkflowActivityModel,
}

impl WorkflowActivityModelObject {
    pub fn new(activity: WorkflowActivityModel) -> Self {
        Self { activity }
    }
}

#[Object(name = "WorkflowActivityModel")]
impl WorkflowActivityModelObject {
    async fn configuration(&self) -> &Option<Value> {
        &self.activity.configuration
    }

    async fn model(&self, ctx: &Context<'_>) -> Result<ModelObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ModelObject::new(
            ctx.workflow
                .get_model(&self.activity.model_id)
                .await?
                .unwrap(),
        ))
    }
}

impl From<WorkflowActivityModel> for WorkflowActivityModelObject {
    fn from(activity: WorkflowActivityModel) -> Self {
        Self::new(activity)
    }
}
