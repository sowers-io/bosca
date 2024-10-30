use crate::graphql::workflows::activity_parameter::ActivityParameterObject;
use crate::models::workflow::activities::{Activity, ActivityParameter};
use async_graphql::{Context, Error, Object};
use serde_json::Value;
use crate::context::BoscaContext;

pub struct ActivityObject {
    activity: Activity,
    input: Option<Vec<ActivityParameter>>,
    output: Option<Vec<ActivityParameter>>,
}

impl ActivityObject {
    pub fn new(
        activity: &Activity,
        input: Option<Vec<ActivityParameter>>,
        output: Option<Vec<ActivityParameter>>,
    ) -> Self {
        Self {
            activity: activity.clone(),
            input,
            output,
        }
    }
}

#[Object(name = "Activity")]
impl ActivityObject {
    async fn id(&self) -> Result<String, Error> {
        Ok(self.activity.id.to_string())
    }

    async fn name(&self) -> &String {
        &self.activity.name
    }

    async fn description(&self) -> &String {
        &self.activity.description
    }

    async fn child_workflow_id(&self) -> &Option<String> {
        &self.activity.child_workflow_id
    }

    async fn configuration(&self) -> &Value {
        &self.activity.configuration
    }
    async fn inputs(&self, ctx: &Context<'_>) -> Result<Vec<ActivityParameterObject>, Error> {
        Ok(match &self.input {
            Some(input) => input.iter().map(|p| p.clone().into()).collect(),
            None => {
                let ctx = ctx.data::<BoscaContext>()?;
                let inputs = ctx.workflow.get_activity_inputs(&self.activity.id).await?;
                inputs.into_iter().map(|p| p.into()).collect()
            }
        })
    }
    async fn outputs(&self, ctx: &Context<'_>) -> Result<Vec<ActivityParameterObject>, Error> {
        Ok(match &self.output {
            Some(output) => output.iter().map(|p| p.clone().into()).collect(),
            None => {
                let ctx = ctx.data::<BoscaContext>()?;
                let inputs = ctx.workflow.get_activity_outputs(&self.activity.id).await?;
                inputs.into_iter().map(|p| p.into()).collect()
            }
        })
    }
}
