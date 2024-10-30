use std::collections::HashSet;
use async_graphql::{Context, Error, Object};
use chrono::{DateTime, Utc};
use crate::context::BoscaContext;
use crate::models::workflow::execution_plan::WorkflowExecutionId;
use crate::queue::message::MessageObject;
use crate::queue::message_queues::MessageQueueStats;

pub struct MessageQueueObject {
    queue: String,
}

pub struct MessageQueueStatsObject {
    stats: MessageQueueStats,
}

#[Object(name = "MessageQueueStats")]
impl MessageQueueStatsObject {
    async fn size(&self) -> i64 {
        self.stats.size
    }

    async fn pending(&self) -> i64 {
        self.stats.pending
    }

    async fn available(&self) -> i64 {
        self.stats.available
    }

    async fn min(&self) -> &Option<DateTime<Utc>> {
        &self.stats.min
    }

    async fn max(&self) -> &Option<DateTime<Utc>> {
        &self.stats.max
    }
}

#[Object(name = "MessageQueue")]
impl MessageQueueObject {
    async fn name(&self) -> &String {
        &self.queue
    }

    async fn stats(&self, ctx: &Context<'_>) -> Result<MessageQueueStatsObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(MessageQueueStatsObject { stats: ctx.messages.get_queue_stats(&self.queue, false).await? })
    }

    async fn archived_stats(&self, ctx: &Context<'_>) -> Result<MessageQueueStatsObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        Ok(MessageQueueStatsObject { stats: ctx.messages.get_queue_stats(&self.queue, true).await? })
    }
}

pub struct QueuesObject {}

#[Object(name = "Queues")]
impl QueuesObject {
    async fn message_queues(&self, ctx: &Context<'_>) -> Result<Vec<MessageQueueObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let admin_group = ctx.security.get_administrators_group().await?;
        if !ctx.principal.has_group(&admin_group.id) {
            return Err(Error::new("invalid permissions"));
        }
        let mut queues = HashSet::new();
        for workflow in ctx.workflow.get_workflows().await?.iter() {
            queues.insert(workflow.queue.clone());
            for activity in ctx.workflow.get_workflow_activities(&workflow.id).await?.iter() {
                queues.insert(activity.queue.clone());
            }
        }
        let mut queues: Vec<String> = queues.into_iter().collect();
        queues.sort();
        Ok(queues.into_iter().map(|q| MessageQueueObject { queue: q }).collect())
    }

    async fn get_messages(&self, ctx: &Context<'_>, queue: String, offset: i64, limit: i64, archived: bool) -> Result<Vec<MessageObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let admin_group = ctx.security.get_administrators_group().await?;
        if !ctx.principal.has_group(&admin_group.id) {
            return Err(Error::new("invalid permissions"));
        }
        Ok(ctx.messages.get_messages_raw(&queue, offset, limit, archived).await?.into_iter().map(MessageObject::new).collect())
    }

    async fn get_message(&self, ctx: &Context<'_>, queue: String, id: i64, archived: bool) -> Result<Option<MessageObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let admin_group = ctx.security.get_administrators_group().await?;
        if !ctx.principal.has_group(&admin_group.id) {
            return Err(Error::new("invalid permissions"));
        }
        let id = WorkflowExecutionId {
            id,
            queue,
        };
        Ok(ctx.messages.get_message_raw(&id, archived).await?.map(MessageObject::new))
    }
}