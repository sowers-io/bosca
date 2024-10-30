use crate::client::{MetadataContentUploadUrl, MetadataSupplementaryUploadUrl};
use bytes::Bytes;
use http::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{multipart, Body};
use std::str::FromStr;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
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
