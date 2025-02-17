use std::error::Error;
use reqwest::Client;
use crate::events::Events;
use crate::events_sink::{EventPipelineContext, EventSink};
use crate::events_transform::EventTransform;

pub struct HttpSink {
    transforms: Vec<Box<dyn EventTransform + Send + Sync + 'static>>,
    client: Client,
    endpoint: String,
}

impl HttpSink {
    pub fn new(transforms: Vec<Box<dyn EventTransform + Send + Sync + 'static>>, endpoint: String) -> Self {
        let client = Client::builder()
            .build()
            .unwrap();
        Self { transforms, client, endpoint }
    }
}

#[async_trait::async_trait]
impl EventSink for HttpSink {
    async fn add(&mut self, context: &mut EventPipelineContext, events: &Events) -> Result<(), Box<dyn Error>> {
        if self.transforms.is_empty() {
            let response = self.client
                .post(&self.endpoint)
                .json(events)
                .send()
                .await?;
            if !response.status().is_success() {
                let txt = response.text().await?;
                return Err(txt.into())
            }
        } else {
            let mut events = events.clone();
            for transform in self.transforms.iter() {
                transform.transform(context, &mut events).await?;
            }
            let response = self.client
                .post(&self.endpoint)
                .json(&events)
                .send()
                .await?;
            if !response.status().is_success() {
                let txt = response.text().await?;
                return Err(txt.into())
            }
        }
        Ok(())
    }

    async fn flush(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn finish(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}