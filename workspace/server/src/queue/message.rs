use async_graphql::{Error, Object};
use crate::models::workflow::execution_plan::{WorkflowExecutionPlan, WorkflowJob};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_postgres::Row;

#[derive(Clone)]
pub struct Message {
    pub id: i64,
    pub visible_timeout: DateTime<Utc>,
    pub value: MessageValue,
}

pub struct MessageObject {
    message: Message,
}

impl MessageObject {
    pub fn new(message: Message) -> Self {
        Self { message }
    }
}

#[Object(name = "Message")]
impl MessageObject {
    async fn id(&self) -> i64 {
        self.message.id
    }

    async fn visible_timeout(&self) -> &DateTime<Utc> {
        &self.message.visible_timeout
    }

    async fn value(&self) -> Result<Value, Error> {
        serde_json::to_value(&self.message.value).map_err(|e| Error::new(e.to_string()))
    }
}


#[derive(Clone, Serialize, Deserialize)]
pub enum MessageValue {
    Plan(WorkflowExecutionPlan),
    Job(WorkflowJob),
}

impl From<&Row> for Message {
    fn from(value: &Row) -> Self {
        let v: Value = value.get("message");
        Message {
            id: value.get("msg_id"),
            visible_timeout: value.get("vt"),
            value: serde_json::from_value(v).unwrap(),
        }
    }
}

impl From<Row> for Message {
    fn from(value: Row) -> Self {
        let v: Value = value.get("message");
        Message {
            id: value.get("msg_id"),
            visible_timeout: value.get("vt"),
            value: serde_json::from_value(v).unwrap(),
        }
    }
}
