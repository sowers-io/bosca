use crate::context::BoscaContext;
use crate::graphql::content::collection::CollectionObject;
use crate::graphql::content::comment::CommentObject;
use crate::graphql::content::metadata::MetadataObject;
use crate::graphql::profiles::profile::ProfileObject;
use crate::graphql::workflows::activity::ActivityObject;
use crate::graphql::workflows::workflow::WorkflowObject;
use crate::graphql::workflows::workflow_activity::WorkflowActivityObject;
use crate::graphql::workflows::workflow_activity_model::WorkflowActivityModelObject;
use crate::graphql::workflows::workflow_activity_prompt::WorkflowActivityPromptObject;
use crate::graphql::workflows::workflow_activity_storage_system::WorkflowActivityStorageSystemObject;
use crate::graphql::workflows::workflow_execution_id::WorkflowExecutionIdObject;
use crate::graphql::workflows::workflow_job_id::WorkflowJobIdObject;
use crate::models::workflow::execution_plan::WorkflowJob;
use async_graphql::{Context, Error, Object};
use serde_json::Value;
use uuid::Uuid;

pub struct WorkflowJobObject {
    job: WorkflowJob,
}

impl WorkflowJobObject {
    pub fn new(job: WorkflowJob) -> Self {
        Self { job }
    }
}

#[Object(name = "WorkflowJob")]
impl WorkflowJobObject {
    async fn parent(&self) -> Option<WorkflowJobIdObject> {
        self.job
            .parent
            .as_ref()
            .map(|p| WorkflowJobIdObject::new(p.clone()))
    }

    async fn plan_id(&self) -> WorkflowExecutionIdObject {
        WorkflowExecutionIdObject::new(self.job.plan_id.clone())
    }

    async fn id(&self) -> WorkflowJobIdObject {
        WorkflowJobIdObject::new(self.job.id.clone())
    }

    async fn workflow(&self, ctx: &Context<'_>) -> Result<WorkflowObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let workflow = ctx.workflow.get_workflow(&self.job.workflow_id).await?;
        Ok(workflow.unwrap().into())
    }

    async fn error(&self) -> &Option<String> {
        &self.job.error
    }

    async fn collection_id(&self) -> &Option<String> {
        &self.job.collection_id
    }

    async fn collection(&self, ctx: &Context<'_>) -> Result<Option<CollectionObject>, Error> {
        if self.job.collection_id.is_none() {
            return Ok(None);
        }
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(self.job.collection_id.clone().unwrap().as_str())?;
        Ok(ctx
            .content
            .collections
            .get(&id)
            .await?
            .map(CollectionObject::from))
    }

    async fn metadata(&self, ctx: &Context<'_>) -> Result<Option<MetadataObject>, Error> {
        if self.job.metadata_id.is_none() {
            return Ok(None);
        }
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(self.job.metadata_id.clone().unwrap().as_str())?;
        Ok(ctx
            .content
            .metadata
            .get(&id)
            .await?
            .map(MetadataObject::from))
    }

    async fn profile(&self, ctx: &Context<'_>) -> Result<Option<ProfileObject>, Error> {
        if self.job.profile_id.is_none() {
            return Ok(None);
        }
        let ctx = ctx.data::<BoscaContext>()?;
        let id = Uuid::parse_str(self.job.profile_id.clone().unwrap().as_str())?;
        Ok(ctx.profile.get_by_id(&id).await?.map(ProfileObject::from))
    }

    async fn comment(&self, ctx: &Context<'_>) -> Result<Option<CommentObject>, Error> {
        let Some(id) = self.job.comment_id else {
            return Ok(None);
        };
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(ctx
            .content
            .comments
            .get_metadata_comment_by_id(&id)
            .await?
            .map(|c| CommentObject::new(true, c)))
    }

    async fn metadata_version(&self) -> Option<i32> {
        self.job.metadata_version
    }

    async fn supplementary_id(&self) -> &Option<String> {
        &self.job.supplementary_id
    }

    async fn activity(&self) -> ActivityObject {
        ActivityObject::new(
            &self.job.activity,
            Some(self.job.activity_inputs.clone()),
            Some(self.job.activity_outputs.clone()),
        )
    }

    async fn children(&self) -> Vec<WorkflowExecutionIdObject> {
        self.job.children.iter().map(|p| p.into()).collect()
    }

    async fn completed_children(&self) -> Vec<WorkflowExecutionIdObject> {
        self.job
            .completed_children
            .iter()
            .map(|p| p.into())
            .collect()
    }

    async fn failed_children(&self) -> Vec<WorkflowExecutionIdObject> {
        self.job.failed_children.iter().map(|p| p.into()).collect()
    }

    async fn workflow_activity(&self) -> WorkflowActivityObject {
        WorkflowActivityObject::new(Some(self.job.clone()), &self.job.workflow_activity)
    }

    async fn prompts(&self) -> Vec<WorkflowActivityPromptObject> {
        self.job.prompts.iter().map(|p| p.clone().into()).collect()
    }

    async fn storage_systems(&self) -> Vec<WorkflowActivityStorageSystemObject> {
        self.job
            .storage_systems
            .iter()
            .map(|p| p.clone().into())
            .collect()
    }

    async fn failures(&self) -> i32 {
        self.job.failures
    }

    async fn models(&self) -> Vec<WorkflowActivityModelObject> {
        self.job.models.iter().map(|p| p.clone().into()).collect()
    }

    async fn context(&self) -> &Option<Value> {
        &self.job.context
    }
}

impl From<WorkflowJob> for WorkflowJobObject {
    fn from(job: WorkflowJob) -> Self {
        Self::new(job)
    }
}
