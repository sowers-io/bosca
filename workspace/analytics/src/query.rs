use axum::extract;
use axum::response::IntoResponse;
use duckdb::Connection;
use http::{HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::str::from_utf8;
use arrow::json::ArrayWriter;
use log::info;
use tokio::task;

#[derive(Serialize, Deserialize)]
pub struct Query {
    pub key: String,
    pub query: String,
}

pub async fn query(extract::Json(query): extract::Json<Query>) -> impl IntoResponse {
    if query.key != std::env::var("QUERY_KEY").unwrap() {
        return Err((StatusCode::FORBIDDEN, "Invalid key").into_response());
    }

    info!(target: "bosca", "running query: {}", query.query);

    let results = match task::spawn_blocking(move || match query_sync(&query) {
        Ok(result) => Ok(result),
        Err(e) => return Err(e),
    })
    .await
    {
        Ok(results) => match results {
            Ok(results) => results,
            Err(e) => {
                return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response());
            }
        },
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()),
    };
    let mut response_headers = HeaderMap::new();
    response_headers.insert("Content-Type", "application/json".parse().unwrap());
    Ok((StatusCode::OK, response_headers, format!("{{\"results\": {results}}}")).into_response())
}

fn query_sync(query: &Query) -> Result<String, Box<dyn Error + Send + Sync>> {
    let conn = Connection::open_in_memory()?;

    let access_key_id = std::env::var("QUERY_AWS_ACCESS_KEY_ID").unwrap();
    let secret_access_key = std::env::var("QUERY_AWS_SECRET_ACCESS_KEY").unwrap();
    let endpoint = std::env::var("QUERY_AWS_ENDPOINT").unwrap();
    let config_stmt = format!("SET s3_access_key_id='{}';SET s3_secret_access_key='{}';SET s3_endpoint='{}';", access_key_id, secret_access_key, endpoint);
    conn.execute_batch(&config_stmt)?;

    let mut stmt = conn.prepare_cached(&query.query)?;
    let rows = stmt.query_arrow(duckdb::params![])?;

    let buf = Vec::new();
    let mut writer = ArrayWriter::new(buf);
    for row in rows {
        writer.write(&row)?;
    }
    writer.finish()?;
    let buf = writer.into_inner();
    Ok(from_utf8(&buf)?.to_owned())
}
