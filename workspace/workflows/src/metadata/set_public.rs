use crate::activity::{Activity, ActivityContext, Error};
use async_trait::async_trait;
use bosca_client::client::{Client, WorkflowJob};

pub struct MetadataSetPublicActivity {
    id: String,
}

impl Default for MetadataSetPublicActivity {
    fn default() -> Self {
        Self::new()
    }
}

impl MetadataSetPublicActivity {
    pub fn new() -> MetadataSetPublicActivity {
        MetadataSetPublicActivity {
            id: "metadata.set.public".to_string(),
        }
    }
}

#[async_trait]
impl Activity for MetadataSetPublicActivity {
    fn id(&self) -> &String {
        &self.id
    }

    async fn execute(
        &self,
        client: &Client,
        _: &mut ActivityContext,
        job: &WorkflowJob,
    ) -> Result<(), Error> {
        let id = if let Some(r) = job.workflow_activity.configuration.get("metadata_id") {
            r.as_str().unwrap_or("").to_owned()
        } else {
            job.metadata.as_ref().unwrap().id.to_owned()
        };
        client.set_metadata_public(&id, true).await?;
        Ok(())
    }
}
