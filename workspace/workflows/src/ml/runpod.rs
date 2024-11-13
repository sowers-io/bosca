use serde_json::Value;
use bosca_client::client::{Client, WorkflowJob};
use crate::activity::Error;
use crate::ml::file_request::{send_file, send_supplementary_file};

pub async fn execute_runpod(client: &Client, function: &str, action: &str, job: &WorkflowJob) -> Result<Value, Error> {
    let metadata_id = &job.metadata.as_ref().unwrap().id;
    let token = std::env::var("RUNPOD_TOKEN").unwrap_or("".to_owned());
    let mut run_url = std::env::var("RUNPOD_URL")?;
    run_url.push_str(function);
    run_url.push_str("/run");
    let response: Value = if let Some(key) = job.activity.inputs.first() {
        let download_url = client.get_metadata_supplementary_download(metadata_id, &key.name).await?.unwrap();
        send_supplementary_file(client, &token, &run_url, action, &download_url).await?
    } else {
        let download_url = client.get_metadata_download_url(metadata_id).await?;
        send_file(client, &token, &run_url, action, &download_url).await?
    };
    let id = response.get("id").unwrap().as_str().unwrap();
    let mut status_url = std::env::var("RUNPOD_URL")?;
    status_url.push_str(function);
    status_url.push_str("/status/");
    status_url.push_str(id);
    loop {
        let status_payload: Value = client.execute_rest(&token, &status_url, None::<Value>).await?;
        let status = status_payload.get("status").unwrap().as_str().unwrap();
        if status == "IN_PROGRESS" || status == "IN_QUEUE" {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        } else if status == "COMPLETED" {
            return Ok(status_payload.get("output").unwrap().clone())
        } else {
            return Err(Error::new(format!("runpod failed status: {}", status_payload)));
        }
    }
}