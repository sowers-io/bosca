use std::error::Error;
use reqwest::Client;
use crate::events::Events;
use crate::events_sink::EventSink;

pub struct HttpSink {
    client: Client,
    endpoint: String,
}

impl HttpSink {
    pub fn new(endpoint: String) -> Self {
        let client = Client::builder()
            .build()
            .unwrap();
        Self { client, endpoint }
    }
}

#[async_trait::async_trait]
impl EventSink for HttpSink {
    async fn add(&mut self, events: Events) -> Result<(), Box<dyn Error>> {
        let response = self.client
            .post(&self.endpoint)
            .json(&events)
            .send()
            .await?;
        if !response.status().is_success() {
            let txt = response.text().await?;
            return Err(txt.into())
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