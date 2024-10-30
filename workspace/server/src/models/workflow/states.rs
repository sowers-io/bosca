use crate::worklfow::yaml::into;
use async_graphql::*;
use bytes::{BufMut, BytesMut};
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde_json::Value;
use tokio_postgres::Row;
use yaml_rust2::Yaml;

#[derive(Enum, Debug, Copy, Clone, Eq, PartialEq)]
pub enum WorkflowStateType {
    Processing,
    Draft,
    Pending,
    Approval,
    Approved,
    Published,
    Failure,
}

pub struct WorkflowState {
    pub id: String,
    pub name: String,
    pub description: String,
    pub state_type: WorkflowStateType,
    pub configuration: Value,

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

impl From<&Yaml> for WorkflowState {
    fn from(yaml: &Yaml) -> Self {
        Self {
            id: yaml["id"].as_str().unwrap().to_string(),
            name: yaml["name"].as_str().unwrap().to_string(),
            description: yaml["description"].as_str().unwrap().to_string(),
            state_type: WorkflowStateType::from_sql(
                &Type::VARCHAR,
                yaml["type"].as_str().unwrap().as_ref(),
            )
            .unwrap(),
            configuration: into(&yaml["description"]),
            workflow_id: if yaml["workflowId"].is_null() {
                None
            } else {
                Some(yaml["workflowId"].as_str().unwrap().to_string())
            },
            entry_workflow_id: if yaml["entryWorkflowId"].is_null() {
                None
            } else {
                Some(yaml["entryWorkflowId"].as_str().unwrap().to_string())
            },
            exit_workflow_id: if yaml["exitWorkflowId"].is_null() {
                None
            } else {
                Some(yaml["exitWorkflowId"].as_str().unwrap().to_string())
            },
        }
    }
}

impl From<&Yaml> for WorkflowStateInput {
    fn from(yaml: &Yaml) -> Self {
        Self {
            id: yaml["id"].as_str().unwrap_or("").to_string(),
            name: yaml["name"].as_str().unwrap_or("").to_string(),
            description: yaml["description"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            state_type: WorkflowStateType::from_sql(
                &Type::VARCHAR,
                yaml["type"].as_str().unwrap().as_ref(),
            )
            .unwrap(),
            configuration: into(&yaml["description"]),
            workflow_id: if yaml["workflowId"].is_null() || yaml["workflowId"].is_badvalue() {
                None
            } else {
                Some(yaml["workflowId"].as_str().unwrap().to_string())
            },
            entry_workflow_id: if yaml["entryWorkflowId"].is_null()
                || yaml["entryWorkflowId"].is_badvalue()
            {
                None
            } else {
                Some(yaml["entryWorkflowId"].as_str().unwrap().to_string())
            },
            exit_workflow_id: if yaml["exitWorkflowId"].is_null()
                || yaml["exitWorkflowId"].is_badvalue()
            {
                None
            } else {
                Some(yaml["exitWorkflowId"].as_str().unwrap().to_string())
            },
        }
    }
}
