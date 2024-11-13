use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use bosca_client::client::{Client, MetadataContentDownloadUrl, MetadataSupplementaryDownloadUrl};
use crate::activity::Error;

#[derive(Serialize, Deserialize, Clone)]
pub struct Request {
    pub input: FileRequest
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FileRequest {
    pub action: String,
    pub url: String,
    pub headers: Vec<FileRequestHeader>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FileRequestHeader {
    pub name: String,
    pub value: String,
}

pub async fn send_supplementary_file<Response: DeserializeOwned + Clone>(client: &Client, token: &str, url: &str, action: &str, request: &MetadataSupplementaryDownloadUrl) -> Result<Response, Error> {
    let request = FileRequest {
        action: action.to_owned(),
        url: request.url.to_owned(),
        headers: request.headers.iter().map(|h| FileRequestHeader {
            name: h.name.to_owned(),
            value: h.value.to_owned(),
        }).collect(),
    };
    let input = Some(Request {
        input: request,
    });
    let response: Response = client.execute_rest(token, url, input).await?;
    Ok(response)
}

pub async fn send_file<Response: DeserializeOwned + Clone>(client: &Client, token: &str, url: &str, action: &str, request: &MetadataContentDownloadUrl) -> Result<Response, Error> {
    let request = FileRequest {
        action: action.to_owned(),
        url: request.url.to_owned(),
        headers: request.headers.iter().map(|h| FileRequestHeader {
            name: h.name.to_owned(),
            value: h.value.to_owned(),
        }).collect(),
    };
    let input = Some(Request {
        input: request,
    });
    let response: Response = client.execute_rest(token, url, input).await?;
    Ok(response)
}