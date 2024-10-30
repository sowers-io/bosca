use std::collections::HashSet;
use crate::models::workflow::execution_plan::WorkflowExecutionId;
use crate::queue::message::{Message, MessageValue};
use crate::queue::message_queues::{MessageQueueExecutor, MessageQueues};
use async_graphql::Error;
use chrono::{TimeDelta, Utc};
use deadpool_postgres::Transaction;
use log::{error, info, warn};
use std::ops::Add;
use crate::worklfow::set_complete::JobQueueSetComplete;

pub struct JobQueueDequeuer {}

#[async_trait::async_trait]
impl MessageQueueExecutor<MessageValue> for JobQueueDequeuer {
    async fn execute(
        &self,
        queues: &MessageQueues,
        txn: &Transaction<'_>,
        queue: &str,
        messages: &mut Vec<Message>,
    ) -> Result<Vec<MessageValue>, Error> {
        let mut plans = Vec::<MessageValue>::new();
        for message in messages {
            let mut dequeue = true;
            match &mut message.value {
                MessageValue::Plan(plan) => {
                    plan.plan_id = message.id;
                    if let Some(id) = &plan.next {
                        if !queues.exists(&id.queue, id.id).await? {
                            warn!(target: "workflow", "plan already has a next, but it is missing, marking complete: {}", plan.plan_id);
                            let completer = JobQueueSetComplete::new_with_job(id.clone());
                            let id = WorkflowExecutionId {
                                id: message.id,
                                queue: queue.to_owned(),
                            };
                            queues.update_message_txn(txn, &id, false, &completer).await?;
                            continue;
                        }
                        warn!(target: "workflow", "plan already has a next: {}", plan.plan_id);
                        message.visible_timeout = Utc::now().add(TimeDelta::minutes(15));
                        queues.update_message_raw_txn(txn, queue, false, message).await?;
                        dequeue = false;
                    } else {
                        if plan.current.is_empty() && plan.running.is_empty() {
                            if plan.pending.is_empty() {
                                error!(target: "workflow", "invalid plan state {}", plan.plan_id);
                                plan.error = Some("invalid plan state".to_string());
                                let id = WorkflowExecutionId {
                                    id: message.id,
                                    queue: queue.to_owned(),
                                };
                                queues.archive_txn(txn, &id).await?;
                                queues
                                    .update_message_raw_txn(txn, queue, true, message)
                                    .await?;
                                return Err(Error::new("invalid plan state"));
                            } else {
                                info!("updating plan current job: {}, {}", message.id, queue);
                                plan.current = plan.jobs.iter().filter(|job| job.workflow_activity.execution_group == 1).map(|job| job.id.clone()).collect();
                            }
                        }
                        if !plan.current.is_empty() {
                            info!(
                                "removing job from current list and queueing as next: {}, {}",
                                message.id, queue
                            );
                            let next = plan.current.remove(0);
                            plan.next = Some(next.clone());
                            message.visible_timeout = Utc::now().add(TimeDelta::minutes(30));
                        }
                        if plan.next.is_none() {
                            let mut running = HashSet::new();
                            if plan.running.is_empty() {
                                for id in plan.running.iter() {
                                    if queues.exists(&id.queue, id.id).await? {
                                        running.insert(id.clone());
                                    }
                                }
                            }
                            plan.running = running;
                            message.visible_timeout = Utc::now();
                            warn!(target: "workflow", "plan is missing next, not returning: {} {}", plan.plan_id, queue);
                            dequeue = false;
                        }
                        queues.update_message_raw_txn(txn, queue, false, message).await?;
                    }
                }
                MessageValue::Job(job) => {
                    info!(target: "workflow", "dequeuing job: {}, {}", message.id, queue);
                    job.id.id = message.id;
                    queues.update_message_raw_txn(txn, queue, false, message).await?;
                }
            }
            if dequeue {
                plans.push(message.value.clone());
            }
        }
        Ok(plans)
    }
}
