use crate::activity::{Activity, ActivityContext, Error};
use async_trait::async_trait;
use bosca_client::client::get_collection_items::GetCollectionItemsContentCollectionItems;
use bosca_client::client::{Client, WorkflowJob};

pub struct CollectionSetReadyActivity {
    id: String,
}

impl Default for CollectionSetReadyActivity {
    fn default() -> Self {
        Self::new()
    }
}

impl CollectionSetReadyActivity {
    pub fn new() -> CollectionSetReadyActivity {
        CollectionSetReadyActivity {
            id: "collection.set.ready".to_string(),
        }
    }

    async fn set_ready(&self, client: &Client, id: &str) -> Result<(), Error> {
        let mut offset = 0;
        loop {
            let items = client.get_collection_items(id, offset, 100).await?;
            if items.len() == 0 {
                break;
            }
            for item in items {
                match item {
                    GetCollectionItemsContentCollectionItems::Metadata(m) => {
                        if m.ready.is_none() {
                            client.set_metadata_ready(&m.id).await?;
                        }
                    }
                    GetCollectionItemsContentCollectionItems::Collection(c) => {
                        if c.ready.is_none() {
                            Box::pin(self.set_ready(client, &c.id)).await?;
                            client.set_collection_ready(&c.id).await?;
                        }
                    }
                }
            }
            offset += 100;
        }
        Ok(())
    }
}

#[async_trait]
impl Activity for CollectionSetReadyActivity {
    fn id(&self) -> &String {
        &self.id
    }

    async fn execute(
        &self,
        client: &Client,
        _: &mut ActivityContext,
        job: &WorkflowJob,
    ) -> Result<(), Error> {
        let mut recursive = false;
        if let Some(r) = job.workflow_activity.configuration.get("recursive") {
            recursive = r.as_bool().unwrap_or(false);
        }
        let id = if let Some(r) = job.workflow_activity.configuration.get("collection_id") {
            r.as_str().unwrap_or("").to_owned()
        } else {
            job.collection.as_ref().unwrap().id.to_owned()
        };
        if recursive {
            self.set_ready(client, &id).await?;
        }
        client.set_collection_ready(&id).await?;
        Ok(())
    }
}
