use crate::worklfow::yaml::into;
use async_graphql::*;
use bytes::{BufMut, BytesMut};
use postgres_types::{to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_postgres::Row;
use uuid::Uuid;
use yaml_rust2::Yaml;

#[derive(Enum, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ActivityParameterType {
    Context,
    Supplementary,
    SupplementaryArray,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub id: String,
    pub name: String,
    pub description: String,
    pub child_workflow_id: Option<String>,
    pub configuration: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowActivity {
    pub id: i64,
    pub workflow_id: String,
    pub activity_id: String,
    pub queue: String,
    pub execution_group: i32,
    pub configuration: Value,
}

#[derive(InputObject)]
pub struct ActivityInput {
    pub id: String,
    pub name: String,
    pub description: String,
    pub child_workflow_id: Option<String>,
    pub configuration: Value,
    pub inputs: Vec<ActivityParameterInput>,
    pub outputs: Vec<ActivityParameterInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityParameter {
    pub name: String,
    pub parameter_type: ActivityParameterType,
}

#[derive(InputObject)]
pub struct ActivityParameterInput {
    pub name: String,
    #[graphql(name = "type")]
    pub parameter_type: ActivityParameterType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowActivityModel {
    pub model_id: Uuid,
    pub configuration: Value,
}

#[derive(Serialize, Deserialize, InputObject)]
pub struct WorkflowActivityModelInput {
    pub model_id: String,
    pub configuration: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowActivityStorageSystem {
    pub system_id: Uuid,
    pub configuration: Value,
}

#[derive(Serialize, Deserialize, InputObject)]
pub struct WorkflowActivityStorageSystemInput {
    pub system_id: String,
    pub configuration: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowActivityPrompt {
    pub prompt_id: Uuid,
    pub configuration: Value,
}

#[derive(Serialize, Deserialize, InputObject)]
pub struct WorkflowActivityPromptInput {
    pub prompt_id: String,
    pub configuration: Value,
}

#[derive(Serialize, Deserialize, InputObject)]
pub struct WorkflowActivityInput {
    pub workflow_id: String,
    pub activity_id: String,
    pub queue: String,
    pub execution_group: i32,
    pub description: String,
    pub inputs: Vec<WorkflowActivityParameterInput>,
    pub outputs: Vec<WorkflowActivityParameterInput>,
    pub models: Vec<WorkflowActivityModelInput>,
    pub storage_systems: Vec<WorkflowActivityStorageSystemInput>,
    pub prompts: Vec<WorkflowActivityPromptInput>,
    pub configuration: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowActivityParameter {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, InputObject)]
pub struct WorkflowActivityParameterInput {
    pub name: String,
    pub value: String,
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
        }
    }
}

impl From<&Row> for WorkflowActivityParameter {
    fn from(row: &Row) -> Self {
        Self {
            name: row.get("name"),
            value: row.get("value"),
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

impl From<&Yaml> for ActivityInput {
    fn from(yaml: &Yaml) -> Self {
        let mut inputs: Vec<ActivityParameterInput> = vec![];
        let mut outputs: Vec<ActivityParameterInput> = vec![];

        if !yaml["inputs"].is_null() && !yaml["inputs"].is_badvalue() {
            for item in yaml["inputs"].as_hash().unwrap() {
                inputs.push(ActivityParameterInput {
                    name: item.0.as_str().unwrap().to_string(),
                    parameter_type: ActivityParameterType::from_sql(
                        &Type::VARCHAR,
                        item.1.as_str().unwrap().as_ref(),
                    )
                    .unwrap(),
                })
            }
        }

        if !yaml["outputs"].is_null() && !yaml["outputs"].is_badvalue() {
            for item in yaml["outputs"].as_hash().unwrap() {
                outputs.push(ActivityParameterInput {
                    name: item.0.as_str().unwrap().to_string(),
                    parameter_type: ActivityParameterType::from_sql(
                        &Type::VARCHAR,
                        item.1.as_str().unwrap().as_ref(),
                    )
                    .unwrap(),
                })
            }
        }

        Self {
            id: yaml["id"].as_str().unwrap_or("").to_string(),
            name: yaml["name"].as_str().unwrap_or("").to_string(),
            child_workflow_id: if yaml["child_workflow_id"].is_null()
                || yaml["child_workflow_id"].is_badvalue()
            {
                None
            } else {
                Some(yaml["child_workflow_id"].as_str().unwrap().to_string())
            },
            description: yaml["description"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            inputs,
            outputs,
            configuration: into(&yaml["configuration"]),
        }
    }
}

impl From<&Yaml> for WorkflowActivityInput {
    fn from(yaml: &Yaml) -> Self {
        let mut inputs: Vec<WorkflowActivityParameterInput> = vec![];
        let mut outputs: Vec<WorkflowActivityParameterInput> = vec![];
        let mut models: Vec<WorkflowActivityModelInput> = vec![];
        let mut storage_systems: Vec<WorkflowActivityStorageSystemInput> = vec![];
        let mut prompts: Vec<WorkflowActivityPromptInput> = vec![];

        if !yaml["prompts"].is_null() && !yaml["prompts"].is_badvalue() {
            for item in yaml["prompts"].as_hash().unwrap() {
                let p = item.1;
                prompts.push(WorkflowActivityPromptInput {
                    prompt_id: item.0.as_str().unwrap().to_string(),
                    configuration: into(&p["configuration"]),
                })
            }
        }

        if !yaml["models"].is_null() && !yaml["models"].is_badvalue() {
            for item in yaml["models"].as_hash().unwrap() {
                let m = item.1;
                models.push(WorkflowActivityModelInput {
                    model_id: item.0.as_str().unwrap().to_string(),
                    configuration: into(&m["configuration"]),
                })
            }
        }

        if !yaml["storageSystems"].is_null() && !yaml["storageSystems"].is_badvalue() {
            for item in yaml["storageSystems"].as_hash().unwrap() {
                let s = item.1;
                storage_systems.push(WorkflowActivityStorageSystemInput {
                    system_id: item.0.as_str().unwrap().to_string(),
                    configuration: into(&s["configuration"]),
                })
            }
        }

        if !yaml["inputs"].is_null() && !yaml["inputs"].is_badvalue() {
            for item in yaml["inputs"].as_hash().unwrap() {
                inputs.push(WorkflowActivityParameterInput {
                    name: item.0.as_str().unwrap().to_string(),
                    value: item.1.as_str().unwrap().to_string(),
                })
            }
        }

        if !yaml["outputs"].is_null() && !yaml["outputs"].is_badvalue() {
            for item in yaml["outputs"].as_hash().unwrap() {
                outputs.push(WorkflowActivityParameterInput {
                    name: item.0.as_str().unwrap().to_string(),
                    value: item.1.as_str().unwrap().to_string(),
                })
            }
        }

        Self {
            workflow_id: yaml["workflow_id"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            activity_id: yaml["activity_id"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            queue: yaml["queue"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            execution_group: yaml["executionGroup"].as_i64().unwrap_or(0) as i32,
            description: yaml["description"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            models,
            prompts,
            storage_systems,
            inputs,
            outputs,
            configuration: into(&yaml["configuration"]),
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
