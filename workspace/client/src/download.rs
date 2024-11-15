use std::fs::create_dir_all;
use crate::client::{MetadataContentDownloadUrl, MetadataSupplementaryDownloadUrl};
use http::{HeaderMap, HeaderName, HeaderValue};
use std::path::Path;
use std::str::FromStr;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use crate::Error;

pub async fn download_path(id: &str, download: &MetadataContentDownloadUrl) -> Result<String, Error> {
    download_path_with_extension(id, download, None).await
}

pub async fn download_path_with_extension(id: &str, download: &MetadataContentDownloadUrl, extension: Option<String>) -> Result<String, Error> {
    let mut headers = HeaderMap::new();
    for hdr in download.headers.iter() {
        headers.insert(
            HeaderName::from_str(hdr.name.as_str()).unwrap(),
            HeaderValue::from_str(hdr.value.as_str()).unwrap(),
        );
    }
    let mut response = reqwest::Client::builder()
        .build()?
        .get(&download.url)
        .headers(headers)
        .send()
        .await?;
    let path_str = if let Some(ext) = extension {
        format!("/tmp/bosca/{}.{}", id, ext)
    } else {
        format!("/tmp/bosca/{}", id)
    };
    let parent_path = Path::new("/tmp/bosca/");
    if !parent_path.exists() {
        create_dir_all(parent_path)?;
    }
    let path = Path::new(path_str.as_str());
    let mut file = File::create_new(path).await?;
    while let Some(chunk) = response.chunk().await? {
        file.write_all(chunk.as_ref()).await?;
    }
    file.flush().await?;
    Ok(path_str)
}

pub async fn download_supplementary_path(id: &str, download: &MetadataSupplementaryDownloadUrl) -> Result<String, Error> {
    let mut headers = HeaderMap::new();
    for hdr in download.headers.iter() {
        headers.insert(
            HeaderName::from_str(hdr.name.as_str()).unwrap(),
            HeaderValue::from_str(hdr.value.as_str()).unwrap(),
        );
    }
    let mut response = reqwest::Client::builder()
        .build()?
        .get(&download.url)
        .headers(headers)
        .send()
        .await?;
    let path_str = format!("/tmp/bosca/{}-{}", id, Uuid::new_v4());
    let parent_path = Path::new("/tmp/bosca/");
    if !parent_path.exists() {
        create_dir_all(parent_path)?;
    }
    let path = Path::new(path_str.as_str());
    let mut file = File::create_new(path).await?;
    while let Some(chunk) = response.chunk().await? {
        file.write_all(chunk.as_ref()).await?;
    }
    file.flush().await?;
    Ok(path_str)
}
