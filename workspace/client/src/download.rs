use crate::client::{MetadataContentDownloadUrl, MetadataSupplementaryDownloadUrl};
use http::{HeaderMap, HeaderName, HeaderValue};
use std::path::Path;
use std::str::FromStr;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use crate::Error;

pub async fn download_path(id: &String, download: &MetadataContentDownloadUrl) -> Result<String, Error> {
    download_path_with_extension(id, download, None).await
}

pub async fn download_path_with_extension(id: &String, download: &MetadataContentDownloadUrl, extension: Option<String>) -> Result<String, Error> {
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
    let path = Path::new(path_str.as_str());
    let mut file = File::create(path).await?;
    while let Some(chunk) = response.chunk().await? {
        file.write_all(chunk.as_ref()).await?;
    }
    file.flush().await?;
    Ok(path_str)
}

pub async fn download_supplementary_path(id: &String, download: &MetadataSupplementaryDownloadUrl) -> Result<String, Error> {
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
    let path_str = format!("/tmp/bosca/{}", id);
    let path = Path::new(path_str.as_str());
    let mut file = File::create(path).await?;
    while let Some(chunk) = response.chunk().await? {
        file.write_all(chunk.as_ref()).await?;
    }
    file.flush().await?;
    Ok(path_str)
}
