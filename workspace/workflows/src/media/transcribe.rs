use std::path::PathBuf;
use crate::activity::{Activity, ActivityContext, Error};
use async_trait::async_trait;
use bytes::Bytes;
use serde_json::json;
use bosca_client::client::{Client, WorkflowJob};
use bosca_client::download::download_path_with_extension;
use uuid::Uuid;
use bosca_client::client::add_metadata_supplementary::MetadataSupplementaryInput;
use bosca_client::upload::upload_multipart_supplementary_bytes;
use crate::ml::whisper::create_segments;
use crate::ml::whisper::model::WhichModel;
use crate::ml::whisper::mp4_to_wav::convert_to_wav;

pub struct TranscribeActivity {
    id: String,
}

impl Default for TranscribeActivity {
    fn default() -> Self {
        Self::new()
    }
}

impl TranscribeActivity {
    pub fn new() -> TranscribeActivity {
        TranscribeActivity {
            id: "media.transcribe".to_string(),
        }
    }
}

#[async_trait]
impl Activity for TranscribeActivity {
    fn id(&self) -> &String {
        &self.id
    }

    async fn execute(&self, client: &Client, context: &mut ActivityContext, job: &WorkflowJob) -> Result<(), Error> {
        let metadata_id = &job.metadata.as_ref().unwrap().id;
        let download_url = client.get_metadata_download_url(metadata_id).await?;
        let download = download_path_with_extension(metadata_id, &download_url, Some("mp4".to_owned())).await?;
        context.add_file_clean(download.clone());
        let id = Uuid::new_v4();
        let path_str = format!("/tmp/bosca/{}-{}.wav", metadata_id, id);
        context.add_file_clean(path_str.clone());
        convert_to_wav(&download, &path_str).await?;
        let path_buf = PathBuf::from(path_str);
        let segments = create_segments(path_buf.as_path(),
                                       true,
                                       None,
                                       None,
                                       None,
                                       Some(WhichModel::MediumEn),
                                       None,
                                       None,
                                       false,
                                       true,
                                       true,
        )?;
        let segments = json!(segments);
        let text = segments.to_string();
        let text_bytes = Bytes::from(text);
        let mime_type = "text/json";
        let key = "transcription";
        if job.metadata.as_ref().unwrap().supplementary.is_empty() {
            client.add_metadata_supplementary(MetadataSupplementaryInput {
                metadata_id: metadata_id.to_owned(),
                key: key.to_owned(),
                name: "Transcription".to_owned(),
                content_type: mime_type.to_owned(),
                content_length: None,
                source_id: None,
                source_identifier: None,
            }).await?;
        }
        let upload_url = client.get_metadata_supplementary_upload(metadata_id, key).await?;
        upload_multipart_supplementary_bytes(metadata_id, mime_type, &upload_url, text_bytes).await?;
        Ok(())
    }
}
