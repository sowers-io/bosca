use async_graphql::*;
use bytes::{BufMut, BytesMut};
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Enum, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ActivityParameterType {
    Context,
    Supplementary,
    SupplementaryArray,
}

#[derive(Enum, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ActivityParameterScope {
    Plan,
    Content,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub id: String,
    pub name: String,
    pub description: String,
    pub child_workflow_id: Option<String>,
    pub configuration: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowActivity {
    pub id: i64,
    pub workflow_id: String,
    pub activity_id: String,
    pub queue: String,
    pub execution_group: i32,
    pub configuration: Option<Value>,
}

#[derive(InputObject)]
pub struct ActivityInput {
    pub id: String,
    pub name: String,
    pub description: String,
    pub child_workflow_id: Option<String>,
    pub configuration: Option<Value>,
    pub inputs: Vec<ActivityParameterInput>,
    pub outputs: Vec<ActivityParameterInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityParameter {
    pub name: String,
    pub parameter_type: ActivityParameterType,
    pub scope: ActivityParameterScope,
}

#[derive(InputObject)]
pub struct ActivityParameterInput {
    pub name: String,
    #[graphql(name = "type")]
    pub parameter_type: ActivityParameterType,
    pub scope: Option<ActivityParameterScope>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowActivityModel {
    pub model_id: Uuid,
    pub configuration: Option<Value>,
}

#[derive(Serialize, Deserialize, InputObject)]
pub struct WorkflowActivityModelInput {
    pub model_id: String,
    pub configuration: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowActivityStorageSystem {
    pub system_id: Uuid,
    pub configuration: Option<Value>,
}

#[derive(Serialize, Deserialize, InputObject)]
pub struct WorkflowActivityStorageSystemInput {
    pub system_id: String,
    pub configuration: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowActivityPrompt {
    pub prompt_id: Uuid,
    pub configuration: Option<Value>,
}

#[derive(Serialize, Deserialize, InputObject)]
pub struct WorkflowActivityPromptInput {
    pub prompt_id: String,
    pub configuration: Option<Value>,
}

#[derive(Serialize, Deserialize, InputObject)]
pub struct WorkflowActivityInput {
    pub activity_id: String,
    pub queue: String,
    pub execution_group: i32,
    pub description: String,
    pub inputs: Vec<WorkflowActivityParameterInput>,
    pub outputs: Vec<WorkflowActivityParameterInput>,
    pub models: Vec<WorkflowActivityModelInput>,
    pub storage_systems: Vec<WorkflowActivityStorageSystemInput>,
    pub prompts: Vec<WorkflowActivityPromptInput>,
    pub configuration: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowActivityParameter {
    pub name: String,
    pub value: String,
    pub scope: ActivityParameterScope,
}

#[derive(Serialize, Deserialize, InputObject)]
pub struct WorkflowActivityParameterInput {
    pub name: String,
    pub value: String,
    pub scope: Option<ActivityParameterScope>,
}

impl From<&Row> for WorkflowActivity {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            workflow_id: row.get("workflow_id"),
            activity_id: row.get("activity_id"),
            queue: row.get("queue"),
            execution_group: row.get("execution_group"),
            configuration: row.get("configuration"),
        }
    }
}

impl From<&Row> for WorkflowActivityModel {
    fn from(row: &Row) -> Self {
        Self {
            model_id: row.get("model_id"),
            configuration: row.get("configuration"),
        }
    }
}

impl From<&Row> for WorkflowActivityStorageSystem {
    fn from(row: &Row) -> Self {
        Self {
            system_id: row.get("storage_system_id"),
            configuration: row.get("configuration"),
        }
    }
}

impl From<&Row> for WorkflowActivityPrompt {
    fn from(row: &Row) -> Self {
        Self {
            prompt_id: row.get("prompt_id"),
            configuration: row.get("configuration"),
        }
    }
}

impl From<&Row> for ActivityParameter {
    fn from(row: &Row) -> Self {
        Self {
            name: row.get("name"),
            parameter_type: row.get("type"),
            scope: row.try_get("scope").unwrap_or(
                ActivityParameterScope::Content,
            ),
        }
    }
}

impl From<&Row> for WorkflowActivityParameter {
    fn from(row: &Row) -> Self {
        Self {
            name: row.get("name"),
            value: row.get("value"),
            scope: row.try_get("scope").unwrap_or(
                ActivityParameterScope::Content,
            ),
        }
    }
}

impl From<&Row> for Activity {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            child_workflow_id: row.get("child_workflow_id"),
            configuration: row.get("configuration"),
        }
    }
}

impl From<Row> for WorkflowActivity {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            workflow_id: row.get("workflow_id"),
            activity_id: row.get("activity_id"),
            queue: row.get("queue"),
            execution_group: row.get("execution_group"),
            configuration: row.get("configuration"),
        }
    }
}

impl<'a> FromSql<'a> for ActivityParameterType {
    fn from_sql(
        _: &Type,
        raw: &'a [u8],
    ) -> Result<ActivityParameterType, Box<dyn std::error::Error + Sync + Send>> {
        let e: String = String::from_utf8_lossy(raw).parse().unwrap();
        match e.as_str() {
            "context" => Ok(ActivityParameterType::Context),
            "supplementary_array" => Ok(ActivityParameterType::SupplementaryArray),
            "supplementary" => Ok(ActivityParameterType::Supplementary),
            _ => Ok(ActivityParameterType::Supplementary),
        }
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "activity_parameter_type"
    }
}

impl ToSql for ActivityParameterType {
    fn to_sql(
        &self,
        _: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        match *self {
            ActivityParameterType::Context => w.put_slice("context".as_ref()),
            ActivityParameterType::Supplementary => w.put_slice("supplementary".as_ref()),
            ActivityParameterType::SupplementaryArray => {
                w.put_slice("supplementary_array".as_ref())
            }
        }
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "activity_parameter_type"
    }

    to_sql_checked!();
}

impl<'a> FromSql<'a> for ActivityParameterScope {
    fn from_sql(
        _: &Type,
        raw: &'a [u8],
    ) -> Result<ActivityParameterScope, Box<dyn std::error::Error + Sync + Send>> {
        let e: String = String::from_utf8_lossy(raw).parse().unwrap();
        match e.as_str() {
            "content" => Ok(ActivityParameterScope::Content),
            "plan" => Ok(ActivityParameterScope::Plan),
            _ => Ok(ActivityParameterScope::Plan),
        }
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "activity_parameter_scope"
    }
}

impl ToSql for ActivityParameterScope {
    fn to_sql(
        &self,
        _: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        match *self {
            ActivityParameterScope::Plan => w.put_slice("plan".as_ref()),
            ActivityParameterScope::Content => w.put_slice("content".as_ref()),
        }
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "activity_parameter_scope"
    }

    to_sql_checked!();
}
