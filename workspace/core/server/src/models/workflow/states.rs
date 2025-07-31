use async_graphql::*;
use bytes::{BufMut, BytesMut};
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_postgres::Row;

#[derive(Enum, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum WorkflowStateType {
    Processing,
    Draft,
    Pending,
    Approval,
    Approved,
    Advertised,
    Published,
    Failure,
}

pub const PUBLISHED: &str = "published";
pub const ADVERTISED: &str = "advertised";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkflowState {
    pub id: String,
    pub name: String,
    pub description: String,
    pub state_type: WorkflowStateType,
    pub configuration: Option<Value>,

    pub workflow_id: Option<String>,
    pub entry_workflow_id: Option<String>,
    pub exit_workflow_id: Option<String>,
}

#[derive(InputObject)]
pub struct WorkflowStateInput {
    pub id: String,
    pub name: String,
    pub description: String,
    #[graphql(name = "type")]
    pub state_type: WorkflowStateType,
    pub configuration: Value,

    pub workflow_id: Option<String>,
    pub entry_workflow_id: Option<String>,
    pub exit_workflow_id: Option<String>,
}

impl From<Row> for WorkflowState {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            state_type: row.get("type"),
            configuration: row.get("configuration"),
            workflow_id: row.get("workflow_id"),
            entry_workflow_id: row.get("entry_workflow_id"),
            exit_workflow_id: row.get("exit_workflow_id"),
        }
    }
}

impl<'a> FromSql<'a> for WorkflowStateType {
    fn from_sql(
        _: &Type,
        raw: &'a [u8],
    ) -> Result<WorkflowStateType, Box<dyn std::error::Error + Sync + Send>> {
        let e: String = String::from_utf8_lossy(raw).parse().unwrap();
        match e.as_str() {
            "processing" => Ok(WorkflowStateType::Processing),
            "draft" => Ok(WorkflowStateType::Draft),
            "pending" => Ok(WorkflowStateType::Pending),
            "approval" => Ok(WorkflowStateType::Approval),
            "approved" => Ok(WorkflowStateType::Approved),
            "advertised" => Ok(WorkflowStateType::Advertised),
            "published" => Ok(WorkflowStateType::Published),
            "failure" => Ok(WorkflowStateType::Failure),
            _ => Ok(WorkflowStateType::Draft),
        }
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "workflow_state_type"
    }
}

impl ToSql for WorkflowStateType {
    fn to_sql(
        &self,
        _: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        match *self {
            WorkflowStateType::Processing => w.put_slice("processing".as_ref()),
            WorkflowStateType::Draft => w.put_slice("draft".as_ref()),
            WorkflowStateType::Pending => w.put_slice("pending".as_ref()),
            WorkflowStateType::Approval => w.put_slice("approval".as_ref()),
            WorkflowStateType::Approved => w.put_slice("approved".as_ref()),
            WorkflowStateType::Advertised => w.put_slice("advertised".as_ref()),
            WorkflowStateType::Published => w.put_slice("published".as_ref()),
            WorkflowStateType::Failure => w.put_slice("failure".as_ref()),
        }
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "workflow_state_type"
    }

    to_sql_checked!();
}
