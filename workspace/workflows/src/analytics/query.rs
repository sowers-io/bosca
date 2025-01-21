use crate::activity::{Activity, ActivityContext, Error};
use async_trait::async_trait;
use bytes::Bytes;
use chrono::Utc;
use duckdb::arrow::json::ArrayWriter;
use duckdb::Connection;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::task;
use bosca_client::client::{Client, WorkflowJob};
use bosca_client::client::add_activity::{ActivityInput, ActivityParameterInput, ActivityParameterType};
use bosca_client::download::download_path;
use bosca_client::upload::{upload_supplementary, upload_supplementary_with_key};

pub struct QueryActivity {
    id: String,
}

impl Default for QueryActivity {
    fn default() -> Self {
        Self::new()
    }
}

impl QueryActivity {
    pub fn new() -> QueryActivity {
        QueryActivity {
            id: "analytics.query".to_string(),
        }
    }

    fn query_sync(query: String) -> Result<Vec<u8>, Error> {
        let conn = Connection::open_in_memory()?;
        let access_key_id = std::env::var("QUERY_AWS_ACCESS_KEY_ID")?;
        let secret_access_key = std::env::var("QUERY_AWS_SECRET_ACCESS_KEY")?;
        let endpoint = std::env::var("QUERY_AWS_ENDPOINT")?;
        let config_stmt = format!("SET s3_access_key_id='{}';SET s3_secret_access_key='{}';SET s3_endpoint='{}';", access_key_id, secret_access_key, endpoint);
        conn.execute_batch(&config_stmt)?;
        let mut stmt = conn.prepare_cached(&query)?;
        let rows = stmt.query_arrow(duckdb::params![])?;
        let buf = Vec::new();
        let mut writer = ArrayWriter::new(buf);
        for row in rows {
            writer.write(&row)?;
        }
        writer.finish()?;
        Ok(writer.into_inner())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryConfiguration {
    pub name: String,
    pub query: String
}

#[async_trait]
impl Activity for QueryActivity {
    fn id(&self) -> &String {
        &self.id
    }

    fn create_activity_input(&self) -> ActivityInput {
        let mut configuration = serde_json::Map::new();
        configuration.insert("date_format".to_owned(), Value::String("%Y%m%d%H%M".to_owned()));
        ActivityInput {
            id: self.id.to_owned(),
            name: "Analytics Query".to_string(),
            description: "Execute an analytics query.  Will output a supplementaryId and a date specific supplementaryId.".to_string(),
            child_workflow_id: None,
            configuration: Value::Object(configuration),
            inputs: vec![],
            outputs: vec![
                ActivityParameterInput {
                    name: "supplementaryId".to_owned(),
                    type_: ActivityParameterType::SUPPLEMENTARY
                }
            ],
        }
    }

    async fn execute(&self, client: &Client, _: &mut ActivityContext, job: &WorkflowJob) -> Result<(), Error> {
        let default_date_format = Value::String("%Y%m%d%H%M".to_owned());
        let metadata_id = job.metadata.as_ref().unwrap().id.as_str();
        let download = client.get_metadata_download_url(metadata_id).await?;
        let filename = download_path(metadata_id, &download).await?;
        let mut file = File::open(filename).await?;
        let mut buf = String::new();
        file.read_to_string(&mut buf).await?;
        let cfg = from_str::<QueryConfiguration>(&buf)?;
        let results = task::spawn_blocking(move || QueryActivity::query_sync(cfg.query)).await??;
        let key = &job.workflow_activity.outputs.first().unwrap().value;
        let format = job.workflow_activity.configuration.get("date_format").unwrap_or(&default_date_format).as_str().unwrap();
        let timestamp = Utc::now().format(format).to_string();
        let key_with_suffix = format!("{}_{}", key, timestamp);
        upload_supplementary_with_key(client, job, &key_with_suffix, &cfg.name, Bytes::from(results.clone()), Some("application/json".to_owned())).await?;
        upload_supplementary(client, job, &cfg.name, Bytes::from(results), Some("application/json".to_owned())).await?;
        Ok(())
    }
}
