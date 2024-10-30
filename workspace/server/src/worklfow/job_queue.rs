use crate::models::workflow::execution_plan::{
    WorkflowExecutionId, WorkflowExecutionPlan, WorkflowJobId,
};
use crate::queue::message::MessageValue;
use crate::queue::message_queues::MessageQueues;
use crate::worklfow::child_workflow_enqueuer::JobQueueChildWorkflowEnqueuer;
use crate::worklfow::dequeuer::JobQueueDequeuer;
use crate::worklfow::failer::JobQueueFailer;
use crate::worklfow::get::JobQueueGet;
use crate::worklfow::job_enqueuer::JobQueueEnqueuer;
use crate::worklfow::set_complete::JobQueueSetComplete;
use crate::worklfow::set_context::JobQueueSetContext;
use async_graphql::Error;
use serde_json::Value;
use crate::worklfow::set_checkin::JobCheckin;

#[derive(Clone)]
pub struct JobQueues {
    queues: MessageQueues,
}

impl JobQueues {
    pub fn new(queues: MessageQueues) -> Self {
        Self { queues }
    }

    pub async fn initialize(&self) -> Result<(), Error> {
        self.queues.initialize().await?;
        Ok(())
    }

    pub async fn create_queue(&self, name: &String) -> Result<(), Error> {
        self.queues.create_queue(name).await?;
        Ok(())
    }

    pub async fn get_all(
        &self,
        queue: &str,
        offset: i64,
        limit: i64,
        archived: bool,
    ) -> Result<Vec<MessageValue>, Error> {
        let executor = JobQueueGet {};
        self.queues
            .get_messages(queue, offset, limit, archived, &executor)
            .await
    }

    pub async fn get(
        &self,
        id: &WorkflowExecutionId,
        archived: bool,
    ) -> Result<Option<MessageValue>, Error> {
        let executor = JobQueueGet {};
        Ok(self
            .queues
            .get_message(id, archived, &executor)
            .await?
            .first()
            .cloned())
    }

    pub async fn enqueue(&self, queue: &str, message: &MessageValue) -> Result<i64, Error> {
        self.queues.enqueue(queue, message).await
    }

    pub async fn exists(&self, queue: &str, id: i64) -> Result<bool, Error> {
        self.queues.exists(queue, id).await
    }

    pub async fn enqueue_job_child_workflows(
        &self,
        job_id: &WorkflowExecutionId,
        plans: &[WorkflowExecutionPlan],
    ) -> Result<Vec<WorkflowExecutionId>, Error> {
        let executor = JobQueueChildWorkflowEnqueuer::new(plans);
        self.queues
            .enqueue_multi_with_executor(job_id, &executor)
            .await
    }

    pub async fn enqueue_execution_job(
        &self,
        plan_id: &WorkflowExecutionId,
        job_index: i32,
    ) -> Result<Option<WorkflowExecutionId>, Error> {
        let executor = JobQueueEnqueuer::new(job_index);
        self.queues.enqueue_with_executor(plan_id, &executor).await
    }

    pub async fn dequeue(&self, queue: &String) -> Result<Option<MessageValue>, Error> {
        let executor = JobQueueDequeuer {};
        let messages: Vec<MessageValue> = self.queues.dequeue(queue, 3600, 1, &executor).await?;
        if messages.is_empty() {
            return Ok(None)
        }
        Ok(messages.into_iter().next())
    }

    pub async fn set_execution_plan_context(
        &self,
        plan_id: &WorkflowExecutionId,
        context: &Value,
    ) -> Result<(), Error> {
        let set_context = JobQueueSetContext::new(context);
        self.queues
            .update_message(plan_id, false, &set_context)
            .await?;
        Ok(())
    }

    pub async fn set_execution_job_context(
        &self,
        job_id: &WorkflowExecutionId,
        context: &Value,
    ) -> Result<(), Error> {
        let set_context = JobQueueSetContext::new(context);
        self.queues
            .update_message(job_id, false, &set_context)
            .await?;
        Ok(())
    }

    pub async fn set_execution_plan_job_failed(
        &self,
        job_id: &WorkflowJobId,
        error: &str,
    ) -> Result<(), Error> {
        let failer = JobQueueFailer::new(error);
        let id = WorkflowExecutionId {
            id: job_id.id,
            queue: job_id.queue.clone(),
        };
        self.queues.update_message(&id, false, &failer).await?;
        Ok(())
    }

    pub async fn set_execution_plan_job_checkin(
        &self,
        job_id: &WorkflowJobId,
    ) -> Result<(), Error> {
        let checkin = JobCheckin::new();
        let id = WorkflowExecutionId {
            id: job_id.id,
            queue: job_id.queue.clone(),
        };
        self.queues.update_message(&id, false, &checkin).await?;
        Ok(())
    }

    pub async fn set_execution_plan_job_complete(
        &self,
        job_id: &WorkflowJobId,
    ) -> Result<(), Error> {
        let completer = JobQueueSetComplete::new();
        let id = WorkflowExecutionId {
            id: job_id.id,
            queue: job_id.queue.clone(),
        };
        self.queues.update_message(&id, false, &completer).await?;
        Ok(())
    }
}
