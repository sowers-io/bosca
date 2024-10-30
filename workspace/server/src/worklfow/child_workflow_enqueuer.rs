use crate::models::workflow::execution_plan::{
    WorkflowExecutionId, WorkflowExecutionPlan, WorkflowJobId,
};
use crate::queue::message::{Message, MessageValue};
use crate::queue::message_queues::{MessageQueueExecutor, MessageQueues};
use async_graphql::Error;
use deadpool_postgres::Transaction;
use log::info;

pub struct JobQueueChildWorkflowEnqueuer {
    plans: Vec<WorkflowExecutionPlan>,
}

impl JobQueueChildWorkflowEnqueuer {
    pub fn new(plans: &[WorkflowExecutionPlan]) -> Self {
        Self {
            plans: plans.to_vec(),
        }
    }
}

#[async_trait::async_trait]
impl MessageQueueExecutor<WorkflowExecutionId> for JobQueueChildWorkflowEnqueuer {
    async fn execute(
        &self,
        queues: &MessageQueues,
        txn: &Transaction<'_>,
        queue: &str,
        messages: &mut Vec<Message>,
    ) -> Result<Vec<WorkflowExecutionId>, Error> {
        let mut jobs = Vec::<WorkflowExecutionId>::new();
        for message in messages {
            match &mut message.value {
                MessageValue::Job(job) => {
                    info!(target: "workflow", "enqueuing job children: {}, {}", message.id, queue);
                    if job.complete {
                        return Err(Error::new("job is already complete"));
                    }
                    let plan_parent = WorkflowExecutionId {
                        id: message.id,
                        queue: queue.to_owned(),
                    };
                    for plan in self.plans.clone().iter_mut() {
                        plan.parent = Some(plan_parent.clone());
                        let new_plan_queue = plan.workflow.queue.clone();
                        let new_plan = MessageValue::Plan(plan.clone());
                        let enqueued_id =
                            queues.enqueue_txn(txn, &new_plan_queue, &new_plan).await?;
                        let new_id = WorkflowExecutionId {
                            id: enqueued_id,
                            queue: new_plan_queue.clone(),
                        };
                        jobs.push(new_id.clone());
                        job.children.insert(new_id.clone());
                        let plan_updater = JobQueueChildWorkflowPlanEnqueuer {
                            job_id: job.id.clone(),
                            child_plan_id: new_id,
                        };
                        queues
                            .update_message_txn(txn, &job.execution_plan, false, &plan_updater)
                            .await?;
                    }
                    queues
                        .update_message_raw_txn(txn, queue, false, message)
                        .await?;
                }
                _ => panic!("unsupported, can only edit jobs"),
            }
        }
        Ok(jobs)
    }
}

struct JobQueueChildWorkflowPlanEnqueuer {
    job_id: WorkflowJobId,
    child_plan_id: WorkflowExecutionId,
}

#[async_trait::async_trait]
impl MessageQueueExecutor<Message> for JobQueueChildWorkflowPlanEnqueuer {
    async fn execute(
        &self,
        queues: &MessageQueues,
        txn: &Transaction<'_>,
        queue: &str,
        messages: &mut Vec<Message>,
    ) -> Result<Vec<Message>, Error> {
        for message in messages {
            match &mut message.value {
                MessageValue::Plan(plan) => {
                    let job = plan.jobs.get_mut(self.job_id.index as usize).unwrap();
                    job.children.insert(self.child_plan_id.clone());
                    queues
                        .update_message_raw_txn(txn, queue, false, message)
                        .await?;
                }
                _ => panic!("unsupported, can only edit plans"),
            }
        }
        Ok(Vec::<Message>::new())
    }
}
