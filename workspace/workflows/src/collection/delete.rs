use crate::activity::{Activity, ActivityContext, Error};
use async_trait::async_trait;
use bosca_client::client::enqueue_child_workflow::WorkflowConfigurationInput;
use bosca_client::client::get_collection_items::GetCollectionItemsContentCollectionItems;
use bosca_client::client::{Client, WorkflowJob};
use serde_json::Value;

pub struct CollectionDeleteActivity {
    id: String,
}

impl Default for CollectionDeleteActivity {
    fn default() -> Self {
        Self::new()
    }
}

impl CollectionDeleteActivity {
    pub fn new() -> CollectionDeleteActivity {
        CollectionDeleteActivity {
            id: "collection.delete".to_string(),
        }
    }

    async fn delete(&self, client: &Client, id: &str) -> Result<(), Error> {
        let mut offset = 0;
        loop {
            let items = client.get_collection_items(&id, offset, 100).await?;
            if items.len() == 0 {
                break;
            }
            for item in items {
                match item {
                    GetCollectionItemsContentCollectionItems::Metadata(m) => {
                        client.delete_metadata(&m.id).await?;
                    }
                    GetCollectionItemsContentCollectionItems::Collection(c) => {
                        Box::pin(self.delete(client, &c.id)).await?;
                        client.delete_collection(&c.id, false).await?
                    }
                }
            }
            offset += 100;
        }
        Ok(())
    }
}

#[async_trait]
impl Activity for CollectionDeleteActivity {
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
            self.delete(client, &id).await?;
        }

        client.delete_collection(&id, false).await?;

        Ok(())
    }
}
