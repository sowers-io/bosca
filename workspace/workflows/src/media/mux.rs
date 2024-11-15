use std::env;
use std::os::linux::fs::MetadataExt;
use crate::activity::{Activity, ActivityContext, Error};
use async_trait::async_trait;
use std::str::FromStr;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use http::{HeaderMap, HeaderName, HeaderValue, StatusCode};
use reqwest::Body;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use bosca_client::client::{Client, WorkflowJob};
use bosca_client::download::download_path;
use crate::util::add_to;

pub struct MuxUploadActivity {
    id: String,
}

impl Default for MuxUploadActivity {
    fn default() -> Self {
        Self::new()
    }
}

impl MuxUploadActivity {
    pub fn new() -> MuxUploadActivity {
        MuxUploadActivity {
            id: "video.send_to_mux".to_string(),
        }
    }
}

#[async_trait]
impl Activity for MuxUploadActivity {
    fn id(&self) -> &String {
        &self.id
    }

    async fn execute(&self, client: &Client, context: &mut ActivityContext, job: &WorkflowJob) -> Result<(), Error> {
        let metadata_id = &job.metadata.as_ref().unwrap().id;
        let download_url = client.get_metadata_download_url(metadata_id).await?;
        let download = download_path(metadata_id, &download_url).await?;
        context.add_file_clean(&download);
        let token_id = env::var("MUX_TOKEN_ID")?;
        let token_secret = env::var("MUX_TOKEN_SECRET")?;
        let authorization = BASE64_STANDARD.encode(format!("{}:{}", token_id, token_secret).as_str());
        let mut authorization_headers = HeaderMap::new();
        authorization_headers.insert(
            HeaderName::from_str("Authorization").unwrap(),
            HeaderValue::from_str(format!("Basic {}", authorization).as_str()).unwrap(),
        );
        authorization_headers.insert(
            HeaderName::from_str("Content-Type").unwrap(),
            HeaderValue::from_str("application/json").unwrap(),
        );
        let mut record = if job.context.is_null() {
            let json_body = json!(UploadRequest {
                new_asset_settings: NewAssetSettings {
                    playback_policy: vec!["public".to_owned()],
                    encoding_tier: "smart".to_owned(),
                    test: env::var("MUX_TEST").unwrap_or("".to_owned()) == "true",
                },
                cors_origin: "*".to_owned(),
            });
            let json_body = json_body.to_string();
            let body = Body::from(json_body);
            let response = reqwest::Client::builder()
                .build()?
                .post("https://api.mux.com/video/v1/uploads")
                .headers(authorization_headers.clone())
                .body(body)
                .send()
                .await?;
            let response = response.json::<UploadResponse>().await?;
            let record = MuxRecord {
                upload: Some(response.data),
                asset: None,
                uploaded: false,
            };
            let record_json = json!(record);
            client.set_job_context(job.id.id, job.id.queue.as_str(), &record_json).await?;
            record
        } else {
            serde_json::from_value::<MuxRecord>(job.context.clone())?
        };
        if let Some(asset) = &record.asset {
            if let Some(playback_ids) = &asset.playback_ids {
                if !playback_ids.is_empty() {
                    return Ok(())
                }
            }
        }
        if !record.uploaded {
            let mut file = File::open(download.as_str()).await?;
            let size = file.metadata().await?.st_size() as usize;
            const CHUNK_SIZE: usize = 1024 * 1024 * 30;
            let mut offset = 0;
            loop {
                let mut buf = vec![0u8; CHUNK_SIZE];
                let read = file.read(&mut buf).await?;
                if read == 0 {
                    break;
                }
                let mut headers = HeaderMap::new();
                headers.insert(
                    HeaderName::from_str("Content-Length").unwrap(),
                    HeaderValue::from(read),
                );
                headers.insert(
                    HeaderName::from_str("Content-Range").unwrap(),
                    HeaderValue::from_str(format!("bytes {}-{}/{}", offset, offset + read - 1, size).as_str()).unwrap(),
                );
                let buf_slice = &buf[0..read];
                let body = Body::from(buf_slice.to_vec());
                let response = reqwest::Client::builder()
                    .build()?
                    .put(record.upload.clone().unwrap().url.unwrap().clone())
                    .headers(headers)
                    .body(body)
                    .send()
                    .await?;
                let status = response.status();
                offset += read;
                if offset >= size {
                    break;
                }
                if status != StatusCode::PERMANENT_REDIRECT && !status.is_success() {
                    let text = response.text().await?;
                    return Err(Error::new(format!("failed to upload: {}: {}", status, text)));
                }
            }
            record.uploaded = true;
            let record_json = json!(record.clone());
            client.set_job_context(job.id.id, job.id.queue.as_str(), &record_json).await?;
        }
        let response = reqwest::Client::builder()
            .build()?
            .get(format!("https://api.mux.com/video/v1/uploads/{}", record.upload.clone().unwrap().id).as_str())
            .headers(authorization_headers.clone())
            .send()
            .await?;
        let upload_response = response.json::<UploadResponse>().await?;
        if upload_response.data.asset_id.is_none() {
            return Err(Error::new("asset id is missing".to_owned()));
        }
        let response = reqwest::Client::builder()
            .build()?
            .get(format!("https://api.mux.com/video/v1/assets/{}", upload_response.data.asset_id.unwrap()).as_str())
            .headers(authorization_headers.clone())
            .send()
            .await?;
        let asset_response = response.json::<AssetResponse>().await?;
        if asset_response.data.is_none() {
            return Err(Error::new("missing data".to_owned()));
        }
        let asset = asset_response.data.unwrap();
        if asset.playback_ids.is_none() {
            return Err(Error::new("missing playback ids".to_owned()));
        }
        let playback_ids = asset.playback_ids.clone().unwrap();
        if playback_ids.is_empty() {
            return Err(Error::new("empty playback ids".to_owned()));
        }
        record.asset = Some(asset.clone());

        let mut attributes_map = Map::new();
        let mut system_attributes_map = Map::new();

        let record_json = json!(record);
        client.set_job_context(job.id.id, job.id.queue.as_str(), &record_json).await?;

        // build attributes
        let playback_id = &playback_ids.first().unwrap().id;
        let attributes = MuxAttributes {
            playback_id: playback_id.clone(),
            hls_url: format!("https://stream.mux.com/{}.m3u8", playback_id),
            video_quality: asset.video_quality.clone(),
            aspect_ratio: asset.aspect_ratio.clone(),
            duration: asset.duration,
        };
        let mux_attributes_json = json!(attributes);
        add_to(&mut attributes_map, &mux_attributes_json);
        add_to(&mut attributes_map, &job.metadata.as_ref().unwrap().attributes);

        // build system attributes
        if let Some(attrs) = &job.metadata.as_ref().unwrap().system_attributes {
            add_to(&mut system_attributes_map, attrs);
        }
        add_to(&mut system_attributes_map, &record_json);

        let attributes = Value::Object(attributes_map);
        let system_attributes = Value::Object(system_attributes_map);
        let metadata_id = &job.metadata.as_ref().unwrap().id;
        client.set_metadata_attributes(metadata_id, &attributes).await?;
        client.set_metadata_system_attributes(metadata_id, &system_attributes).await?;
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct UploadResponse {
    pub data: Upload
}

#[derive(Clone, Serialize, Deserialize)]
struct Upload {
    pub id: String,
    pub url: Option<String>,
    pub asset_id: Option<String>,
    pub status: String,
}

#[derive(Clone, Serialize)]
#[allow(dead_code)]
struct UploadRequest {
    pub new_asset_settings: NewAssetSettings,
    pub cors_origin: String
}

#[derive(Clone, Serialize)]
#[allow(dead_code)]
struct NewAssetSettings {
    pub playback_policy: Vec<String>,
    pub encoding_tier: String,
    pub test: bool
}


#[derive(Clone, Serialize, Deserialize)]
struct AssetResponse {
    pub data: Option<Asset>,
}

#[derive(Clone, Serialize, Deserialize)]
struct AssetTrack {
    #[serde(rename = "type")]
    pub track_type: String,
    pub max_width: Option<i32>,
    pub max_height: Option<i32>,
    pub max_frame_rate: Option<f32>,
    pub id: String,
    pub duration: f32,
    pub max_channels: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize)]
struct PlaybackIds {
    pub id: String,
    pub policy: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct Asset {
    pub tracks: Option<Vec<AssetTrack>>,
    pub status: String,
    pub resolution_tier: Option<String>,
    pub playback_ids: Option<Vec<PlaybackIds>>,
    pub passthrough: Option<String>,
    pub mp4_support: Option<String>,
    pub max_stored_resolution: Option<String>,
    pub max_stored_frame_rate: Option<f32>,
    pub master_access: Option<String>,
    pub id: String,
    pub encoding_tier: Option<String>,
    pub video_quality: Option<String>,
    pub duration: f32,
    pub aspect_ratio: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
struct MuxRecord {
    pub asset: Option<Asset>,
    pub upload: Option<Upload>,
    pub uploaded: bool
}

#[derive(Clone, Serialize, Deserialize)]
struct MuxAttributes {
    pub playback_id: String,
    pub hls_url: String,
    pub aspect_ratio: Option<String>,
    pub duration: f32,
    pub video_quality: Option<String>,
}
