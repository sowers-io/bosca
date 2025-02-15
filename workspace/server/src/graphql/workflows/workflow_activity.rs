use crate::context::BoscaContext;
use crate::graphql::workflows::workflow_activity_model::WorkflowActivityModelObject;
use crate::graphql::workflows::workflow_activity_parameter::WorkflowActivityParameterObject;
use crate::models::workflow::activities::WorkflowActivity;
use crate::models::workflow::execution_plan::WorkflowJob;
use async_graphql::{Context, Error, Object};
use serde_json::Value;
use crate::graphql::workflows::workflow_activity_prompt::WorkflowActivityPromptObject;
use crate::graphql::workflows::workflow_activity_storage_system::WorkflowActivityStorageSystemObject;

pub struct WorkflowActivityObject {
    job: Option<WorkflowJob>,
    activity: WorkflowActivity,
}

impl WorkflowActivityObject {
    pub fn new(job: Option<WorkflowJob>, activity: &WorkflowActivity) -> Self {
        Self {
            job,
            activity: activity.clone(),
        }
    }
}

#[Object(name = "WorkflowActivity")]
impl WorkflowActivityObject {
    async fn id(&self) -> i64 {
        self.activity.id
    }

    async fn activity_id(&self) -> &String {
        &self.activity.activity_id
    }

    async fn queue(&self) -> &String {
        &self.activity.queue
    }

    async fn execution_group(&self) -> i32 {
        self.activity.execution_group
    }

    async fn configuration(&self) -> &Option<Value> {
        &self.activity.configuration
    }

    async fn models(&self, ctx: &Context<'_>) -> Result<Vec<WorkflowActivityModelObject>, Error> {
        let models = if let Some(job) = &self.job {
            job.models.clone()
        } else {
            let ctx = ctx.data::<BoscaContext>()?;
            ctx.workflow
                .get_workflow_activity_models(&self.activity.id)
                .await?
        };
        Ok(models
            .iter()
            .map(|m| WorkflowActivityModelObject::new(m.clone()))
            .collect())
    }

    async fn prompts(&self, ctx: &Context<'_>) -> Result<Vec<WorkflowActivityPromptObject>, Error> {
        let prompts = if let Some(job) = &self.job {
            job.prompts.clone()
        } else {
            let ctx = ctx.data::<BoscaContext>()?;
            ctx.workflow
                .get_workflow_activity_prompts(&self.activity.id)
                .await?
        };
        Ok(prompts
            .iter()
            .map(|m| WorkflowActivityPromptObject::new(m.clone()))
            .collect())
    }

    async fn storage_systems(&self, ctx: &Context<'_>) -> Result<Vec<WorkflowActivityStorageSystemObject>, Error> {
        let storage_systems = if let Some(job) = &self.job {
            job.storage_systems.clone()
        } else {
            let ctx = ctx.data::<BoscaContext>()?;
            ctx.workflow
                .get_workflow_activity_storage_systems(&self.activity.id)
                .await?
        };
        Ok(storage_systems
            .iter()
            .map(|m| WorkflowActivityStorageSystemObject::new(m.clone()))
            .collect())
    }

    async fn inputs(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<WorkflowActivityParameterObject>, Error> {
        let inputs = if let Some(job) = &self.job {
            job.workflow_inputs.clone()
        } else {
            let ctx = ctx.data::<BoscaContext>()?;
            ctx.workflow
                .get_workflow_activity_inputs(&self.activity.id)
                .await?
        };
        Ok(inputs
            .iter()
            .map(|p| WorkflowActivityParameterObject::new(p.clone()))
            .collect())
    }

    async fn outputs(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<WorkflowActivityParameterObject>, Error> {
        let inputs = if let Some(job) = &self.job {
            job.workflow_outputs.clone()
        } else {
            let ctx = ctx.data::<BoscaContext>()?;
            ctx.workflow
                .get_workflow_activity_outputs(&self.activity.id)
                .await?
        };
        Ok(inputs
            .iter()
            .map(|p| WorkflowActivityParameterObject::new(p.clone()))
            .collect())
    }
}
