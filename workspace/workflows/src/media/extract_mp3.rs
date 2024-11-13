use crate::activity::{Activity, ActivityContext, Error};
use async_trait::async_trait;
use tokio::fs::File;
use tokio::process::Command;
use bosca_client::client::{Client, WorkflowJob};
use bosca_client::download::download_path_with_extension;
use uuid::Uuid;
use bosca_client::client::add_metadata_supplementary::MetadataSupplementaryInput;
use bosca_client::upload::upload_multipart_supplementary_file;

pub struct ExtractMp3Activity {
    id: String,
}

impl Default for ExtractMp3Activity {
    fn default() -> Self {
        Self::new()
    }
}

impl ExtractMp3Activity {
    pub fn new() -> ExtractMp3Activity {
        ExtractMp3Activity {
            id: "media.extract.mp3".to_string(),
        }
    }
}

#[async_trait]
impl Activity for ExtractMp3Activity {
    fn id(&self) -> &String {
        &self.id
    }

    async fn execute(&self, client: &Client, context: &mut ActivityContext, job: &WorkflowJob) -> Result<(), Error> {
        let metadata_id = &job.metadata.as_ref().unwrap().id;
        let download_url = client.get_metadata_download_url(metadata_id).await?;
        let download = download_path_with_extension(metadata_id, &download_url, Some("mp4".to_owned())).await?;
        context.add_file_clean(download.clone());
        let id = Uuid::new_v4();
        let path_str = format!("/tmp/bosca/{}-{}.mp3", metadata_id, id);
        context.add_file_clean(path_str.clone());

        let output = Command::new("ffmpeg")
            .arg("-i")
            .arg(download)
            .arg("-vn")  // Disable video
            .arg("-acodec")
            .arg("pcm_s16le")  // Set audio codec to 16-bit PCM
            .arg("-ar")
            .arg("16000")  // Sample rate
            .arg("-ac")
            .arg("2")  // Set to stereo
            .arg("-y")  // Overwrite output file if it exists
            .arg(&path_str)
            .output()
            .await?;
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(Error::new(error.as_ref().to_owned()));
        }

        let key = &job.activity.outputs.first().unwrap().name;
        client.add_metadata_supplementary(MetadataSupplementaryInput {
            metadata_id: metadata_id.to_owned(),
            key: key.to_owned(),
            name: "mp3".to_owned(),
            content_type: "audio/mp3".to_owned(),
            content_length: None,
            source_id: None,
            source_identifier: None,
        }).await?;
        let upload_url = client.get_metadata_supplementary_upload(metadata_id, key).await?;
        let file = File::open(path_str).await?;
        upload_multipart_supplementary_file(metadata_id, "audio/mp3", &upload_url, file).await?;
        Ok(())
    }
}
