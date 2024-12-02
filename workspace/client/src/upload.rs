use crate::client::{Client, MetadataContentUploadUrl, MetadataSupplementaryUploadUrl, WorkflowJob};
use bytes::Bytes;
use http::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{multipart, Body};
use std::str::FromStr;
use serde_json::{Map, Value};
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use crate::client::add_metadata_supplementary::MetadataSupplementaryInput;
use crate::Error;

pub async fn upload_multipart_file(
    id: &str,
    mime_type: &str,
    upload: &MetadataContentUploadUrl,
    file: File,
) -> Result<(), Error> {
    let mut headers = HeaderMap::new();
    for hdr in upload.headers.iter() {
        headers.insert(
            HeaderName::from_str(hdr.name.as_str()).unwrap(),
            HeaderValue::from_str(hdr.value.as_str()).unwrap(),
        );
    }
    let stream = ReaderStream::new(file);
    let body = Body::wrap_stream(stream);
    upload_multipart_body(id, mime_type, &upload.url, headers, body).await
}

pub async fn upload_multipart_supplementary_file(
    id: &str,
    mime_type: &str,
    upload: &MetadataSupplementaryUploadUrl,
    file: File,
) -> Result<(), Error> {
    let mut headers = HeaderMap::new();
    for hdr in upload.content.urls.upload.headers.iter() {
        headers.insert(
            HeaderName::from_str(hdr.name.as_str()).unwrap(),
            HeaderValue::from_str(hdr.value.as_str()).unwrap(),
        );
    }
    let stream = ReaderStream::new(file);
    let body = Body::wrap_stream(stream);
    upload_multipart_body(
        id,
        mime_type,
        &upload.content.urls.upload.url,
        headers,
        body,
    )
    .await
}

pub async fn upload_multipart_bytes(
    id: &str,
    mime_type: &str,
    upload: &MetadataContentUploadUrl,
    bytes: Bytes,
) -> Result<(), Error> {
    let mut headers = HeaderMap::new();
    for hdr in upload.headers.iter() {
        headers.insert(
            HeaderName::from_str(hdr.name.as_str()).unwrap(),
            HeaderValue::from_str(hdr.value.as_str()).unwrap(),
        );
    }
    let body = Body::from(bytes);
    upload_multipart_body(id, mime_type, &upload.url, headers, body).await
}

pub async fn upload_multipart_supplementary_bytes(
    id: &str,
    mime_type: &str,
    upload: &MetadataSupplementaryUploadUrl,
    bytes: Bytes,
) -> Result<(), Error> {
    let mut headers = HeaderMap::new();
    for hdr in upload.content.urls.upload.headers.iter() {
        headers.insert(
            HeaderName::from_str(hdr.name.as_str()).unwrap(),
            HeaderValue::from_str(hdr.value.as_str()).unwrap(),
        );
    }
    let body = Body::from(bytes);
    upload_multipart_body(
        id,
        mime_type,
        &upload.content.urls.upload.url,
        headers,
        body,
    )
    .await
}

pub async fn upload_multipart_body(
    id: &str,
    mime_type: &str,
    url: &str,
    headers: HeaderMap,
    body: Body,
) -> Result<(), Error> {
    let part = multipart::Part::stream(body)
        .file_name(id.to_owned())
        .mime_str(mime_type.as_ref())?;
    let form = multipart::Form::new().part(id.to_owned(), part);
    let response = reqwest::Client::builder()
        .build()?
        .post(url)
        .headers(headers)
        .multipart(form)
        .send()
        .await?;
    let status = response.status();
    if !status.is_success() {
        let response_text = response.text().await?;
        return Err(Error::new(format!(
            "Failed to upload file: {}",
            response_text
        )));
    }
    Ok(())
}

pub async fn upload_supplementary(client: &Client, job: &WorkflowJob, name: &str, bytes: Bytes, content_type: Option<String>) -> Result<(), Error> {
    let key = &job.workflow_activity.outputs.first().unwrap().value;
    upload_supplementary_with_key(client, job, key, name, bytes, content_type).await?;
    Ok(())
}

pub async fn upload_supplementary_with_key(client: &Client, job: &WorkflowJob, key: &str, name: &str, bytes: Bytes, content_type: Option<String>) -> Result<(), Error> {
    let metadata_id = &job.metadata.as_ref().unwrap().id;
    let content_type = content_type.unwrap_or("application/octet-stream".to_string());

    if !job.metadata.as_ref().unwrap().supplementary.iter().any(|s| s.key == key) {
        let mut attributes = Map::new();
        if let Some(source) = job.workflow_activity.configuration.get("source") {
            attributes.insert("source".to_owned(), source.clone());
        }
        client
            .add_metadata_supplementary(MetadataSupplementaryInput {
                metadata_id: metadata_id.to_owned(),
                key: key.to_owned(),
                attributes: Some(Value::Object(attributes)),
                name: name.to_owned(),
                content_type: content_type.to_owned(),
                content_length: None,
                source_id: None,
                source_identifier: None,
            })
            .await?;
    }

    let upload_url = client
        .get_metadata_supplementary_upload(metadata_id, key)
        .await?;
    upload_multipart_supplementary_bytes(
        metadata_id,
        &content_type,
        &upload_url,
        bytes,
    ).await?;

    Ok(())
}
